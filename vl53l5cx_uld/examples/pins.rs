
use esp_hal::{
    gpio::{Io, InputPin, OutputPin, NO_PIN},
};

#[allow(non_camel_case_types)]
pub fn get_pins<SDA,SCL,PWR_EN,_RESERVED>(&io: &Io) -> (SDA,SCL,Option<PWR_EN>,Option<_RESERVED>)
    where SDA: OutputPin + InputPin,
        SCL: OutputPin + InputPin,
        PWR_EN: OutputPin,
        _RESERVED: OutputPin        // tbd. either I2C_RST and/or INT (pull down pin)
{
    // changed via running './set-target.sh'
    (io.pins.gpio4, io.pins.gpio5, Some(io.pins.gpio0), NO_PIN)      // esp32c3
    //(io.pins.gpio22, io.pins.gpio23, Some(io.pins.gpio21), NO_PIN)    // esp32c6
}
