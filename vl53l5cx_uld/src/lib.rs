#![no_std]
#![allow(non_snake_case)]

mod macros;
pub mod ranging;    // TEMP: for now, exposing also as 'ranging::{Resolution, ...}'; let's see
mod platform;
mod uld_raw;
pub mod units;

#[allow(unused_imports)]
#[cfg(feature = "defmt")]
use defmt::{debug, warn, trace, error};

use core::{
    ffi::CStr,
    mem::MaybeUninit,
    ptr::addr_of_mut,
    result::Result as CoreResult,
};

use macros::field_size;
pub use ranging::{
    RangingConfig,
    Ranging,
    *   // Resolution, TargetOrder, ...
};
pub use units::*;   // Hz, Ms

#[allow(unused_imports)]    // remove warnings on 'as _'
use uld_raw::{
    VL53L5CX_Configuration,
    vl53l5cx_init,
    API_REVISION as API_REVISION_r,   // &[u8] with terminating '\0'
    vl53l5cx_get_power_mode,
    vl53l5cx_set_power_mode,
    vl53l5cx_set_i2c_address,
    PowerMode,
    ST_OK,

    /*** tbd. if needed, bring under features
    *vl53l5cx_disable_internal_cp,
    *vl53l5cx_enable_internal_cp,
    *    //
    *vl53l5cx_dci_read_data,
    *vl53l5cx_dci_write_data,
    *    //
    *vl53l5cx_get_VHV_repeat_count,
    *vl53l5cx_set_VHV_repeat_count,
    */
};

use crate::uld_raw::ST_ERROR;

pub type Result<T> = core::result::Result<T,u8>;

/**
* @brief App provides, to talk to the I2C and do blocking delays.
*/
pub trait Platform {
    // provided by the app
    //
    // Note: We're using a slightly unconventional 'Result<(),()>' (no type for the errors).
    //      This is because of adaptation difficulties between the application-level I2C stack
    //      error values and the vendor ULD C API (that deals with 'u8's, but basically is a
    //      binary between ST_OK/ST_ERR). We didn't want to expose the library to I2C, nor the
    //      application to the ST_OK/ST_ERR.
    //
    //      This essentially eradicates the error type. We could (and tried):
    //          - using 'impl Any'  | not supported under 'stable'
    //          - using 'E: Error'  | works, but makes prototypes look complex (not good for training..)
    //          - bool              | just feels... wrong in Rust
    //
    //      A boolean would do, but using 'Result' is kinda customary.
    //
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> CoreResult<(),()>;
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> CoreResult<(),()>;
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
    fn init_with<P : Platform + 'static>(/*move*/ mut p: P) -> Result<Self> {

        #[allow(unused_unsafe)]
        let ret: Result<VL53L5CX_Configuration> = unsafe {
            let mut uninit = MaybeUninit::<VL53L5CX_Configuration>::uninit();
                // note: use '::zeroed()' in place of '::uninit()' to get more predictability

            let up = uninit.as_mut_ptr();

            // Get a '&mut dyn Platform' reference to 'p'. Converting '*c_void' to a templated 'P'
            // type is beyond the author's imagination (perhaps impossible?) whereas '&mut dyn Platform'
            // *may be* just within doable!
            //
            // Nice part of using '&mut dyn Platform' is also that the size and alignment requirements
            // (16 and 8 bytes), remain constant for the C side.
            //
            let dd: &mut dyn Platform = &mut p;

            // Make a bitwise copy of 'dd' in 'uninit.platform'; ULD C 'vl.._init()' will need it,
            // and pass back to us (Rust) once platform calls (I2C/delays) are needed.
            {
                // First, let's check size and alignment
                let (sz1,al1, sz2,al2) = (
                    field_size!(VL53L5CX_Configuration::platform),
                    align_of::<VL53L5CX_Configuration>(),     // it's the first field; thus same (C) alignment
                    size_of_val(dd), align_of_val(dd)   // 16,8
                );

                assert_eq!(sz1,sz2, "Tunnel entry and exit sizes don't match");   // edit 'platform.h' to adjust
                assert_eq!(al1,al2, "Tunnel alignments don't match");

                let pp = addr_of_mut!((*up).platform);
                assert!( (pp as usize)%al2 == 0, "bad alignment on C side" );   // 2nd check (TEMP?)

                *(pp as *mut &mut dyn Platform) = dd;
            }

            // Initialize those fields we know C API won't touch (just in case)
            addr_of_mut!((*up).streamcount).write(u8::MAX);
            addr_of_mut!((*up).data_read_size).write(u32::MAX);

            // Call ULD C API to arrange the rest
            //
            // Note: Already this will call the platform methods (via the tunnel).
            //
            match vl53l5cx_init(up) {
                0 => Ok(uninit.assume_init()),  // we guarantee it's now initialized
                e => Err(e)
            }
        };
        ret
    }
}

pub struct VL53L5CX<'a> {
    // The vendor ULD driver wants to have a "playing ground" (they call it 'Dev', presumably for
    // "device"), in the form of the "configuration" struct. It's not really configuration;
    // more of a driver working memory area where all the state and buffers exist.
    //
    // The good part of this arrangement is, we have separate state when handling multiple sensors. :)
    //
    // The "state" also carries our 'Platform' struct within it (at the very head). The ULD
    // code uses it to reach back to the app level, for MCU hardware access: I2C and delays.
    //
    // The "state" can be read, but we "MUST not manually change these field[s]". In this Rust API,
    // the whole "state" is kept private, to enforce such read-only nature.
    //
    vl: VL53L5CX_Configuration,

    pub API_REVISION: &'a str,
}

impl VL53L5CX<'_> {
    /** @brief Open a connection to a specific sensor; uses I2C and delays via the 'Platform'
    ** provided by the caller.
    **/
    // Note: Didn't want to call this 'new()' since it does initialization, and returns 'Result'.
    //      Making it in two phases ('::new().init()') with the intermediate struct not being
    //      'VL53L5CX' also feels wrong. What would be the idiomatic way , in Rust? #advice
    //
    pub fn new_and_init<P : Platform + 'static>(/*move*/ mut p: P) -> Result<Self> {
        let API_REVISION: &str = CStr::from_bytes_with_nul(API_REVISION_r).unwrap()
            .to_str().unwrap();

        Self::ping(&mut p).map_err(|_| ST_ERROR)?;
        let vl = VL53L5CX_Configuration::init_with(/*move*/ p)?;

        Ok(Self {
            vl,
            API_REVISION
        })
    }

    fn ping<P : Platform>(p: &mut P) -> CoreResult<(),()> {
        match vl53l5cx_ping(p)? {
            (a@ 0xf0, b@ 0x02) =>
                debug!("Ping succeeded: {=u8:#04x},{=u8:#04x}", a,b),   // vendor driver ONLY proceeds with this
            t => {
                #[cfg(feature="defmt")]
                warn!("Unexpected '(device id, rev id)'; not '(0xf0,0x02)': {}", t);
            }
        }
        Ok(())
    }

    //---
    // Ranging (getting values)
    //
    pub fn start_ranging(&mut self, cfg: &RangingConfig) -> Result<Ranging> {    // Rust note: 'impl Into<...>' would allow either '&T' or 'T' by the caller.
        let r: Ranging = Ranging::new_maybe(&mut self.vl, cfg)?;
        Ok(r)
    }

    //---
    // Maintenance; special use
    //
    pub fn get_power_mode(&mut self) -> Result<PowerMode> {
        let mut tmp: u8 = 0;
        match unsafe { vl53l5cx_get_power_mode(&mut self.vl, &mut tmp) } {
            ST_OK => Ok(PowerMode::from_repr(tmp).unwrap()),
            e => Err(e)
        }
    }
    pub fn set_power_mode(&mut self, v: PowerMode) -> Result<()> {
        match unsafe { vl53l5cx_set_power_mode(&mut self.vl, v as u8) } {
            ST_OK => Ok(()),
            e => Err(e)
        }
    }

    pub fn set_i2c_address(&mut self, addr: u8) -> Result<()> {
        match unsafe { vl53l5cx_set_i2c_address(&mut self.vl, addr as u16) } {
            ST_OK => Ok(()),
            e => Err(e)
        }
    }

    // tbd. if exposing these, make them into a "dci" feature
    //pub fn dci_read_data(index: u16, buf: &mut [u8]) { unimplemented!() }
    //pub fn dci_write_data(index: u16, buf: &[u8]) { unimplemented!() }

    // 'dci_replace_data' doesn't seem useful; easily reproduced using the 'read' and 'write'. Skip.

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
*
* This is the only code that the ULD C driver calls on the device, prior to '.init()', i.e. it
* is supposed to be functioning also before the firmware and parameters initialization.
*
* Vendor note:
*   - ULD C driver code expects '(0xf0, 0x02)'. The author has only seen '(0x03, 0x00)'.
*/
fn vl53l5cx_ping<P : Platform>(pl: &mut P) -> CoreResult<(u8,u8),()> {
    let mut buf: [u8;2] = [u8::MAX;2];

    pl.wr_bytes(0x7fff, &[0x00])?;
    pl.rd_bytes(0, &mut buf)?;   // [dev_id, rev_id]
    pl.wr_bytes(0x7fff, &[0x02])?;

    Ok( (buf[0], buf[1]) )
}

// keep for possible use
#[cfg(disabled)]
fn mem_slice(p: *const c_void, _n: usize) -> [u32;2] {
    let pp: *const u32 = p as _;
    unsafe {
        [*pp, *pp.wrapping_add(1)]
    }
}