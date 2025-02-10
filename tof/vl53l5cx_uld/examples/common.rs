#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace, panic};

use esp_hal::{
    delay::Delay,
    i2c::master::{I2c, Operation},
    Blocking,
};
#[cfg(not(feature = "esp-hal-0_22"))]
use esp_hal::i2c::master::I2cAddress;

extern crate vl53l5cx_uld as uld;
use uld::{
    DEFAULT_I2C_ADDR,
    I2cAddr,
    Platform,
};

#[cfg(not(feature = "esp-hal-0_22"))]
const I2C_ADDR: I2cAddress = I2cAddress::SevenBit( DEFAULT_I2C_ADDR.as_7bit() );    // esp-hal address type

#[cfg(feature = "esp-hal-0_22")]
const I2C_ADDR: u8 = DEFAULT_I2C_ADDR.as_7bit();

/*
*/
pub struct MyPlatform {
    i2c: I2c<'static, Blocking>,
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl MyPlatform {
    #[allow(non_snake_case)]
    pub fn new(i2c: I2c<'static,Blocking>) -> Self {
        Self{ i2c }
    }
}

impl Platform for MyPlatform {
    /*
    */
    fn rd_bytes(&mut self, index: u16, buf: &mut [u8]) -> Result<(),()/* !*/> {     // "'!' type is experimental"

        self.i2c.write_read(I2C_ADDR, &index.to_be_bytes(), buf).unwrap_or_else(|e| {
            // If we get an error, let's stop right away.
            panic!("I2C read at {:#06x} ({=usize} bytes) failed: {}", index, buf.len(), e);
        });

        // Whole 'buf' should now have been read in.
        //
        if buf.len() <= 20 {
            trace!("I2C read: {:#06x} -> {:#04x}", index, buf);
        } else {
            trace!("I2C read: {:#06x} -> {:#04x}... ({} bytes)", index, slice_head(buf,20), buf.len());
        }

        // There should be 1.3ms between transmissions, by the VL spec. (see 'tBUF', p.15)
        blocking_delay_us(1000);    // 1300

        Ok(())
    }

    /***
    * Vendor ULD driver calls us with chunk lengths of 32768, during the initialization.
    *
    * IF we get errors from the HAL, we panic. ULD C level would often go on for too long; it's best
    * to stop early. CERTAIN error codes MAY lead to a single retry, if we think we have a chance
    * to recover.
    */
    fn wr_bytes(&mut self, index: u16, vs: &[u8]) -> Result<(),() /* !*/> {   // "'!' type is experimental" (nightly)
        trace!("Writing: {:#06x} <- {:#04x}", index, vs);    // TEMP

        // 'esp-hal' doesn't have '.write_write()', but it's easy to make one. This means we don't
        // need to concatenate the slices in a buffer.
        //
        self.i2c.transaction(I2C_ADDR, &mut [Operation::Write(&index.to_be_bytes()), Operation::Write(&vs)]).unwrap_or_else(|e| {
            panic!("I2C write to {:#06x} ({=usize} bytes) failed: {}", index, vs.len(), e);
        });

        let n = vs.len();
        if n <= 20 {
            trace!("I2C written: {:#06x} <- {:#04x}", index, vs);
        } else {
            trace!("I2C written: {:#06x} <- {:#04x}... ({=usize} bytes)", index, slice_head(vs,20), n);
        }

        // There should be 1.3ms between transmissions, by the VL spec. (see 'tBUF', p.15)
        blocking_delay_us(1000);    // 1300

        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) {
        trace!("🔸 {}ms", ms);
        blocking_delay_us(ms*1000);
    }

    fn addr_changed(&mut self, _: &I2cAddr) {
        unimplemented!()
    }
}

fn slice_head(vs: &[u8],n_max: usize) -> &[u8] {
    use core::cmp::min;
    &vs[..min(vs.len(),n_max)]
}

const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_us(us: u32) {
    D_PROVIDER.delay_micros(us);
}
