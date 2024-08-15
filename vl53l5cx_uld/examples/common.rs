
#[allow(unused_imports)]
use defmt::{info, debug};

use esp_hal::{
    clock::Clocks,
    delay::Delay,
};

extern crate vl53l5cx_uld as uld;
use uld::Platform;

pub struct MyPlatform {
    // tbd. I2C access state/handle (so we can have multiple)

    d_provider: Delay,      // kept alive until the end of 'self' lifespan
}


impl MyPlatform {
    pub fn new(clocks: &Clocks, _i2c_addr: u8) -> Self {
        Self{
            d_provider: Delay::new(&clocks),
        }
    }
}

impl Platform for MyPlatform {
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> uld::Result<()> {
        unimplemented!()
    }
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> uld::Result<()> {
        unimplemented!()
    }

    fn delay_ms(&mut self, ms: u32) {
        self.d_provider.delay_millis(ms);
    }
}
