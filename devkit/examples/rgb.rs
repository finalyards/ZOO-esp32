/*
* Showcase setting the RGB LED.
*
* Based on:
*   - https://github.com/esp-rs/esp-hal-community/blob/main/esp-hal-smartled/examples/hello_rgb.rs
*/
#![no_std]
#![no_main]

use anyhow::Result;

#[allow(unused_imports)]
use defmt::{info, error};
use defmt_rtt as _;

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    //gpio::Output,
    prelude::*,
    rmt::Rmt,
};
use esp_hal_smartled::{
    smartLedBuffer,
    SmartLedsAdapter
};

use semihosting::process;
use smart_leds::{
    brightness,
    gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};

mod common;
use common::init_defmt;

include!("./pins_gen.in");  // pins!

#[entry]
fn main() -> ! {
    init_defmt();

    match main2() {
        Err(e) => panic!("Failed with: {:?}", e),
        Ok(()) => process::exit(0)      // back to developer's command line
    }
}

fn main2() -> Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    #[allow(non_snake_case)]
    #[allow(unused_parens)]
    let (LED,) = pins!(peripherals);

    #[allow(non_snake_case)]
    let RMT_FREQ = 80.MHz();
    //const RMT_FREQ: HertzU32 = 80.MHz();  // "esp32h2" would have it as '32.MHz()' - but we don't support H2 currently
        // cannot seem to give 'fugit' frequencies as 'const' (fugit 0.3.7)

    let rmt = Rmt::new(peripherals.RMT, RMT_FREQ)
        .unwrap();

    // "Uses one of the RMT channels to instantiate a `SmartLedsAdapter`; can be used with all `smart_led` implementations."
    //
    let rmt_buffer = smartLedBuffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, LED /*peripherals.GPIO8*/, rmt_buffer);

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data;

    loop {
        for hue in 0..=255 {
            color.hue = hue;

            // "Convert from the HSV color space (where we can easily transition from one
            // color to the other) to the RGB color space that we can then send to the LED."
            data = [hsv2rgb(color)];

            // "When sending to the LED, we do a gamma correction first (see 'smart_leds'
            // documentation) and finally limit the brightness to X out of 255."
            led.write( brightness(gamma(data.iter().cloned()), 5) )
                .unwrap();
            blocking_delay_ms(20);
        }
    }
}

const D_PROVIDER: Delay = Delay::new();
fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
