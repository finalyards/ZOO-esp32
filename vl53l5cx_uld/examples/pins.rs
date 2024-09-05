
use esp_hal::{
    gpio::{Io, InputPin, OutputPin, NO_PIN},
    peripheral::Peripheral
};

/***R
* This does NOT compile:
#[allow(non_camel_case_types)]
pub fn get_pins<SDA,SCL,PWR_EN,_RESERVED>(&io: &Io) -> (SDA, SCL, Option<PWR_EN>,Option<_RESERVED>)
    where SDA: Peripheral<P = impl OutputPin + InputPin>,
        SCL: Peripheral<P = impl OutputPin + InputPin>,
        PWR_EN: Peripheral<P = impl OutputPin>,
        _RESERVED: Peripheral<P = impl OutputPin>        // tbd. either I2C_RST and/or INT (pull down pin)
    { }
***/

#[allow(non_camel_case_types)]
pub fn get_pins<SDA,SCL,PWR_EN,_RESERVED>(&io: &Io) -> (SDA, SCL, Option<PWR_EN>,Option<_RESERVED>)
where SDA: Peripheral<P: OutputPin + InputPin>,
      SCL: Peripheral<P: OutputPin + InputPin>,
      PWR_EN: Peripheral<P: OutputPin>,
      _RESERVED: Peripheral<P: OutputPin>,       // tbd. either I2C_RST and/or INT (pull down pin)
{   // changed via running './set-target.sh'
    (io.pins.gpio4, io.pins.gpio5, Some(io.pins.gpio0), NO_PIN)      // esp32c3
    //(io.pins.gpio22, io.pins.gpio23, Some(io.pins.gpio21), NO_PIN)    // esp32c6
}
