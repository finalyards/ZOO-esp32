
// tbd. find standard structs for 'Hz', 'ms'; are there such?

use defmt::{Format, Formatter};

#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Hz(pub u8);      // VL needs max 15 and 60

#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ms(pub u16);     // likely enough to go to ~1min (60t); VL uses 'u32'

#[derive(Clone)]
pub struct TempC(pub i8);

#[cfg(feature = "defmt")]
impl Format for TempC {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "{=i8}°C", self.0);
    }
}
