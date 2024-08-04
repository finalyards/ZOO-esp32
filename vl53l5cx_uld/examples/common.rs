
extern crate vl53l5cx_uld as uld;

use uld::Platform;

pub struct MyPlatform {
    // would have something on I2C, in a real app
}

impl MyPlatform {
    pub fn new() -> Self {
        // use the default 'xxx' I2C address
        let _i2c_addr = 0x52;

        Self{
            // tbd.
        }
    }
}

impl Platform for MyPlatform {
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> uld::Result<()> { unimplemented!() }
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> uld::Result<()> { unimplemented!() }
    fn delay_ms(&mut self, ms: u32) { unimplemented!() }
}


//---
/*** #pending?
// Note: [vendor] writes of 7 and 10-bit I2C addresses, but in practice, all addresses are 7 bits.
#[allow(non_camel_case_types)]
//#[derive(Copy, Clone)]      // tbd. is this needed?
struct I2C_Addr(u8);

/***R impl I2C_Addr {
    fn from(v: u16) -> Self {
        assert!(v <= 0xff, "Unexpected I2C address: {}", v);
        Self(v as u8)
    }
}***/
***/

