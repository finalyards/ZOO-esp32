
#[allow(unused_imports)]
use defmt::{info, debug};

use core::result::Result as CoreResult;

use embedded_hal::i2c::{
    I2c,
    Error as I2cError,
    Operation as I2cOperation
};

use esp_hal::{
    clock::Clocks,
    delay::Delay,
    gpio::Io,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*
};

extern crate vl53l5cx_uld as uld;
use uld::Platform;

// Note: Vendor docs use 7-bit I2C addresses shifted one left
//
const I2C_ADDR: u8 = 0x52 >> 1;    // vendor default

/*
* Pin routing expected (VL53L5CX-SATEL -> ESP32-C{3_}):
*   - SDA => GPIO4      // same as in 'esp-hal' I2C example
*   - SCL => GPIO5      //  -''-
*   - ...
*/
pub struct MyPlatform<'a> {
    i2c: I2C<'a>,
    d_provider: Delay,      // kept alive until the end of 'self' lifespan
}


impl MyPlatform {
    /*
    * Note: The 'i2c_addr' is take as 7-bit - as 'esp-hal' handles them.
     */
    pub fn new(peripherals: Peripherals, clocks: &Clocks, i2c_addr: u8) -> Self {
        let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

        let mut i2c = I2C::new(
            peripherals.I2C0,
            io.pins.gpio4,  // SDA
            io.pins.gpio5,  // SCL
            100.kHz(),
            clocks,
            None        // option: interrupt handler
        );

        Self{
            i2c,
            d_provider: Delay::new(&clocks),
        }
    }
}

impl Platform for MyPlatform {
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> CoreResult<(),I2cError> {
        let addr = addr.to_be_bytes();

        self.i2c.write_read(I2C_ADDR, &addr, buf)
    }
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> CoreResult<(),I2cError> {
        let addr = addr.to_be_bytes();

        // Note: There didn't seem to be (Rust stable 1.80) a way to concat two slices together,
        //      in 'no-alloc'. However, the I2C library *might* support the below (adapted from
        //      the way '.write_read()' is implemented).
        // tbd. tell whether this works.

        self.i2c.transaction(I2C_ADDR,
         &mut [I2cOperation::Write(&addr), I2cOperation::Write(vs)]
        )
    }

    fn delay_ms(&mut self, ms: u32) {
        self.d_provider.delay_millis(ms);
    }
}
