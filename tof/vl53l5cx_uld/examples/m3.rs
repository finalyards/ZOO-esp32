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
    gpio::{AnyPin, Input, InputConfig, Output, OutputConfig, Level, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    main,
    time::{now, RateExtU32}
};

const D_PROVIDER: Delay = Delay::new();

extern crate vl53l5cx_uld as uld;

include!("./pins_gen.in");  // pins!

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

#[main]
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

#[allow(non_snake_case)]
struct Pins<const BOARDS: usize>{
    SDA: AnyPin,
    SCL: AnyPin,
    PWR_EN: AnyPin,
    LPns: [AnyPin;BOARDS],
    INT: AnyPin
}

fn main2() -> Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let o_low_cfg = OutputConfig::default().with_level(Level::Low);
    let i_pull_none_cfg = InputConfig::default().with_pull(Pull::None);

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, LPns, INT } = pins!(peripherals);

    #[allow(non_snake_case)]
    let mut PWR_EN = Output::new(PWR_EN, o_low_cfg).unwrap();
    #[allow(non_snake_case)]
    let mut LPns = LPns.map(|n| { Output::new(n, o_low_cfg).unwrap() });
    #[allow(non_snake_case)]
    let INT = Input::new(INT, i_pull_none_cfg).unwrap();

    let pl = {
        let i2c_bus = I2c::new(peripherals.I2C0, I2cConfig::default())
            .unwrap()
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

    // Have only one board comms-enabled (the pins are initially low).
    LPns[0].set_high();

    let vl = VL53L5CX::new_with_ping(pl)?.init()?;

    info!("Init succeeded");

    //--- ranging loop
    //
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),HzU8::try_from(10.Hz()).unwrap() /*HzU8(10)*/))    // tbd.!!!!!!!
        .with_target_order(CLOSEST);
    debug!("A");
    let mut ring = vl.start_ranging(&c)
        .expect("to start ranging");
    debug!("B");
    // #BUG: DOES NOT REACH

    for round in 0..3 {
        let t0= now();

        // wait for 'INT' to fall
        loop {
            if INT.is_low() {
                debug!("INT after: {}ms", (now()-t0).to_micros() as f32 / 1000.0);
                break;
            } else if (now()-t0).to_millis() > 1000 {
                panic!("No INT detected");
            }
            delay_us(20);   // < 100us
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
*
* Note: If you use Embassy, a better way is to depend on 'embassy-sync' and enable its
*       "defmt-timestamp-uptime" feature.
*/
fn init_defmt() {
    use esp_hal::time::now;

    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
}
