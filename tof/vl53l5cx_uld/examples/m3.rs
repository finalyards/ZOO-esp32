/*
* Reading a single board, polling.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level},
    i2c::I2c,
    prelude::*,
    time::now
};

const D_PROVIDER: Delay = Delay::new();

extern crate vl53l5cx_uld as uld;
mod common;

include!("./pins_gen.in");  // pins!

use common::MyPlatform;

use uld::{
    Result,
    VL53L5CX,
    state_ranging::{
        RangingConfig,
        TargetOrder::CLOSEST,
        Mode::AUTONOMOUS,
    },
    units::*,
    API_REVISION,
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
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    #[allow(non_snake_case)]
    let (SDA, SCL, PWR_EN, INT) = pins!(io);

    let pl = {
        let i2c_bus = I2c::new(
            peripherals.I2C0,
            SDA,
            SCL,
            400.kHz()
        );
        MyPlatform::new(i2c_bus)
    };

    let delay_ms = |ms| D_PROVIDER.delay_millis(ms);

    // Reset VL53L5CX by pulling down their power for a moment
    if let Some(mut pin) = PWR_EN {
        pin.set_low();
        delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        pin.set_high();
        info!("Target powered off and on again.");
    }

    let mut vl = VL53L5CX::new_maybe(pl)?.init()?;

    info!("Init succeeded, driver version {}", API_REVISION);

    //--- ranging loop
    //
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),10.Hz()))
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c)
        .expect("Failed to start ranging");

    /*** #Rust; does not compile: 'expected `dyn Fn`, found closure'
    let wait_till_ready: dyn Fn() -> () = match INT {
        None => || {    // poll
            while !ring.is_ready().unwrap() {
                delay_ms(5);
            }
        },
        Some(INT_PIN) => || {   // wait for 'INT' to go down
            loop {
                let v= INT_PIN.get_level();
                debug!("INT: {}", v);
                if v == Level::Low { break; }
                delay_ms(1);
            }
        }
    };***/

    for round in 0..10 {
        let t0= now();

        //wait_till_ready(&mut ring);
        match INT {
            None => {    // poll
                while !ring.is_ready().unwrap() {
                    delay_ms(5);
                }
                debug!("Data after: {}ms", (now()-t0).to_micros() as f32 / 1000.0);
            },
            Some(ref INT_PIN) => {   // wait for 'INT' to go down
                loop {
                    let v= INT_PIN.get_level();
                    //debug!("INT: {}", v);
                    if v == Level::Low {
                        debug!("INT after: {}ms", (now()-t0).to_micros() as f32 / 1000.0);
                        break;
                    }
                    delay_ms(1);
                }
            }
        };

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
