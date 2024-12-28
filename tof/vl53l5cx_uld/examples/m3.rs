/*
* Just read some data from board with 'LPns[0]'.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;

use esp_hal::{
    delay::Delay,
    gpio::Level,
    prelude::*,
    time::now
};
use esp_hal::{
    i2c::master::{Config as I2cConfig, I2c},
};

const D_PROVIDER: Delay = Delay::new();

extern crate vl53l5cx_uld as uld;
mod common;

include!("./pins_gen.in");  // pins!

use common::MyPlatform;

use uld::{
    Result,
    VL53L5CX,
    RangingConfig,
    TargetOrder::CLOSEST,
    Mode::AUTONOMOUS,
    units::*,
};

#[entry]
fn main() -> ! {
    init_defmt();

    match main2() {
        Err(e) => {
            panic!("Failed with ULD error code: {}", e);
        },

        Ok(()) => {
            info!("End of ULD demo");
            semihosting::process::exit(0);      // back to developer's command line
        }
    }
}

fn main2() -> Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    #[allow(non_snake_case)]
    let (SDA, SCL, mut PWR_EN, mut LPns, INT) = pins!(peripherals);

    let pl = {
        let i2c_bus = I2c::new(peripherals.I2C0, I2cConfig::default())
            .with_sda(SDA)
            .with_scl(SCL);

        MyPlatform::new(i2c_bus)
    };

    let delay_ms = |ms| D_PROVIDER.delay_millis(ms);
    let delay_us = |us| D_PROVIDER.delay_micros(us);

    // Reset VL53L5CX(s) by pulling down their power for a moment
    {
        PWR_EN.set_low();
        delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        PWR_EN.set_high();
        info!("Target powered off and on again.");
    }

    // Leave only one board comms-enabled.
    LPns[0].set_high();

    let vl = VL53L5CX::new_with_ping(pl)?.init()?;

    info!("Init succeeded");

    //--- ranging loop
    //
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),HzU8(10)))
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c)
        .expect("to start ranging");

    for round in 0..3 {
        let t0= now();

        // wait for 'INT' to fall
        loop {
            let v= INT.level();
            if v == Level::Low {
                debug!("INT after: {}ms", (now()-t0).to_micros() as f32 / 1000.0);
                break;
            } else {
                delay_us(20);   // < 100us
            }
        }

        let (res, temp_degc) = ring.get_data()
            .expect("Failed to get data");

        info!("Data #{} ({})", round, temp_degc);

        #[cfg(feature = "target_status")]
        info!(".target_status:    {}", res.target_status);
        #[cfg(feature = "nb_targets_detected")]
        info!(".targets_detected: {}", res.targets_detected);

        #[cfg(feature = "ambient_per_spad")]
        info!(".ambient_per_spad: {}", res.ambient_per_spad);
        #[cfg(feature = "nb_spads_enabled")]
        info!(".spads_enabled:    {}", res.spads_enabled);
        #[cfg(feature = "signal_per_spad")]
        info!(".signal_per_spad:  {}", res.signal_per_spad);
        #[cfg(feature = "range_sigma_mm")]
        info!(".range_sigma_mm:   {}", res.range_sigma_mm);
        #[cfg(feature = "distance_mm")]
        info!(".distance_mm:      {}", res.distance_mm);
        #[cfg(feature = "reflectance_percent")]
        info!(".reflectance:      {}", res.reflectance);
    }

    Ok(())
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
*
* Note: 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
*       read operation". Out 'esp_hal::time::now' isn't, but sure seems to work.
*
* Reference:
*   - defmt book > ... > Hardware timestamp
*       -> https://defmt.ferrous-systems.com/timestamps#hardware-timestamp
*/
fn init_defmt() {
    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
}
