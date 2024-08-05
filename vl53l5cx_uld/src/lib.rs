#![no_std]
#![allow(non_snake_case)]

mod platform;

mod uld_raw;

use core::ffi::{c_void, CStr};
use core::{mem, pin};
use uld_raw::{
    VL53L5CX_Configuration,
    vl53l5cx_init,
    API_REVISION as API_REVISION_r,   // &[u8] with terminating '\0'
};

use defmt::{debug, warn};
use mem::MaybeUninit;
use pin::{Pin, pin};
use crate::uld_raw::PowerMode;

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

pub struct VL53L5CX<'a, P: Platform> {
    pl: Pin<&'a mut P>,   // lifespan equals that of 'Self'

    // The vendor ULD driver wants to have a "playing ground" (they call it 'Dev', presumably for
    // "device"), in the form of the "configuration" struct. It's not really (only) configuration.
    // It's a handle/bundle that also carries the 'VL53L5CX_Platform' within it, used for the ULD
    // code to reach back to the app level, for MCU hardware access - I2C and delays). Somewhat
    // peculiar API design. 'cfg' can be read, but we "MUST not manually change these field[s]".
    //
    // In this API, the whole 'cfg' is kept private, to enforce the read-only nature.
    //
    cfg: Pin<&'a mut VL53L5CX_Configuration>,

    pub API_REVISION: &'a str,
}

impl<P: Platform> VL53L5CX<'_, P> {
    /** @brief Open a connection to a specific sensor; uses I2C (and delays) via the 'Platform'
    ** provided by the caller.
    **/
    pub fn new(/*move*/ mut pl: P) -> Result<Self> {
        let API_REVISION: &str = CStr::from_bytes_with_nul(API_REVISION_r).unwrap()
            .to_str().unwrap();

        // Check if there is a sensor out there.
        match vl53l5cx_ping(&mut pl)? {
            (0xf0, 0x02) => {},     // approved! (vendor driver only proceeds with this)
            t => {
                warn!("Unexpected device id, rev id: {}", t);
            }
        };

        let pl = pin!(pl);
        let pl4c = unsafe { pl.get_mut() } as *mut c_void;

        debug!("aaa");

        let mut this = Self {
            pl,
            cfg: pin!(VL53L5CX_Configuration {
                platform: pl4c,
                ..VL53L5CX_Configuration::default()
            }),
            API_REVISION
        };

        debug!("bbb");

        match unsafe { vl53l5cx_init( unsafe { this.cfg.get_mut() }) } {
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
        unimplemented!()
    }
    pub fn set_power_mode(&mut self, v: PowerMode) -> Result<()> {
        unimplemented!()
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
