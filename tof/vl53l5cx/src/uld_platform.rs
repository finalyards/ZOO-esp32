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
    i2c::master::I2c,
    Blocking
};

use vl53l5cx_uld::{
    DEFAULT_I2C_ADDR,
    I2cAddr,
    Platform
};

use core::cell::RefCell;

#[cfg(feature = "defmt")]
const TRACE_HEAD_N:usize=20;        // Number of first bytes to show

/*
*/
pub(crate) struct Pl<'a> {
    i2c_shared: &'a RefCell<I2c<'static, Blocking>>,
    i2c_addr: I2cAddr
}

// Rust note: for the lifetime explanation, see:
//  - "Lost in lifetimes" (answer)
//      -> https://users.rust-lang.org/t/lost-with-lifetimes/82484/4?u=asko
//
impl<'a> Pl<'a> {
    pub fn new(i2c_shared: &'a RefCell<I2c<'static, Blocking>>) -> Self {
        Self{
            i2c_shared,
            i2c_addr: DEFAULT_I2C_ADDR     // every board starts with the default address
        }
    }
}

impl Platform for Pl<'_> {
    // Note: With Rust Edition 2024 out, try '!' or 'Infallible' as the return type (we don't provide
    //      errors). In Edition 2021, Rust 1.82, 'Infallible' doesn't coerce to '()' (it could),
    //      so cannot use it now. //2-Nov-24

    /*
    * ULD reads can be in sizes of 492 bytes (or more). The 'esp-hal' requires these to be handled
    * in multiple parts.
    */
    fn rd_bytes(&mut self, index: u16, buf: &mut [u8]) -> Result<(),()/* !*/> {     // "'!' type is experimental"
        let mut i2c = self.i2c_shared.borrow_mut();

        i2c.write_read(self.i2c_addr.as_7bit(), &index.to_be_bytes(), buf)
        .unwrap_or_else(|e| {
            panic!("I2C read at {:#06x} ({=usize} bytes) failed: {}", index, buf.len(), e);
        });

        #[cfg(feature = "defmt")]
        {
            if buf.len() <= TRACE_HEAD_N {
                trace!("I2C read: {:#06x} -> {:#04x}", index_orig, buf);
            } else {
                trace!("I2C read: {:#06x} -> {:#04x}... ({} bytes)", index_orig, slice_head(buf,TRACE_HEAD_N), buf.len());
            }
        }

        // There should be 1.2ms between transactions, by the VL spec.
        blocking_delay_us(1000);

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
        let mut i2c = self.i2c_shared.borrow_mut();
        let addr: u8 = self.i2c_addr.as_7bit();

        // 'esp-hal' doesn't have '.write_write()', but it's easy to make one.
        //
        i2c.transaction(addr, &mut [Operation::Write(&index.to_be_bytes()), Operation::Write(&vs)])
        .unwrap_or_else(|e| {
            panic!("I2C write to {:#06x} ({=usize} bytes) failed: {}", index, vs.len(), e);
        });

        #[cfg(feature = "defmt")]
        {
            let n = vs.len();
            if n <= TRACE_HEAD_N {
                trace!("I2C written: {:#06x} <- {:#04x}", index, vs);
            } else {
                trace!("I2C written: {:#06x} <- {:#04x}... ({=usize} bytes)", index, slice_head(vs,TRACE_HEAD_N), n);
            }
        }

        // There should be 1.3ms between transmissions, by the VL spec. (see 'tBUF', p.15)
        blocking_delay_us(1000);    // 1300

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
fn blocking_delay_us(us: u32) {
    D_PROVIDER.delay_micros(us);
}
