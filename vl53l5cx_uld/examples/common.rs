
#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace};

#[allow(unused_imports)]
use core::{
    cell::RefCell,
    ffi::c_void,
    ptr::NonNull
};
use defmt::Format;
use embedded_hal::i2c::{
    Operation as I2cOperation
};
use embedded_hal::i2c::SevenBitAddress;
use esp_hal::{
    clock::Clocks,
    delay::Delay,
    //prelude::*
};

extern crate vl53l5cx_uld as uld;
use uld::Platform;

// Note: Vendor docs use 7-bit I2C addresses shifted one left
//
const I2C_ADDR: u8 = 0x52 >> 1;    // vendor default

/*
* Wiring (VL53L5CX-SATEL -> ESP32-C[36]):
*   - SDA => GPIO4      // same as in 'esp-hal' I2C example
*   - SCL => GPIO5      //  -''-
*   - LPn => 47k => GND
*/
pub struct MyPlatform<T>
    where T: embedded_hal::i2c::I2c<SevenBitAddress>
{
    i2c: RefCell<T>,
    d_provider: Delay,
}

impl<T> MyPlatform<T>
    where T: embedded_hal::i2c::I2c<SevenBitAddress>
{
    pub fn new(clocks: &Clocks, i2c: T) -> Self {
        let i2c = RefCell::new(i2c);

        Self{
            i2c,
            d_provider: Delay::new(&clocks)
        }
    }
}

impl<T> Platform for MyPlatform<T>
    where T: embedded_hal::i2c::I2c<SevenBitAddress>,
    T::Error: Format    // defmt output can be made
{
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> Result<(),()> {
        //Rtrace!("I2C read: {:#06x}", addr);
        let addr_orig = addr;

        let addr = addr.to_be_bytes();
        let mut i2c = self.i2c.borrow_mut();

        match i2c.write_read(I2C_ADDR, &addr, buf) {
            Err(e) => { error!("I2C read failed: {}", e); Err(()) },
            Ok(()) => {
                trace!("I2C read: {:#06x} -> {:#04x}", addr_orig, buf);
                Ok(())
            }
        }
    }
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> Result<(),()> {
        trace!("I2C write: {:#06x} <- {:#04x}", addr, vs);

        let addr = addr.to_be_bytes();
        let mut i2c = self.i2c.borrow_mut();

        // Note: There didn't seem to be (Rust stable 1.80) a way to concat two slices together,
        //      in 'no-alloc'. However, the I2C library *might* support the below (adapted from
        //      the way '.write_read()' is implemented).
        // tbd. tell whether this works.

        match i2c.transaction(I2C_ADDR, &mut [I2cOperation::Write(&addr), I2cOperation::Write(vs)]) {
            Err(e) => { error!("I2C write failed: {}", e); Err(()) },
            Ok(()) => Ok(())
        }
    }

    fn delay_ms(&mut self, ms: u32) {
        trace!("🔸 {}ms", ms);

        self.d_provider.delay_millis(ms);
    }
}

/*** WIP
// Need *SOME* way to turn the platform pointer from vendor ULD C API, into the Rust platform.
//
pub fn surface<P: Platform>(vp: *mut c_void) -> &'static mut P {

    let p: &mut RustPlatform = unsafe {
        NonNull::new_unchecked(vp).cast::<RustPlatform>().as_mut()
    };

    p.print();
}
***/