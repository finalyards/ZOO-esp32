#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace};

#[allow(unused_imports)]
use core::{
    cell::RefCell,
    ffi::c_void,
    mem::MaybeUninit,
    ptr::NonNull,
    iter::zip
};
use esp_hal::{
    clock::Clocks,
    delay::Delay,
    i2c::{self, I2C},
    Blocking
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

/*
* Wiring (VL53L5CX-SATEL -> ESP32-C[36]):
*   - SDA => GPIO4      // same as in 'esp-hal' I2C example
*   - SCL => GPIO5      //  -''-
*   - LPn => 47k => GND
*/
pub struct MyPlatform<'a, T: i2c::Instance> {
    i2c: I2C<'a, T, Blocking>,
    d_provider: Delay,
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl<'a,T> MyPlatform<'a,T> where T: i2c::Instance {
    pub fn new(clocks: &Clocks, i2c: I2C<'a,T,Blocking>) -> Self {
        Self{
            i2c,
            d_provider: Delay::new(&clocks),
        }
    }
}

impl<T> Platform for MyPlatform<'_,T> where T: i2c::Instance
{
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> Result<(),()> {
        let addr_orig = addr;
        let addr = &addr.to_be_bytes();

        match self.i2c.write_read(I2C_ADDR, addr, buf) {
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
        let mut addr = addr;    // rolled further with the chunks

        const MAX_CHUNK_LEN: usize = MAX_WR_BLOCK-2;
        let chunks = vs.chunks(MAX_CHUNK_LEN);
        let chunks_n = (&chunks).len();   // needs taking before we move 'chunks' as an iterator

        // Note: We can just chunk everything (if + recursion being the alternative).
        //
        for (round,chunk) in zip(1.., chunks) {
            let n: usize = chunk.len();

            if chunks_n == 1 {
                trace!("Writing: {:#06x} <- {=usize} bytes", addr, n);
            } else {
                trace!("Writing (round {}/{}): {:#06x} <- {=usize} bytes", round, chunks_n, addr, n);
            }

            // Does NOT make a difference
            /***R #[cfg(feature = "exp_mitigate_timeouts")]
            if n >= 100 {     // delay before large chunks; EXP
                self.delay_ms(50);
            }***/

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

            self.i2c.write(I2C_ADDR, &out).map_err(|e|
                { error!("I2C write failed: {}", e); () }
            )?;

            // Give the "written" log here, separately for each chunk (clearer to follow log).
            if n <= 20 {
                trace!("I2C written: {:#06x} <- {:#04x}", addr, chunk);
            } else {
                trace!("I2C written: {:#06x} <- {:#04x}... ({} bytes)", addr, slice_head(chunk,20), n);
            }

            addr = addr + n as u16;

            // There should be 1.2ms between transactions. Until the 'TimeOut' issue is resolved,
            // let's keep a delay here.
            //
            // #later: Check from the signal that/whether we are within spec (and remove delay).
            self.delay_ms(2);
        }
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
