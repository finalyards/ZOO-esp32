/*
* Just read some data from a Satel board.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn, panic};

#[cfg(feature = "run_with_espflash")]
use esp_println as _;
#[cfg(feature = "run_with_probe_rs")]
use defmt_rtt as _;

use esp_backtrace as _;

use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, Input, /*InputConfig,*/ Output, /*OutputConfig,*/ Level},
    i2c::master::{Config as I2cConfig, I2c},
    //main,
    //time::{Instant::now, RateExtU32}
};
#[cfg(not(feature="esp-hal-0_23"))]
use esp_hal::{
    gpio::{InputConfig, OutputConfig},
    time::{Instant, Rate}
};
#[cfg(feature="esp-hal-0_23")]
use esp_hal::{
    gpio::Pull
};

#[cfg(feature="esp-hal-0_23")]
use esp_hal::{
    main,
};
#[cfg(feature="esp-hal-0_23")]
use esp_hal::{
    time::RateExtU32,
};

extern crate vl53l5cx_uld as uld;

include!("./pins_gen.in");  // pins!

mod common;
use common::MyPlatform;

#[cfg(feature="esp-hal-0_23")]
mod instant_temp;
#[cfg(feature="esp-hal-0_23")]
use instant_temp::Instant;

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
struct Pins {
    SDA: AnyPin,
    SCL: AnyPin,
    PWR_EN: AnyPin,
    INT: AnyPin
}

#[allow(non_upper_case_globals)]
#[cfg(not(feature="esp-hal-0_23"))]
const I2C_SPEED: Rate = Rate::from_khz(400);        // use max 400

fn main2() -> Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, INT } = pins!(peripherals);

    #[allow(non_snake_case)]
    #[cfg(not(feature = "esp-hal-0_23"))]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low, OutputConfig::default());
    #[allow(non_snake_case)]
    #[cfg(feature = "esp-hal-0_23")]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low);

    #[allow(non_snake_case)]
    #[cfg(not(feature = "esp-hal-0_23"))]
    let INT = Input::new(INT, InputConfig::default());  // no pull
    #[allow(non_snake_case)]
    #[cfg(feature = "esp-hal-0_23")]
    let INT = Input::new(INT, Pull::None);

    #[allow(non_snake_case)]
    #[cfg(feature="esp-hal-0_23")]
    let I2C_SPEED = {       // could not be 'const'
        400_u32.kHz()  // max 400
    };

    let pl = {
        let x = I2c::new(peripherals.I2C0, I2cConfig::default()
            .with_frequency(I2C_SPEED)
        ).unwrap();

        let i2c_bus = x
            .with_sda(SDA)
            .with_scl(SCL);

        MyPlatform::new(i2c_bus)
    };

    // Reset VL53L5CX(s) by pulling down their power for a moment
    {
        PWR_EN.set_low();
        blocking_delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        PWR_EN.set_high();
        info!("Target powered off and on again.");
    }

    let mut vl = VL53L5CX::new_with_ping(pl)?.init()?;

    info!("Init succeeded");

    //#[cfg(not(all()))]
    // Extra test, to see basic comms work
    {
        blocking_delay_ms(1500);

        vl.i2c_no_op()
            .expect("to pass");
        info!("I2C no-op (get power mode) succeeded");
    }
    panic!();

    //--- ranging loop
    //
    let freq = HzU8(10);    // tbd. later, have this as 'Rate::from_hz(10)'

    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),freq))
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c)
        .expect("to start ranging");

    for round in 0..3 {
        let t0= Instant::now();

        // wait for 'INT' to fall
        loop {
            if INT.is_low() {
                debug!("INT after: {}", t0.elapsed());
                break;
            } else if t0.elapsed().as_millis() > 1000 {
                panic!("No INT detected");
            }
            blocking_delay_us(20);   // < 100us
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

const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) { D_PROVIDER.delay_millis(ms); }
fn blocking_delay_us(us: u32) { D_PROVIDER.delay_micros(us); }

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
    #[cfg(not(feature="esp-hal-0_23"))]
    use esp_hal::time::Instant;
    #[cfg(feature="esp-hal-0_23")]
    use esp_hal::time::now;

    defmt::timestamp!("{=u64:us}", {
        #[cfg(not(feature="esp-hal-0_23"))]
        {
            let now = Instant::now();
            now.duration_since_epoch().as_micros()
        }
        #[cfg(feature="esp-hal-0_23")]
        now().duration_since_epoch().to_micros()
    });
}
