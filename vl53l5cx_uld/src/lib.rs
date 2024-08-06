#![no_std]
#![allow(non_snake_case)]

mod platform;

mod uld_raw;

use core::ffi::{/*c_void,*/ CStr};
use core::{mem, pin};
use core::ptr::addr_of_mut;
use uld_raw::{
    VL53L5CX_Configuration,
    vl53l5cx_init,
    API_REVISION as API_REVISION_r,   // &[u8] with terminating '\0'
    vl53l5cx_get_power_mode,
    vl53l5cx_set_power_mode,
    vl53l5cx_start_ranging,
    PowerMode,
    VL53L5CX_Platform
};

use defmt::{debug, warn};
use mem::MaybeUninit;
use pin::{Pin, pin};

pub type Result<T> = core::result::Result<T,u8>;

/**
* @brief App provides, to talk to the I2C and do blocking delays.
*/
pub trait Platform {
    // provided by the app
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> Result<()>;
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> Result<()>;
    fn delay_ms(&mut self, ms: u32);
}

impl Default for VL53L5CX_Configuration {
    fn default() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }    // undefined behavior (but ok for us)
    }
}

impl VL53L5CX_Configuration {
    fn prep<P: Platform>(p: P) -> Self {
        let need = size_of::<P>();
        let have_prefix = size_of::<VL53L5CX_Platform>();
        assert!(need <= have_prefix);

        let mut o = VL53L5CX_Configuration::default();

        // *move* 'p' to the reserved space, in the beginning of the '..._Configuration' struct
        unsafe {
            //let rp: &mut P = &o as &mut P;    // didn't cut it
            let rp: *mut P = addr_of_mut!(o) as _;
            *rp = p;
        }
        o
    }
}

// DO NOT move it.
//
//impl !Unpin for VL53L5CX_Configuration {}
    //
    // gives (stable, 1.80.0): error[E0658]: negative trait bounds are not yet fully implemented; use marker types for now

// Note: Cannot add 'PhantomPinned' field to an existing (C) struct 'VL53L5CX_Configuration'.
//
//  tbd. How to ensure it doesn't get moved, despite our 'pin!()'. 🤞

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
                warn!("Unexpected device id, rev id: {}", t);
            }
        };

        debug!("aaa");

        let tmp2 = VL53L5CX_Configuration::prep(/*move*/ pl);
        let tmp = pin!(tmp2);

        let mut this = Self {
            cfg: tmp,    //: pin!(VL53L5CX_Configuration::prep(/*move*/ pl)),
            API_REVISION
        };

        _ = tmp2;

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

