#![no_std]
#![allow(non_snake_case)]

mod platform;

mod uld_raw;

use core::ffi::CStr;
use core::mem;
use core::ptr::addr_of_mut;
use uld_raw::{
    VL53L5CX_Configuration,
    vl53l5cx_init,
    API_REVISION as API_REVISION_r,   // &[u8] with terminating '\0'
    vl53l5cx_get_power_mode,
    vl53l5cx_set_power_mode,
    //vl53l5cx_start_ranging,
    PowerMode,
    VL53L5CX_Platform
};

#[cfg(feature="defmt")]
use defmt::{debug, warn};
use mem::MaybeUninit;

pub type Result<T> = core::result::Result<T,u8>;

// from -> https://stackoverflow.com/a/70222282/14455
macro_rules! field_size {
    ($t:ident :: $field:ident) => {{
        let m = core::mem::MaybeUninit::<$t>::uninit();
        // According to https://doc.rust-lang.org/stable/std/ptr/macro.addr_of_mut.html#examples,
        // you can dereference an uninitialized MaybeUninit pointer in addr_of!
        // Raw pointer deref in const contexts is stabilized in 1.58:
        // https://github.com/rust-lang/rust/pull/89551
        let p = unsafe {
            core::ptr::addr_of!((*(&m as *const _ as *const $t)).$field)
        };

        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        size_of_raw(p)
    }};
}

/**
* @brief App provides, to talk to the I2C and do blocking delays.
*/
pub trait Platform {
    // provided by the app
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> Result<()>;
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> Result<()>;
    fn delay_ms(&mut self, ms: u32);
}

impl VL53L5CX_Configuration {
    /** @brief Returns a default 'VL53L5CX_Configuration' struct, spiced with the application
    * provided 'Platform'-derived state (opaque to us, except for its size).
    *
    * Initialized state is (as per ULD C code):
    *   <<
    *       .platform: dyn Platform     = anything the app keeps there
    *       .streamcount: u8            = 0 (undefined by ULD C code)
    *       .data_read_size: u32        = 0 (undefined by ULD C code)
    *       .default_configuration: *mut u8 = VL53L5CX_DEFAULT_CONFIGURATION (a const table)
    *       .default_xtalk: *mut u8     = VL53L5CX_DEFAULT_XTALK (a const table)
    *       .offset_data: [u8; 488]     = data read from the sensor
    *       .xtalk_data: [u8; 776]      = copy of 'VL53L5CX_DEFAULT_XTALK'
    *       .temp_buffer: [u8; 1452]    = { being used for multiple things }
    *       .is_auto_stop_enabled: u8   = 0
    *   <<
    *
    * Side effects:
    *   - the sensor is reset, and firmware uploaded to it
    *   - NVM (non-volatile?) data is read from the sensor to the driver
    *   - default Xtalk data programmed to the sensor
    *   - default configuration ('.default_cconfiguration') written to the sensor
    *   - four bytes written to sensor's DCI memory at '0xDB80U' ('VL53L5CX_DCI_PIPE_CONTROL'):
    *       {VL53L5CX_NB_TARGET_PER_ZONE, 0x00, 0x01, 0x00}
    *   - if 'NB_TARGET_PER_ZONE' != 1, 1 byte updated at '0x5478+0xc0' ('VL53L5CX_DCI_FW_NB_TARGET'+0xc0)  // if I got that right!?!
    *       {VL53L5CX_NB_TARGET_PER_ZONE}
    *   - one byte written to sensor's DCI memory at '0xD964' ('VL53L5CX_DCI_SINGLE_RANGE'):
    *       {0x01}
    *   - two bytes updated at sensor's DCI memory at '0x0e108' ('VL53L5CX_GLARE_FILTER'):
    *       {0x01, 0x01}
    */
    fn init_with<P : Platform>(/*gulp*/ p: P) -> Result<Self> {

        let st: u8 = unsafe {
            let mut uninit = MaybeUninit::<VL53L5CX_Configuration>::uninit();
                // note: use '::zeroed()' in place of '::uninit()' to get more predictability

            // Move 'p' to 'uninit.platfor' (the beginning of the structure); ULD C 'vl.._init()' will need it.
            //
            let up = uninit.as_mut_ptr();

            let reserved = field_size!(VL53L5CX_Configuration::platform);
            assert!(size_of::<P>() <= reserved, "Reserved size in ULD C side is not enough");   // edit 'platform.h' if need more

            addr_of_mut!((*up).platform as *mut P).write(p);

            // Initialize those fields we know C API won't touch (just in case)
            addr_of_mut!((*up).streamcount).write(u8::MAX);
            addr_of_mut!((*up).data_read_size).write(u32::MAX);

            // Call ULD C API to arrange the rest
            vl53l5cx_init(up)
        };

        match st {
            0 => Ok(Self),
            st => Err(st)
        }
    }
}

 pub struct VL53L5CX<'a> {
    // The vendor ULD driver wants to have a "playing ground" (they call it 'Dev', presumably for
    // "device"), in the form of the "configuration" struct. It's not really configuration;
    // more of a driver "heap" where all the state, and also temporary memory buffers exist.
    // The good part of this arrangement is, we have separate state for >1 drivers. :)
    //
    // The "state" also carries our 'Platform' struct within it, at the very head. The ULD
    // code uses it to reach back to the app level, for MCU hardware access - I2C and delays.
    // The "state" can be read, but we "MUST not manually change these field[s]". In this API,
    // the whole "state" is kept private, to enforce the read-only nature.
    //
    cfg: Pin<&'a mut VL53L5CX_Configuration>,

    pub API_REVISION: &'a str,
}

impl VL53L5CX<'_> {
    /** @brief Open a connection to a specific sensor; uses I2C (and delays) via the 'Platform'
    ** provided by the caller.
    **/
    pub fn new(/*move*/ mut pl: impl Platform) -> Result<Self> {
        let API_REVISION: &str = CStr::from_bytes_with_nul(API_REVISION_r).unwrap()
            .to_str().unwrap();

        // Check if there is a sensor out there.
        match vl53l5cx_ping(&mut pl)? {
            (0xf0, 0x02) => {},     // approved! (vendor driver only proceeds with this)
            t => {
                #[cfg(feature="defmt")]
                warn!("Unexpected device id, rev id: {}", t);
            }
        };

        #[cfg(feature="defmt")]
        debug!("aaa");

        let tmp2 = VL53L5CX_Configuration::prep(/*move*/ pl);
        let tmp = pin!(tmp2);

        let mut this = Self {
            cfg: tmp,    //: pin!(VL53L5CX_Configuration::prep(/*move*/ pl)),
            API_REVISION
        };

        _ = tmp2;

        #[cfg(feature="defmt")]
        debug!("bbb");

        // Need a C pointer to 'this.cfg' that can be passed (and left) to ULD C driver.
        //
        let p_cfg: *mut VL53L5CX_Configuration = unsafe { this.cfg.as_mut().get_unchecked_mut() };

        match unsafe { vl53l5cx_init(p_cfg) } {
            0 => Ok(this),
            st => Err(st)
        }
    }

    //---
    // Ranging (getting values)
    //
    pub fn start_ranging(&mut self) -> Result<() /*tbd. RangingHandle*/> {
        unimplemented!()
    }

    //---
    // Maintenance; special use
    //
    pub fn get_power_mode(&mut self) -> Result<PowerMode> {
        let mut tmp: u8 = 0;
        match unsafe { vl53l5cx_get_power_mode(self.cfg.get_unchecked_mut(), &mut tmp) } {
            ST_OK => Ok(PowerMode::from_repr(tmp).unwrap()),
            e => Err(e)
        }
    }
    pub fn set_power_mode(&mut self, v: PowerMode) -> Result<()> {
        match unsafe { vl53l5cx_set_power_mode(self.cfg.get_unchecked_mut(), v as u8) } {
            ST_OK => Ok(()),
            e => Err(e)
        }
    }

    pub fn set_i2c_address(&mut self, addr: u8) -> Result<()> {
        unimplemented!()
    }

    pub fn dci_read_data(index: u16, buf: &mut [u8]) { unimplemented!() }
    pub fn dci_write_data(index: u16, buf: &[u8]) { unimplemented!() }

    // 'dci_replace_data' doesn't seem useful for applications, and can be easily
    // reproduced using the 'read' and 'write'. Not exposing it.

    // Remaining to be implemented:
    //  vl53l5cx_enable_internal_cp()
    //  vl53l5cx_disable_internal_cp
    //  vl53l5cx_set_VHV_repeat_count
    //  vl53l5cx_get_VHV_repeat_count
}

/**
* Function, modeled akin to the vendor ULD 'vl53l5cx_is_alive()', but:
*   - made in Rust
*   - returns the device and revision id's
*/
fn vl53l5cx_ping<P : Platform>(pl: &mut P) -> Result<(u8,u8)> {
    let mut buf: [u8;2] = [0,0];        // tbd. any more elegant way? #help

    pl.wr_bytes(0x7fff, &[0x00])?;
    pl.rd_bytes(0, &mut buf)?;   // [dev_id, rev_id]
    pl.wr_bytes(0x7fff, &[0x02])?;

    Ok( (buf[0], buf[1]) )
}

