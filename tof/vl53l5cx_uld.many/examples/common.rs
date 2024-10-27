// THIS IS A COPY from '../../vl53l5cx_uld/examples/common.rs' (almost)

// tbd. Since the 'flock' lib anyways has a dependency on 'esp-hal', consider making this part
//      of the lib.

#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace, panic};

use core::{
    mem::MaybeUninit,
    iter::zip
};
use esp_hal::{
    delay::Delay,
    i2c::{I2c, Instance},
    Blocking
};

use vl53l5cx_uld::{     // <-- differs here from 'vl53l5cx_uld/examples'
    Platform,
    DEFAULT_I2C_ADDR
};

// Maximum sizes for I2C writes and reads, via 'esp-hal'. Unfortunately, it does not expose these
// as values we could read (the values are burnt-in).
//
const MAX_WR_LEN: usize = 254;
const MAX_RD_LEN: usize = 254;      // trying to read longer than this would err with 'ExceedingFifo'

const D_PROVIDER: Delay = Delay::new();

const I2C_ADDR: u8 = DEFAULT_I2C_ADDR >> 1;

/*
*/
pub struct MyPlatform<'a, T: Instance> {
    i2c: I2c<'a, T, Blocking>,
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl<'a,T> MyPlatform<'a,T> where T: Instance {
    #[allow(non_snake_case)]
    pub fn new(i2c: I2c<'a,T,Blocking>) -> Self {
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
        let mut index = index;    // rolled further with the chunks

        let chunks = buf.chunks_mut(MAX_RD_LEN);
        let rounds = (&chunks).len();

        // Chunks we get are *views* to the 'buf' backing them. Thus, reading to the chunk automatically
        // fills it.
        //
        for (round,chunk) in zip(1.., chunks) {

            match self.i2c.write_read(I2C_ADDR, &index.to_be_bytes(), chunk) {
                Err(e) => {
                    // If we get an error, let's stop right away.
                    panic!("I2C read at {:#06x} ({=usize} bytes; chunk {}/{}) failed: {}", index_orig, buf.len(), round, rounds, e);
                },
                Ok(()) => {
                    index = index + chunk.len() as u16;

                    // There should be 1.2ms between transactions, by the VL spec.
                    delay_ms(1);
                }
            };
        }

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
    * The 'esp-hal' (v. 0.19.0) has a limit of 254 (inclusive) bytes per write, beyond which
    * 'ExceedingFifo' error is returned. Unfortunately, we cannot 'use' this limit in code; it's
    * hard coded. In order to proceed, we chunk the larger parts. ((Which is kind of good, since
    * it allows us an excuse to make a limited buffer ourselves, which we need for merging the
    * writing of the address, and the data bytes, into a *single* write transaction. There are no
    * slice concatenation in 'alloc':less Rust.))
    *
    * IF we get errors from the HAL, we panic. ULD C level would often go on for too long; it's best
    * to stop early. CERTAIN error codes MAY lead to a single retry, if we think we have a chance
    * to recover.
    *
    * Note: The '254' is hard coded within 'esp-hal', not exposed as a value we could read,
    *       automatically.
    */
    fn wr_bytes(&mut self, index: u16, vs: &[u8]) -> Result<(),() /* !*/> {   // "'!' type is experimental" (nightly)
        let mut index = index;    // rolled further with the chunks

        const MAX_CHUNK_LEN: usize = MAX_WR_LEN-2;
        let chunks = vs.chunks(MAX_CHUNK_LEN);
        let _rounds = (&chunks).len();   // needs taking before we move 'chunks' as an iterator

        let mut buf: [u8;MAX_WR_LEN] = { let un = MaybeUninit::zeroed(); unsafe { un.assume_init() }};

        // Note: We can just chunk everything (if + recursion being the alternative).
        //
        for (_round,chunk) in zip(1.., chunks) {
            let n: usize = chunk.len();

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
            let out: &[u8] = {
                buf[0..2].copy_from_slice(&index.to_be_bytes());
                buf[2..2+n].copy_from_slice(chunk);
                &buf[..2+n]
            };

            self.i2c.write(I2C_ADDR, &out).unwrap_or_else(|e| {
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
            delay_ms(1);
        }

        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) {
        trace!("🔸 {}ms", ms);
        delay_ms(ms);
    }
}

fn slice_head(vs: &[u8],n_max: usize) -> &[u8] {
    use core::cmp::min;
    &vs[..min(vs.len(),n_max)]
}

fn delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
