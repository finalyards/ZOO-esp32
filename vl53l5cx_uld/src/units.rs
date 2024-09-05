
// tbd. find standard structs for 'Hz', 'ms' in ESP32; are there such?

#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Hz(pub u8);      // VL needs max 15 and 60; uses 'u8'

#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ms(pub u16);     // likely enough to go to ~1min (60t); VL uses 'u32'
