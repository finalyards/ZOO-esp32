/*
* 'Flock-platform' uses interior mutability [1] to share a single, application side 'Platform'
* with multiple sensors access. Access to the sensors is serialized, so the Rust guarantees on
* references are fulfilled.
*
* References:
*   - The Rust Programming Language > RefCell<T> and the Interior Mutability Pattern
*       -> https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
*/
use core::{
    cell::RefCell,
    result::Result as CoreResult
};

use vl53l5cx_uld::Platform;

/*
* Sharing the one 'Platform' to multiple (sequential) users.
*/
pub struct Dispenser<P: Platform + 'static>(
    RefCell<P>
);

impl<P: Platform + 'static> Dispenser<P> {
    pub fn new(p: P) -> Self {
        Self( RefCell::new(p) )
    }
}

/*
* Rust note:
*   It's _cool_ (and ...weird?) how Rust allows one to implement traits on _references_.
*   But it's really handy. This means "if you have a (read-only; can be multiple) reference to
*   'Dispenser', you can use it as a ULD 'Platform'. SO NEAT!
*/
impl<P: Platform + 'static> Platform for &Dispenser<P> {
    fn rd_bytes(&mut self, index: u16, buf: &mut [u8]) -> CoreResult<(), ()> {
        self.0.get_mut().rd_bytes(index, buf)
    }

    fn wr_bytes(&mut self, index: u16, vs: &[u8]) -> CoreResult<(), ()> {
        self.0.get_mut().wr_bytes(index, vs)
    }

    fn delay_ms(&mut self, ms: u32) {
        self.0.get_mut().delay_ms(ms)
    }
}
