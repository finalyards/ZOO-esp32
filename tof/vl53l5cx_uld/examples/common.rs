#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace, panic};

use esp_hal::{
    delay::Delay,
    Blocking,
};
use esp_hal::i2c::master::Instance;

extern crate vl53l5cx_uld as uld;
use uld::{
    DEFAULT_I2C_ADDR as I2C_ADDR,
    I2cAddr,
    Platform,
};

#[allow(non_camel_case_types)]
type I2c_Blocking<'a,T /*: Instance*/> = esp_hal::i2c::master::I2c<'a,Blocking,T>;

/*
*/
pub struct MyPlatform<'a, T: Instance> {
    i2c: I2c_Blocking<'a,T>
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl<'a,T> MyPlatform<'a,T> where T: Instance {
    #[allow(non_snake_case)]
    pub fn new(i2c: I2c_Blocking<'a,T>) -> Self {
        Self{
            i2c,
        }
    }
}

impl<T> Platform for MyPlatform<'_,T> where T: Instance
{
    /*
    * Reads can be in sizes of 492 bytes (or more). The 'esp-hal' requires these to be handled
    * in multiple parts.
    */
    fn rd_bytes(&mut self, index: u16, buf: &mut [u8]) -> Result<(),()/* !*/> {     // "'!' type is experimental"
        let index_orig = index;

        match self.i2c.write_read(I2C_ADDR.as_7bit(), &index.to_be_bytes(), buf) {
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
        if buf.len() <= 20 {
            trace!("I2C read: {:#06x} -> {:#04x}", index_orig, buf);
        } else {
            trace!("I2C read: {:#06x} -> {:#04x}... ({} bytes)", index_orig, slice_head(buf,20), buf.len());
        }

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

            self.i2c.write(I2C_ADDR.as_7bit(), &out).unwrap_or_else(|e| {
                // If we get an error, let's stop right away.
                panic!("I2C write to {:#06x} ({=usize} bytes) failed: {}", index, n, e);
            });

            // Give the "written" log here, separately for each chunk (clearer to follow log).
            if n <= 20 {
                trace!("I2C written: {:#06x} <- {:#04x}", index, chunk);
            } else {
                trace!("I2C written: {:#06x} <- {:#04x}... ({=usize} bytes)", index, slice_head(chunk,20), n);
            }

            index = index + n as u16;

            // There should be 1.3ms between transactions, by the VL spec. (see 'tBUF', p.15)
            blocking_delay_ms(1);
        }

        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) {
        trace!("🔸 {}ms", ms);
        blocking_delay_ms(ms);
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

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
