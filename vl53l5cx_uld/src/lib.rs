#![no_std]

mod platform;

mod uld;
mod uld_raw;

use core::ffi::CStr;
use uld_raw::{
    VL53L5CX_Configuration,
    vl53l5cx_init,
    VL53L5CX_API_REVISION,  // &[u8] with terminating '\0'
        // tbd. is there a way to make 'bindgen' turn that into a 'CStr', at generation?
};

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

pub struct VL53L5CX {
    // The vendor ULD driver wants to have a "playing ground", provided by us. This is readable,
    // but we "MUST not manually change these field[s]".
    //
    vc: VL53L5CX_Configuration,
        // nb. vendor samples call this "Dev"

    API_REVISION: &str,     // e.g. "VL53L5CX_2.0.0"
}

impl VL53L5CX {
    /** @brief Open a connection to a new sensor; uses I2C (and delays) via the 'Config' provided
    ** by the caller.
    **/
    pub fn new(pl: /*&mut*/ impl Platform) -> Result<Self> {

        let API_REVISION: &str = CStr::from_bytes_with_nul(VL53L5CX_API_REVISION).unwrap()
            .to_str().unwrap();

        let o = Self {
            x: VL53L5CX_Configuration {
                platform: pl,
            },
            API_REVISION
        };

        // Check if there is a sensor out there.
        match vl53l5cx_ping(&mut pl)? {
            IDS{dev_id: 0, some_id: 1} => {},
            _ => unimplemented!()   // unexpected id's; how to proceed??
        };

        match unsafe { vl53l5cx_init(&mut o.x) } {
            0 => Ok(Self),
            st => Err(st)
        }
    }

    /*
    * Function, modeled akin to the vendor ULD API 'vl53l5cx_is_alive', but:
    *   - made in Rust
    *   - returns the device and ... id's
    */
    fn vl53l5cx_ping(&mut pl) -> Result<IDS> {
        unimplemented!()
    }
}

struct IDS{dev_id: u8, rev_id: u8}     // expected: IDS(0xf0, 0x02)

const EXPECTED_DEV_ID: u8 = 0xf0;
const EXPECTED_REV_ID: u8 = 0x02;

fn vl53l5cx_ping(pl: &mut Platform) -> Result<IDS> {
    let mut buf [u8;2];

    pl.wr_bytes(0x7fff, [0x00])?;
    pl.rd_bytes(0, buf)?;   // [dev_id, rev_id]
    pl.wr_bytes(0x7fff, [0x02])?;

    let ret= IDS{dev_id = buf.0, rev_id = buf.1 };     // leave judging the values to the caller
    Ok(ret)
}
