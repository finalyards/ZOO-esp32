mod platform;

mod uld;
mod uld_raw;

pub type Result<T> = core::result::Result<T,u8>;

#[allow(non_camel_case_types)]
pub struct I2C_Addr(u8);

impl I2C_Addr {
    fn from(v: u16) -> Self {
        assert!(v <= 0xff, "Unexpected I2C address: {}", v);
        Self(v as u8)
    }
}