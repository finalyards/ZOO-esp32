/*
* Steering of the RGB LED, as found on the dev kits:
*   - ESP32-C3-DevKitC-02
*   - ESP32-C6-DevKitM-1
*
* The code is based on the 'esp-hal/examples/async/embassy_rmt_tx' code:
*   -> https://github.com/esp-rs/esp-hal/tree/main/examples/async/embassy_rmt_tx
*/
#[allow(unused_imports)]
#[cfg(feature = "defmt")]
use defmt::debug;

use core::mem::MaybeUninit;
use esp_hal::{
    gpio::Level,
    rmt::{Channel, PulseCode, Tx /*, TxChannelConfig*/},
    Async
};
use palette::{
    Hsv,
    IntoColor,
    Srgb,
};

pub struct RgbLed<'a> {
    ch: Channel<'a, Async, Tx>,
    brightness: Brightness
}

impl<'a> RgbLed<'a> {
    // The caller should provide the RMT channel with such configuration:
    //  ```rust
    //    TxChannelConfig::default()
    //        .with_clk_divider(4)    // 20MHz
    //        .with_idle_output_level(Level::Low)
    //        .with_carrier_modulation(false)
    //        .with_idle_output(true)
    //  ```
    pub fn new_with_channel(ch: Channel<'a, Async, Tx>, brightness: f32) -> Self {

        RgbLed{
            ch,
            brightness: brightness.into()
        }
    }

    pub async fn set(&mut self, color: &Srgb<u8>) {
        let mut buf: [PulseCode; 25] = {
            let un = MaybeUninit::uninit();
            unsafe { un.assume_init() }
        };

        let v: Srgb<u8> = self.brightness.apply(color).into_format();

        Self::to_pulses(&v, &mut buf);

        // Playing safe: the LED wants a long LOW (>80us for C3 devkit; >50us for C6 devkit) to place the color into effect.
        #[cfg(false)]
        {
            // 1600 = 80us; 1000 = 50us
            buf[24] = PulseCode::new(Level::Low, 1000, Level::Low, 0);
        }
        // Playing rushed: we'll leave the signal LOW (and wish no new write within the 80us occurs).
        #[cfg(true)]
        { buf[24] = PulseCode::end_marker(); }

        self.ch.transmit(&buf).await .unwrap();
    }

    // Place the RGB (as GRB, bits 7..0) into 'buf'
    //
    // Future note: With generators, it would be relatively easy to make this return an 'Iterator' (not needing to fill a mut slice).
    //
    fn to_pulses(color: &Srgb<u8>, out: &mut [PulseCode]) {
        let mut counter= 0;

        const PULSE_ONE: PulseCode = PulseCode::new(Level::High, 18, Level::Low, 7);
        const PULSE_ZRO: PulseCode = PulseCode::new(Level::High, 7, Level::Low, 18);

        for bits in [color.green, color.red, color.blue] {  // GRB
            for i in (0..=7).rev() {
                out[counter] = match bits >> i & 0x01 {
                    1 => PULSE_ONE,
                    0 => PULSE_ZRO,
                    _ => unreachable!(),
                };
                counter +=1;
            }
        }
    }
}

pub struct Brightness {
    f: f32  // 0.0..1.0
}

impl Brightness {
    fn new(f: f32) -> Self {
        assert!( f == f.clamp(0.0, 1.0), "Brightness not within range: 0.0 .. 1.0: {}", f);
        Self{f}
    }

    // Apply the brightness
    fn apply(&self, color: &Srgb<u8>) -> Srgb<u8> {
        let mut hsv: Hsv = (*color).into_format::<f32>().into_color();
        hsv.value *= self.f;        // no '.clamp' should be needed: f is within 0.0 .. 1.0

        //didn't build: hsv.into_color().into_format()
        <Hsv as IntoColor<Srgb>>::into_color(hsv).into_format()
    }
}

impl From<f32> for Brightness {
    fn from(f: f32) -> Self {
        Self::new(f)
    }
}
