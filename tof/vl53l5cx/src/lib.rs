#![no_std]
#![allow(non_snake_case)]
extern crate alloc;

#[cfg(feature = "single")]
mod ranging;
#[cfg(feature = "flock")]
mod ranging_flock;
mod uld_platform;
mod vl;
#[cfg(feature = "flock")]
mod z_array_try_map;

#[cfg(feature = "single")]
pub use ranging::Ranging;

#[cfg(feature = "flock")]
pub use ranging_flock::RangingFlock;

pub use vl::VL;

use vl53l5cx_uld::{
    DEFAULT_I2C_ADDR_8BIT
};

// Elements we pass through from the ULD level. Careful here: ideally all API is under our direct control!
pub use vl53l5cx_uld::{
    API_REVISION as ULD_VERSION,
    Mode,
    RangingConfig,
    TargetOrder,
    units
};

pub const DEFAULT_I2C_ADDR: I2cAddr = I2cAddr(DEFAULT_I2C_ADDR_8BIT);

pub type Instant = esp_hal::time::Instant;

/*
* Wrapper to eliminate 8-bit vs. 7-bit I2C address misunderstandings.
*/
//#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct I2cAddr(u8);     // stored as 7-bit (doesn't matter)

impl I2cAddr {
    fn from_8bit(v: u8) -> Self {
        assert!(v % 2 == 0, "8-bit I2C address is expected to be even");    // tbd. de-IDE-underscore
        Self(v >> 1)
    }
    fn as_7bit(&self) -> u8 { self.0 }
}
