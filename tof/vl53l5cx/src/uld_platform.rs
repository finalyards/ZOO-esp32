/*
* 'Platform' implementation for the VL53L5CX ULD interface.
*
* For access to the I2C bus, a 'RefCell' is used. The intended use is Embassy tasks, on the same
* priority, where multiple devices can borrow the bus, but not across 'await' boundaries.
*/
#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace, panic};

use esp_hal::{
    delay::Delay,
    i2c::master::Instance,
};

use vl53l5cx_uld::{
    DEFAULT_I2C_ADDR,
    I2cAddr,
    Platform
};

use core::cell::RefCell;
use crate::I2c_Blocking;

#[cfg(feature = "defmt")]
const TRACE_HEAD_N:usize=20;        // Number of first bytes to show

/*
*/
pub(crate) struct Pl<'a, T: Instance> {
    i2c_shared: &'a RefCell<I2c_Blocking<'a,T>>,
    i2c_addr: I2cAddr
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl<'a,T> Pl<'a,T>
    where T: Instance
{
    pub fn new(i2c_shared: &'a RefCell<I2c_Blocking<'a,T>>) -> Self {
        Self{
            i2c_shared,
            i2c_addr: DEFAULT_I2C_ADDR     // every board starts with the default address
        }
    }
}

impl<T> Platform for Pl<'_,T> where T: Instance
{
    // Note: With Rust Edition 2024 out, try '!' or 'Infallible' as the return type (we don't provide
    //      errors). In Edition 2021, Rust 1.82, 'Infallible' doesn't coerce to '()' (it could),
    //      so cannot use it now. //2-Nov-24

    /*
    * ULD reads can be in sizes of 492 bytes (or more). The 'esp-hal' requires these to be handled
    * in multiple parts.
    */
    fn rd_bytes(&mut self, index: u16, buf: &mut [u8]) -> Result<(),()/* !*/> {     // "'!' type is experimental"
        let index_orig = index;

        let mut i2c = self.i2c_shared.borrow_mut();

        match i2c.write_read(self.i2c_addr.as_7bit(), &index.to_be_bytes(), buf) {
            Err(e) => {
                // If we get an error, let's stop right away.
                panic!("I2C read at {:#06x} ({=usize} bytes) failed: {}", index_orig, buf.len(), e);
            },
            Ok(()) => {
                // There should be 1.2ms between transactions, by the VL spec.
                blocking_delay_ms(1);
            }
        };

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
    * IF we get errors from the HAL, we panic. ULD C level would often go on for too long; it's best
    * to stop early. CERTAIN error codes MAY lead to a single retry, if we think we have a chance
    * to recover.
    */
    fn wr_bytes(&mut self, index: u16, vs: &[u8]) -> Result<(),() /* !*/> {   // "'!' type is experimental" (nightly)
        use core::{iter::zip, mem::MaybeUninit};

        // 'esp-hal' has autochunking since 0.22.0. It doesn't, however,
        // have a 'I2c::write_write()' that would allow us to give two slices, to be written
        // consecutively (or using an iterator, which would also solve the case). VL53L5CX needs
        // this, because it takes the writing index as the first two bytes.
        //
        // Concatenating slices using 'ArrayVec' (or 'MaybeUninit') is an option. If we do that,
        // we might just as well explicitly chunk the whole payload, to keep RAM consumption small.
        // This is not a criticism of the esp-hal API. The VL53L5CX use of the I2C (in uploading
        // its firmware) is likely unusual - and it's not a bit trouble to keep the chunking in.
        // (However, we can now chunk in longer pieces than with 0.21.1).
        //
        const BUF_SIZE: usize = 10*1024;   // can be anything (1..32k)
        //const BUF_SIZE: usize = MAX_WR_LEN;   // prior (0.21.x)

        let mut index = index;    // rolled further with the chunks

        let chunks = vs.chunks(BUF_SIZE-2);
        let _rounds = (&chunks).len();   // needs taking before we move 'chunks' as an iterator

        let mut buf: [u8; BUF_SIZE] = {
            let un = MaybeUninit::zeroed();
            unsafe { un.assume_init() }
        };

        let mut i2c = self.i2c_shared.borrow_mut();
        let addr: u8 = self.i2c_addr.as_7bit();

        for (_round, chunk) in zip(1.., chunks) {
            let n: usize = chunk.len();

            // Writing needs to be done in one block, where the first two bytes are the address.
            //
            // Rust note:
            //      Since slices are *continuous memory blocks*, there's no way to concatenate two
            //      of them together, without allocation.
	    //
            // We could:
            //  - bring in 'alloc' and use '[addr,vs].concat()' [no]
            //  - use a singular buffer with 'self' (overkill)
            //  - use a local buffer, separate on each call (good; we steer the memory use)
            //
            let out: &[u8] = {
                buf[0..2].copy_from_slice(&index.to_be_bytes());
                buf[2..2 + n].copy_from_slice(chunk);
                &buf[..2 + n]
            };

            i2c.write(addr, &out).unwrap_or_else(|e| {
                // If we get an error, let's stop right away.
                panic!("I2C write to {:#06x} ({=usize} bytes) failed: {}", index, n, e);
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
    fn addr_changed(&mut self, addr: &I2cAddr) {
        self.i2c_addr = addr.clone();
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
