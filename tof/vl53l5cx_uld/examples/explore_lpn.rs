/*
* Explore:
*   - what LPn's really disable
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output},
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
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    #[allow(non_snake_case)]
    let (SDA, SCL, PWR_EN, mut LPns, INT): (_,_,_,[Output;2],_) = pins!(io);
    #[allow(non_snake_case)]

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
    let delay_us = |us| D_PROVIDER.delay_micros(us);

    // Reset VL53L5CX(s) by pulling down their power for a moment
    if let Some(mut pin) = PWR_EN {
        pin.set_low();
        delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        pin.set_high();
        info!("Target powered off and on again.");
    }

    // Leave only board #0 comms-enabled.
    LPns[0].set_high();

    let vl = VL53L5CX::ping_new(pl)?.init()?;

    info!("Init succeeded");

    //--- ranging loop
    //
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),HzU8(10)))
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c)
        .expect("to start ranging");

    for round in 0..3 {
        // Read the first round, then disable 'LPn[0]'
        LPns[0].set_low();
        debug!("Switched LPn off (low)");

        // wait for 'INT' to fall
        //
        // Note: '.wait_for_falling_edge()' needs async (Embassy); We need to be careful not to
        //      wait too long (or at all): the INT falls down for 100us and is AUTOMATICALLY RAISED
        //      by the VL.
        //
        debug!("Waiting for an INT");
        let t0= now();
        loop {
            let v= INT.get_level();
            if v == Level::Low {
                debug!("INT after: {}ms", (now()-t0).to_micros() as f32 / 1000.0);
                break;
            } else {
                delay_us(20);   // < 100us
            }
        }

        // First round is often partial; skip it
        if round==0 {
            debug!("Skipping round 0");
            continue;
        }

        // Re-enable 'LPn[0]' for reading the results
        LPns[0].set_low();
        debug!("Switched LPn ON (for reading the results)");

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

    // 'ring::Drop' will shut the ranging, but it needs the comms enable to be high.
    //
    LPns[0].set_high();

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
