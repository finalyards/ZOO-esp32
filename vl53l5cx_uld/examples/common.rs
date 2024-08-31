#[allow(unused_imports)]
use defmt::{info, debug, error, warn, trace, panic};

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
    i2c::{I2C, Instance},
    Blocking
};

extern crate vl53l5cx_uld as uld;
use uld::Platform;

// Note: Vendor docs use 7-bit I2C addresses shifted one left
//
const I2C_ADDR: u8 = 0x52 >> 1;    // vendor default

// Maximum sizes for I2C writes and reads, via 'esp-hal'. Unfortunately, it does not expose these
// as values we could read (the values are burnt-in).
//
const MAX_WR_LEN: usize = 254;
const MAX_RD_LEN: usize = 254;      // trying to read longer than this would err with 'ExceedingFifo'

/*
* Wiring (VL53L5CX-SATEL -> ESP32-C[36]):
*   - SDA => GPIO4      // same as in 'esp-hal' I2C example
*   - SCL => GPIO5      //  -''-
*   - LPn => 47k => GND
*/
pub struct MyPlatform<'a, T: Instance> {
    i2c: I2C<'a, T, Blocking>,
    d_provider: Delay,
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl<'a,T> MyPlatform<'a,T> where T: Instance {
    pub fn new(clocks: &Clocks, i2c: I2C<'a,T,Blocking>) -> Self {
        Self{
            i2c,
            d_provider: Delay::new(&clocks),
        }
    }
}

impl<T> Platform for MyPlatform<'_,T> where T: Instance
{
    /*
    * Reads can be in sizes of 492 bytes (or more). The 'esp-hal' requires these to be handled
    * in multiple parts.
    */
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> Result<(),()/* !*/> {     // "'!' type is experimental"
        let addr_orig = addr;
        let mut addr = addr;    // rolled further with the chunks

        let chunks = buf.chunks_mut(MAX_RD_LEN);
        let rounds = (&chunks).len();

        // Chunks we get are *views* to the 'buf' backing them. Thus, reading to the chunk automatically
        // fills it.
        //
        for (round,chunk) in zip(1.., chunks) {

            match self.i2c.write_read(I2C_ADDR, &addr.to_be_bytes(), chunk) {
                Err(e) => {
                    // If we get an error, let's stop right away.
                    panic!("I2C read at {:#06x} ({=usize} bytes; chunk {}/{}) failed: {}", addr_orig, buf.len(), round, rounds, e);
                },
                Ok(()) => {
                    addr = addr + chunk.len() as u16;

                    // There should be 1.2ms between transactions, by the VL spec.
                    //
                    // #later: Check from the signal that/whether we are within spec (and remove delay).
                    //      - separately for both C3, C6!!!
                    //
                    self.d_provider.delay_millis(1);
                }
            };
        }

        // Whole 'buf' should now have been read in.
        //
        if buf.len() <= 20 {
            trace!("I2C read: {:#06x} -> {:#04x}", addr_orig, buf);
        } else {
            trace!("I2C read: {:#06x} -> {:#04x}... ({} bytes)", addr_orig, slice_head(buf,20), buf.len());
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
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> Result<(),() /* !*/> {   // "'!' type is experimental" (nightly)
        let mut addr = addr;    // rolled further with the chunks

        const MAX_CHUNK_LEN: usize = MAX_WR_LEN-2;
        let chunks = vs.chunks(MAX_CHUNK_LEN);
        let _rounds = (&chunks).len();   // needs taking before we move 'chunks' as an iterator

        let mut buf: [u8;MAX_WR_LEN] = { let un = MaybeUninit::zeroed(); unsafe { un.assume_init() }};

        // Note: We can just chunk everything (if + recursion being the alternative).
        //
        for (_round,chunk) in zip(1.., chunks) {
            let n: usize = chunk.len();

            /*** disabled /less trace
            if chunks_n == 1 {
                trace!("Writing: {:#06x} <- {=usize} bytes", addr, n);
            } else {
                trace!("Writing (round {}/{}): {:#06x} <- {=usize} bytes", round, rounds, addr, n);
            } ***/

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
                buf[0..2].copy_from_slice(&addr.to_be_bytes());
                buf[2..2+n].copy_from_slice(chunk);
                &buf[..2+n]
            };

            self.i2c.write(I2C_ADDR, &out).map_err(|e|
                { error!("I2C write to {:#06x} ({=usize} bytes) failed: {}", addr, n, e); () }
            )?;

            // Give the "written" log here, separately for each chunk (clearer to follow log).
            if n <= 20 {
                trace!("I2C written: {:#06x} <- {:#04x}", addr, chunk);
            } else {
                trace!("I2C written: {:#06x} <- {:#04x}... ({=usize} bytes)", addr, slice_head(chunk,20), n);
            }

            addr = addr + n as u16;

            // There should be 1.3ms between transactions, by the VL spec. (see 'tBUF', p.15)
            //
            // #later: Check from the signal that/whether we are within spec (and remove delay).
            //      - separately for both C3, C6!!!
            //
            self.d_provider.delay_millis(1);
        }

        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) {
        trace!("🔸 {}ms", ms);

        self.d_provider.delay_millis(ms);
    }
}

/*** REMOVE
fn write_chunk_with_retry(&mut some, bytes: &[u8]) {

    for round in 0..2 {
        match self.i2c.write(I2C_ADDR, &bytes) {
            Err(ExceedingFifo) if round==0 => {     // give a warning; try again
                warn!("'ExceedingFifo', reading {} bytes from {}. Will retry after {}ms", n, addr_orig, WAIT_MS);
                self.d_provider.delay_millis(10);
            },
            Err(e) => {
                // If we get an error, let's stop right away. The ULD C API would throw in more
                // transactions (which are likely to fail); it would only eventually yield.
                // We don't need to give it errors. Ever.
                //
                panic!("I2C read at {:#06x} ({} bytes) failed: {}", addr_orig, buf.len(), e);
                //was: error!("I2C read failed: {}", e); e
            },
            Ok(()) => {
                // tbd. can make a wrapper on the 'buf' that implements '::Format' so that we get
                //      nice ', ...]' -ending output. 🌞
                //
                if n <= 20 {
                    trace!("I2C read: {:#06x} -> {:#04x}", addr, buf);
                } else {
                    trace!("I2C read: {:#06x} -> {:#04x}... ({} bytes)", addr, slice_head(buf,20), n);
                }
                trace!("I2C read: {:#06x} -> {:#04x}", addr_orig, buf);
                return Ok(())
            }
        };
}***/

fn slice_head(vs: &[u8],n_max: usize) -> &[u8] {
    use core::cmp::min;
    &vs[..min(vs.len(),n_max)]
}
