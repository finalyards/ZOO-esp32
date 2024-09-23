/*
* Working with multiple boards
*
* We do this by using a 'flock' variant of the drivers.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::Io,
    i2c::I2C,
    prelude::*,
    time::now
};

const D_PROVIDER: Delay = Delay::new();

//extern crate vl53l5cx_flock as flock;
mod flock_lib;
use flock_lib as flock;

mod common;

include!("./pins.in");  // pins!

use common::MyPlatform;     // same as for single sensors ('uld')

extern crate vl53l5cx_uld as uld;
use uld::{
    Result,
    VL53L5CX,
    ranging::{
        RangingConfig,
        TargetOrder::CLOSEST,
        Mode::AUTONOMOUS,
    },
    units::*,
    API_REVISION,
};

use flock::{
    Flock
};

#[entry]
fn main() -> ! {
    init_defmt();

    match main2() {
        Err(e) => {
            panic!("Failed with ULD error code: {}", e);
        },

        Ok(()) => {
            info!("End of FLOCK demo");
            semihosting::process::exit(0);      // back to developer's command line
        }
    }
}

fn main2() -> Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    #[allow(non_snake_case)]
    let (SDA, SCL, PWR_EN, LPns) = pins!(io);

    let pl = {
        let i2c_bus = I2C::new(
            peripherals.I2C0,
            SDA,
            SCL,
            400.kHz()
        );
        MyPlatform::new(i2c_bus)
    };

    let delay_ms = |ms| D_PROVIDER.delay_millis(ms);

    // Reset VL53L5CX's by pulling down their power for a moment
    if let Some(mut pin) = PWR_EN {
        pin.set_low();
        delay_ms(20);      // tbd. how long is suitable, by the specs?
        pin.set_high();
        info!("Targets powered off and on again.");
    }

    let mut vls = Flock::new_maybe(pl, LPns)?.init()?;

    info!("Init succeeded, {} sensors, driver '{}'", LPns.len(), API_REVISION);

    //--- ranging loop
    //
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(Ms(5),Hz(10)))
        .with_target_order(CLOSEST);

    let mut ring = vls.start_ranging(&c)
        .expect("Failed to start ranging");

    for _round in 0..10 {
        while !ring.is_ready().unwrap() {   // poll; 'async' will allow sleep
            delay_ms(5);
        }

        let (res, sensor_id, time_stamp, temp_c) = ring.get_data()
            .expect("Failed to get data");

        info!("Data: #{}; {}; stamp={} ms", sensor_id, temp_c, time_stamp);

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

    // Rust automatically stops the ranging in the ULD C driver, when 'Ranging' is dropped.
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
