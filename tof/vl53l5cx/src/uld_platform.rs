/*
* 'Platform' implementation for the VL53L5CX ULD interface.
*
* For access to the I2C bus, a 'RefCell' is used. The intended use is Embassy tasks, on the same
* priority, where multiple devices can borrow the bus, but not across 'await' boundaries.
*/
#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace, panic};

use core::{
    cell::RefCell,
    mem::MaybeUninit,
    iter::zip
};
use core::convert::Infallible;
use esp_hal::{
    delay::Delay,
    i2c::{I2c, Instance},
    Blocking
};

use vl53l5cx_uld::{Platform, DEFAULT_I2C_ADDR_8BIT};

use crate::I2cAddr;

// Maximum sizes for I2C writes and reads, via 'esp-hal' (0.19.0). Unfortunately, it does not
// expose these as values we could read (the values are burnt-in).
//
const MAX_WR_LEN: usize = 254;
const MAX_RD_LEN: usize = 254;      // trying to read longer than this would err with 'ExceedingFifo'

#[cfg(feature = "defmt")]
const TRACE_HEAD_N:usize=20;        // Number of first bytes to show

/*
*/
pub(crate) struct Pl<'a, T: Instance> {
    i2c_shared: &'a RefCell<I2c<'a, T, Blocking>>,
    i2c_addr: I2cAddr
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl<'a,T> Pl<'a,T>
    where T: Instance
{
    pub fn new(i2c_shared: &'a RefCell<I2c<'a, T,Blocking>>) -> Self {
        Self{
            i2c_shared,
            i2c_addr: I2cAddr::from_8bit(DEFAULT_I2C_ADDR_8BIT)     // every board starts with the default address
        }
    }
}

impl<T> Platform for Pl<'_,T> where T: Instance
{
    /*
    * ULD reads can be in sizes of 492 bytes (or more). The 'esp-hal' requires these to be handled
    * in multiple parts.
    */
    fn rd_bytes(&mut self, index: u16, buf: &mut [u8]) -> Result<(),Infallible /* !*/> {     // 'Infallible' -> '!' when no longer experimental (on stable)
        let index_orig = index;

        let chunks = buf.chunks_mut(MAX_RD_LEN);
        let _rounds = chunks.len();

        // Chunks we get are *views* to the 'buf' backing them. Thus, reading to the chunk automatically
        // fills it.
        //
        let mut i2c = self.i2c_shared.borrow_mut();
        let addr: u8 = self.i2c_addr.as_7bit();

        let mut index = index;    // rolled further with the chunks

        for (_round,chunk) in chunks.enumerate() {
            i2c.write_read(addr, &index.to_be_bytes(), chunk).unwrap_or_else(|e| {
                // If we get an error, let's stop right away.
                panic!("I2C read at {:#06x} ({=usize} bytes; chunk {}/{}) failed: {}", index_orig, buf.len(), _round+1, _rounds, e);
            });

            index = index + chunk.len() as u16;

            // There should be 1.2ms between transactions, by the VL spec.
            blocking_delay_ms(1);
        }

        // Whole 'buf' should now have been read in.
        //
        #[cfg(feature = "defmt")]
        {
            if buf.len() <= TRACE_HEAD_N {
                trace!("I2C read: {:#06x} -> {:#04x}", index_orig, buf);
            } else {
                trace!("I2C read: {:#06x} -> {:#04x}... ({} bytes)", index_orig, slice_head(buf,TRACE_HEAD_N), buf.len());
            }
        }

        Ok(())
    }

    /***
    * Vendor ULD driver calls us with up to 32768 bytes, during the initialization.
    *
    * The 'esp-hal' has a limit of 254 (inclusive) bytes per write, beyond which 'ExceedingFifo'
    * error is returned. In order to proceed, we chunk the larger parts. ((Which is kind of good,
    * since it allows us an excuse to make a limited buffer ourselves, which we need for merging
    * the writing of the address, and the data bytes, into a *single* write transaction. There are
    * no slice concatenation in 'alloc':less Rust.))
    *
    * IF we get errors from the HAL, we panic. ULD C level would often go on for too long; it's best
    * to stop early. CERTAIN error codes MAY lead to a single retry, if we think we have a chance
    * to recover.
    */
    fn wr_bytes(&mut self, index: u16, vs: &[u8]) -> Result<(),Infallible /* !*/> {   // Infallible -> '!' when no longer experimental (on stable)
        let index_orig = index;

        let chunks = vs.chunks(MAX_WR_LEN-2);
        let _rounds = chunks.len();   // needs taking before we consume 'chunks' as an iterator

        let mut buf: [u8;MAX_WR_LEN] = unsafe { MaybeUninit::zeroed().assume_init() };

        let mut i2c = self.i2c_shared.borrow_mut();
        let addr: u8 = self.i2c_addr.as_7bit();

        let mut index = index;    // rolled further with the chunks

        for (_round,chunk) in chunks.enumerate() {
            let n: usize = chunk.len();

            // Writing needs to be done in one block, where the first two bytes are the index.
            //
            // Rust note:
            //      Since slices are *continuous memory blocks*, there's no way to concatenate two
            //      of them together, without allocation.

            // Make a slice of the right length
            let out: &[u8] = {
                buf[0..2].copy_from_slice(&index.to_be_bytes());
                buf[2..2+n].copy_from_slice(chunk);
                &buf[..2+n]
            };

            i2c.write(addr, &out).unwrap_or_else(|e| {
                // If we get an error, let's stop right away.
                panic!("I2C write to {:#06x} ({=usize} bytes; chunk {}/{}) failed: {}", index_orig, n, _round+1, _rounds, e);
            });

            // Give the "written" log here, separately for each chunk (clearer to follow log).
            #[cfg(feature = "defmt")]
            {
                if n <= TRACE_HEAD_N {
                    trace!("I2C written: {:#06x} <- {:#04x}", index, chunk);
                } else {
                    trace!("I2C written: {:#06x} <- {:#04x}... ({=usize} bytes)", index, slice_head(chunk,TRACE_HEAD_N), n);
                }
            }

            index = index + n as u16;

            // There should be 1.3ms between transactions, by the VL spec. (see 'tBUF', p.15)
            blocking_delay_ms(1);
        }

        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) {
        #[cfg(feature = "defmt")]
        trace!("🔸 {}ms", ms);   // shows traces when the ULD code calls for delays (not for our own short ones)

        blocking_delay_ms(ms);
    }

    /*
    * During a ULD I2C address change, the address changes in-the-fly. We get called, once the next
    * transaction should use a new address. This is transparent to the application level.
    */
    fn addr_changed(&mut self, new_addr_8bit: u8) {
        self.i2c_addr = I2cAddr::from_8bit(new_addr_8bit);
    }
}

fn slice_head(vs: &[u8],n_max: usize) -> &[u8] {
    use core::cmp::min;
    &vs[..min(vs.len(),n_max)]
}

const D_PROVIDER: Delay = Delay::new();
fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
