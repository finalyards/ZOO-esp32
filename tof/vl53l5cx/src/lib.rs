#![no_std]
#![allow(non_snake_case)]

mod vl;
mod uld_platform;
mod ranging;

pub use vl::VL;
pub use ranging::RingN;

/*
* Wrapper to eliminate 8-bit vs. 7-bit I2C address misunderstandings.
*/
//#[derive(Copy, Clone)]
pub struct I2cAddr(u8);     // stored as 7-bit (doesn't matter)

impl I2cAddr {
    fn from_8bit(v: u8) -> Self {
        assert!(v % 2 == 0, "8-bit I2C address is expected to be even");    // tbd. de-IDE-underscore
        Self(v >> 1)
    }
    fn as_7bit(&self) -> u8 { self.0 }
}
