mod platform;

mod uld;
mod uld_raw;

pub type Result<T> = core::result::Result<T,u8>;

/*** (We might not need this, at all)
// Note: vendor writes of 7 and 10-bit I2C addresses, but in practice, all addresses are 7 bits.
#[allow(non_camel_case_types)]
//#[derive(Copy, Clone)]      // tbd. is this needed?
pub struct I2C_Addr(u8);

impl I2C_Addr {
    fn from(v: u16) -> Self {
        assert!(v <= 0xff, "Unexpected I2C address: {}", v);
        Self(v as u8)
    }
}
***/

pub use uld::{
    API_REVISION
};     // pass through
