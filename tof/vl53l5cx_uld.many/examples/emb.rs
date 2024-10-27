/*
* Reading two (or more) boards, using Embassy for multitasking.
*/
#![no_std]
#![no_main]

#![allow(for_loops_over_fallibles)]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use {defmt_rtt as _, esp_backtrace as _};

use embassy_executor::Spawner;
use embassy_time::{Duration as EmbDuration, Timer};

use esp_hal::{
    gpio::{Io, Input},
    i2c::I2c,
    prelude::*,
    time::{now, Instant, Duration},
    timer::timg::TimerGroup,
};

extern crate vl53l5cx_uld as uld;
mod common;

include!("./pins_gen.in");  // pins!

use common::MyPlatform;

use uld::{
    VL53L5CX,
    ranging::{
        RangingConfig,
        TargetOrder::CLOSEST,
        Mode::AUTONOMOUS,
    },
    units::*,
    API_REVISION,
    VL53L5CX_InAction
};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    init_defmt();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    #[allow(non_snake_case)]
    let (SDA, SCL, INT, mut LPns) = pins!(io);

    let pl = {
        let i2c_bus = I2c::new(
            peripherals.I2C0,
            SDA,
            SCL,
            400.kHz()
        );
        MyPlatform::new(i2c_bus)
    };

    // Initially, enable just one of the boards; leave others disabled ('snippets/pins.in' defaults them to Low)
    LPns[0].set_high();

    let vl = VL53L5CX::new_maybe(pl).unwrap()
        .init().unwrap();

    info!("Init succeeded, driver version {}", API_REVISION);

    spawner.spawn(ranging(vl)).unwrap();
    spawner.spawn(track_INT(INT)).unwrap();
}


// Initially, have two tasks:
//  1. runs the TOF sensor
//  2. sees whether the 'INT' pin gets high->low edges, and logs them

#[embassy_executor::task]
async fn ranging(/*move*/ mut vl: VL53L5CX_InAction) {

    //--- ranging loop
    //
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(Ms(5),Hz(10)))
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c)
        .expect("Failed to start ranging");

    let mut last_t1: Option<Instant> = None;

    for round in 0..10 {
        let t0 = now();
        while !ring.is_ready().unwrap() {
            Timer::after(EmbDuration::from_millis(1)).await;
        }
        let t1 = now();

        let (res, temp_degc) = ring.get_data()
            .expect("Failed to get data");

        let t2 = now();

        info!("Data #{} ({})", round, temp_degc);

        info!(".target_status:    {}", res.target_status);
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

        let t3 = now();

        debug!("Timing [ms] (total {=f32}): poll {}, read {}, output {}", ms(t3-t0), ms(t1-t0), ms(t2-t1), ms(t3-t2));
        for last_t1 in last_t1 {
            debug!("Ranging cycle [ms]: {}", ms(t1-last_t1));  // ready-to-ready
        }
        last_t1 = Some(t1);
    }

    // Rust automatically stops the ranging in the ULD C driver, when 'Ranging' is dropped.
}

// Note: 'Duration' doesn't allow passing by reference, unless we scramble it like 'ms(&(t3-t0))' in the call. We're fine, gulp.
//
fn ms(dur: /*&*/Duration) -> f32 {
    dur.to_micros() as f32 / 1000.0
}

#[embassy_executor::task]
#[allow(non_snake_case)]
async fn track_INT(mut pin: Input<'static>) {

    loop {
        pin.wait_for_rising_edge().await;
        debug!("INT detected");
    }
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
