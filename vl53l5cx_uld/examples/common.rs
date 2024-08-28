#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace};

#[allow(unused_imports)]
use core::{
    cell::RefCell,
    ffi::c_void,
    mem::MaybeUninit,
    ptr::NonNull
};
use defmt::Format;
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

// Maximum size of a block that can be written to I2C, via 'esp-hal', at once.
// Unfortunately, the 'esp-hal' module does not expose this, as a constant.
//
const MAX_WR_BLOCK: usize = 254;
    // Note: The number doesn't contribute to getting 'TimeOut'.

/*
* Wiring (VL53L5CX-SATEL -> ESP32-C[36]):
*   - SDA => GPIO4      // same as in 'esp-hal' I2C example
*   - SCL => GPIO5      //  -''-
*   - LPn => 47k => GND
*/
pub struct MyPlatform<T>
    where T: embedded_hal::i2c::I2c<SevenBitAddress>
{
    // tbd. No need for 'embedded_hal'.
    // tbd. Get rid of the 'RefCell'; embedded-hal/I2c docs tell to use just 'I2C' #study
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
            d_provider: Delay::new(&clocks),
        }
    }
}

impl<T> Platform for MyPlatform<T>
    where T: embedded_hal::i2c::I2c<SevenBitAddress>,
    T::Error: Format    // defmt output can be made
{
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> Result<(),()> {
        let addr_orig = addr;

        let addr = &addr.to_be_bytes();
        let mut i2c = self.i2c.borrow_mut();

        match i2c.write_read(I2C_ADDR, addr, buf) {
            Err(e) => { error!("I2C read failed: {}", e); Err(()) },
            Ok(()) => {
                trace!("I2C read: {:#06x} -> {:#04x}", addr_orig, buf);
                Ok(())
            }
        }
    }

    /***
    * Vendor ULD driver calls us with chunk lengths of 32768, during the initialization.
    *
    * The 'esp-hal' (v. 0.19.0) has a limit of 254 (inclusive) bytes per write, beyond which
    * 'ExceedingFifo' error is returned. This means we need to chunk the larger parts, no matter what.
    * Which is kind of good, since it allows us an excuse to make a buffer ourselves, and clearly
    * states what size the buffer should be.
    *
    * Note: The '254' is hard coded within 'esp-hal', not exposed as a value we could read,
    *       automatically.
    */
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> Result<(),()> {
        let mut i2c = self.i2c.borrow_mut();
        let addr_orig = addr;
        let mut addr = addr;    // rolled further with the chunks

        // Note: We can just chunk everything (if + recursion being the alternative).
        //
        for chunk in vs.chunks(MAX_WR_BLOCK-2) {
            let n: usize = chunk.len();

            trace!("Writing: {:#06x} <- {=usize} bytes", addr, n);

            // Writing needs to be done in one block, where the first two bytes are the address.
            //
            // Unfortunately, wasn't able to find a way in 'no_alloc' Rust to concatenate slices
            // together. Obvious, since slices are *continuous memory blocks*, so either some
            // allocation, and/or copying bytes, is inevitable.
            //
            // We could:
            //  - bring in 'alloc' and use '[addr,vs].concat()' [no]
            //  - use a singular buffer with 'self' (overkill)
            //  - use a local buffer, separate on each call (good, especially since we know that
            //    the maximum for 'esp-hal' is rather small).
            //
            let mut buf: [u8;MAX_WR_BLOCK] = { let un = MaybeUninit::zeroed(); unsafe { un.assume_init() }};
            let out: &[u8] = {
                buf[0..2].copy_from_slice(&addr.to_be_bytes());
                buf[2..2+n].copy_from_slice(chunk);
                &buf[..2+n]
            };

            i2c.write(I2C_ADDR, &out).map_err(|e|
                { error!("I2C write failed: {}", e); () }
            )?;
            addr = addr + n as u16;
        }

        trace!("I2C written: {:#06x} <- {:#04x}", addr_orig, slice_head(vs,20));
        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) {
        trace!("🔸 {}ms", ms);

        self.d_provider.delay_millis(ms);
    }
}

fn slice_head(vs: &[u8],n_max: usize) -> &[u8] {
    use core::cmp::min;
    &vs[..min(vs.len(),n_max)]
}
