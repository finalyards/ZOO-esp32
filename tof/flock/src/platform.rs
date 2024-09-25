/*
* Flock-level 'Platform' uses interior mutability [1] to share a single, application side 'Platform'
* with multiple sensors. Access to the sensors is serialized, so the Rust guarantees on references
* are fulfilled.
*
* References:
*   - The Rust Programming Language > RefCell<T> and the Interior Mutability Pattern
*       -> https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
*/
use core::{
    cell::RefCell,
    result::Result as CoreResult
};

use vl53l5cx_uld::{
    Platform
};

/*
* Sharing the one 'Platform' to multiple (sequential) users.
*/
type Dispenser<P: Platform + 'static> = RefCell<P>;

/*
* Rust note:
*   It's cool (and ...somewhat weird, learning these things...) how Rust allows one to implement
*   traits on _references_. References are other types than the referred type. Let is sink...
*
*   But it's really handy!!! This means, if you have a (read-only; can be multiple such) reference
*   to 'Dispenser' (which really is 'RefCell<P>', see above), you can use it _as-is_ as a ULD-level
*   'Platform'. SO NEAT!!! :)
*/
impl<P: Platform + 'static> Platform for &Dispenser<P> {
    fn rd_bytes(&mut self, index: u16, buf: &mut [u8]) -> CoreResult<(), ()> {
        self.0.get().rd_bytes(index, buf)
    }

    fn wr_bytes(&mut self, index: u16, vs: &[u8]) -> CoreResult<(), ()> {
        self.0.get().wr_bytes(index, vs)
    }

    fn delay_ms(&mut self, ms: u32) {
        self.0.get().delay_ms(ms)
    }
}
