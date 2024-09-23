/*
* 'Flock-platform' uses interior mutability [1] to share a single, application side 'Platform'
* with multiple sensors access. The access to said sensors is serialized - our code handles only
* one at a time, so the Rust guarantees on (mutable) references are fulfilled.
*
* References:
*   - The Rust Programming Language > RefCell<T> and the Interior Mutability Pattern
*       -> https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
*/
use core::{
    cell::RefCell,
    result::Result as CoreResult
};

extern crate vl53l5cx_uld as uld;
use uld::{
    Platform,
};

/*
* Sharing the one 'Platform' to multiple (sequential) users.
*/
pub struct Dispenser<P: Platform + 'static> {
    p: P
}

pub struct Cover<P: Platform + 'static>(RefCell<P>);    // needed for implementing 'Platform' on the spread type

impl<P: Platform + 'static> Dispenser<P> {
    pub fn new(p: P) -> Self {
        Self{p}
    }
    pub fn dispense(&mut self) -> Cover<P> {
        Cover(
            RefCell::new(&self.p)
        )
    }
}

impl<P: Platform + 'static> Platform for Cover<P> {
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
