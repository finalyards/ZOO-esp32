/*
* Carriers for physical units (ms, Hz, °C)
*
*   - ability to pattern match; keep them apart from each other and integers
*   - pleasurable syntax for providing config: '1.ms()', '5.Hz()'
*
* Note:
*   - 'fugit' has duration (ms) and rate (Hz), but it is geared towards conversion rather than
*     carrying. It's a no.
*   - IF there is a public library that does these, happy to start using one.
*/
#[cfg(feature = "defmt")]
use defmt::{Format, Formatter};

// Input
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HzU8(pub u8);      // Vendor ULD needs max 15 and 60

// Input
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MsU16(pub u16);     // 'u16' enough to go to ~1min; vendor uses 'u32'

pub trait ExtU32 {
    fn ms(self) -> MsU16;

    #[allow(non_snake_case)]
    fn Hz(self) -> HzU8;
}

impl ExtU32 for u32 {
    #[inline]
    fn ms(self) -> MsU16 {
        assert!(self <= 0xffff);
        MsU16(self as u16)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn Hz(self) -> HzU8 {
        assert!(self <= 0xff);
        HzU8(self as u8)
    }
}

// Output
//
// Haven't found a general Rust library for carrying temperatures. We only need output.
//
// Note: Takes in also negative temperatures. Vendor ULD does, and it's.. possible the sensor gets
//      operated below 0°C.
//
#[derive(Copy, Clone)]
pub struct TempC(pub i8);

#[cfg(feature = "defmt")]
impl Format for TempC {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "{=i8}°C", self.0);
    }
}
