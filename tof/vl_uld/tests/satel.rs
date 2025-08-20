/*
* Common tools for the tests.
*
* Note: These are derivatives of what's available in the 'examples/'.
*/
#![no_std]
#![no_main]

use defmt::{info, debug, error, warn, panic};
use defmt_rtt as _;     // we do it, tests don't need to

use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, Input, InputConfig, Output, OutputConfig, Level},
    i2c::master::{Config as I2cConfig, I2c},
    time::{now, Rate}
};

extern crate vl53l5cx_uld as uld;

// Sneak in the platform implementation from 'examples'
#[path="../examples/common.rs"]
mod common;
use common::MyPlatform;

use uld::{
    Result,
    VL53L5CX,
    RangingConfig,
    TargetOrder::CLOSEST,
    Mode::AUTONOMOUS,
    units::*,
};

#[allow(non_snake_case)]
struct Pins<const BOARDS: usize>{
    SDA: AnyPin,
    SCL: AnyPin,
    PWR_EN: AnyPin,
    LPns: [AnyPin;BOARDS],
    INT: AnyPin
}

#[allow(non_snake_case)]
pub struct SATEL {
    pl: MyPlatform,
    PWR_EN: AnyPin
}

impl SATEL {
    pub fn new<const _B: usize>(pins: &mut Pins<_B>, peripherals: Peripherals) -> Self {
        #[allow(non_snake_case)]
        let Pins{ SDA, SCL, PWR_EN, LPns, INT } = pins!(peripherals);

        #[allow(non_snake_case)]
        let PWR_EN = Output::new(PWR_EN, Level::Low, OutputConfig::default());
        #[allow(non_snake_case)]
        let mut LPns = LPns.map(|n| { Output::new(n, Level::Low, OutputConfig::default()) });
        #[allow(non_snake_case)]
        let _INT = Input::new(INT, InputConfig::default() /*no pull*/);

        let pl = {
            let i2c_bus = I2c::new(peripherals.I2C0, I2cConfig::default()
                .with_frequency(1000.kHz())
            )
                .unwrap()
                .with_sda(SDA)
                .with_scl(SCL);

            MyPlatform::new(i2c_bus)
        };

        // Have only one board comms-enabled (the pins are initially low).
        LPns[0].set_high();

        let self= Self {
            pl,
            PWR_EN
        };
        self.reset();
        self
    }

    pub fn reset(&mut self) {
        self.PWR_EN.set_low();
        blocking_delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        self.PWR_EN.set_high();
        info!("Target powered off and on again.");
    }
}

const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}

pub fn init_defmt() {
    use esp_hal::time::now;

    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
}
