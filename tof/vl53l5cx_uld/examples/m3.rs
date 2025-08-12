/*
* Just read some data from board with 'LPns[0]'.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn, panic};
use defmt_rtt as _;

use esp_backtrace as _;

#[cfg(not(feature = "esp-hal-0_22"))]
use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, Input, /*InputConfig,*/ Output, /*OutputConfig,*/ Level},
    i2c::master::{Config as I2cConfig, I2c},
    //main,
    time::{now, /*RateExtU32*/}
};
#[cfg(feature="esp-hal-next")]
use esp_hal::gpio::{InputConfig, OutputConfig};
#[cfg(not(feature="esp-hal-next"))]
use esp_hal::gpio::Pull;

#[cfg(not(feature="esp-hal-0_22"))]
use esp_hal::{
    main,
    time::RateExtU32
};
#[cfg(feature = "esp-hal-0_22")]
use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, Input, Output, Level},
    i2c::master::{Config as I2cConfig, I2c},
    entry as main,
    time::now
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

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, LPns, INT } = pins!(peripherals);

    #[allow(non_snake_case)]
    #[cfg(feature = "esp-hal-next")]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low, OutputConfig::default());
    #[allow(non_snake_case)]
    #[cfg(not(feature = "esp-hal-next"))]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low);

    #[allow(non_snake_case)]
    #[cfg(feature = "esp-hal-next")]
    let mut LPns = LPns.map(|n| { Output::new(n, Level::Low, OutputConfig::default()) });
    #[allow(non_snake_case)]
    #[cfg(not(feature = "esp-hal-next"))]
    let mut LPns = LPns.map(|n| { Output::new(n, Level::Low) });

    #[allow(non_snake_case)]
    #[cfg(feature = "esp-hal-next")]
    let INT = Input::new(INT, InputConfig::default() /*no pull*/);
    #[allow(non_snake_case)]
    #[cfg(not(feature = "esp-hal-next"))]
    let INT = Input::new(INT, Pull::None);

    let pl = {
        #[cfg(not(feature = "esp-hal-0_22"))]
        let i2c_bus = I2c::new(peripherals.I2C0, I2cConfig::default())
            .unwrap()
            .with_sda(SDA)
            .with_scl(SCL);
        #[cfg(feature = "esp-hal-0_22")]
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

    // Have only one board comms-enabled (the pins are initially low).
    LPns[0].set_high();

    let mut vl = VL53L5CX::new_with_ping(pl)?.init()?;

    info!("Init succeeded");

    // Extra test, to see basic comms work      // BUG: GETS STUCK
    {
        vl.i2c_no_op()
            .expect("to pass");
        info!("I2C no-op (get power mode) succeeded");
    }

    //--- ranging loop
    //
    #[cfg(not(feature = "esp-hal-0_22"))]
    let freq = HzU8::try_from( 10.Hz() ).unwrap();  // tbd. make so we could 'just' give '10.Hz' (if '#fugit' feature is given)
    #[cfg(feature = "esp-hal-0_22")]
    let freq = HzU8(10);

    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),freq /*HzU8::try_from( 10.Hz() ).unwrap()*/))    // tbd.!!!!!!!
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c)
        .expect("to start ranging");

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
