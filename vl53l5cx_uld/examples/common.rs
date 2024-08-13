
#[allow(unused_imports)]
use defmt::{info, debug};

use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

extern crate vl53l5cx_uld as uld;
use uld::Platform;

extern "C"
pub struct MyPlatform {
    // would have something on I2C, in a real app

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
