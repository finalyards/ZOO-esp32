
use esp_hal::{
    gpio::{Io, InputPin, OutputPin, NO_PIN, GpioPin},
    peripheral::Peripheral
};

#[allow(non_camel_case_types)]
pub fn get_pins<SDA,SCL,PWR_EN,_RESERVED, const A: u8, const B: u8, const C: u8, const D: u8>(&io: &Io) -> (SDA, SCL, Option<PWR_EN>,Option<_RESERVED>)
where SDA: Peripheral<P: OutputPin + InputPin> + From<GpioPin<A>>,
      SCL: Peripheral<P: OutputPin + InputPin> + From<GpioPin<B>>,
      PWR_EN: Peripheral<P: OutputPin> + From<GpioPin<C>>,
      _RESERVED: Peripheral<P: InputPin> + From<GpioPin<D>>,       // placeholder for INT detection (floating high; detect low pulls)
{   // changed via running './set-target.sh'
    (io.pins.gpio4.into(), io.pins.gpio5, Some(io.pins.gpio0), NO_PIN)      // esp32c3
    //(io.pins.gpio22, io.pins.gpio23, Some(io.pins.gpio21), NO_PIN)    // esp32c6
}
