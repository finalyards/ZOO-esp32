/*
* Based on vendor 'Example_1_ranging_basic.c'
*
* Initializes the ULD and starts a ranging to capture 10 frames, with:
*   - Resolution 4x4
*   - Ranging period 1Hz
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

extern crate vl53l5cx_uld as uld;
mod common;

use common::MyPlatform;
use uld::{VL53L5CX, Ranging, RangingConfig};

// "we also suppose that the number of target per zone is set to 1, and all output are enabled."
//
// Note: seems the example gets to know the features used in compilation of actual library.
//      This is good.
//
// Note: 'Cargo.toml' may use 'required_features' to make sure we'd not get build with a bad combo.
//      This one is just a 2nd tier checkout point.
//
#[cfg(not(feature = "targets_per_zone_1"))]
panic!("Cancel the build!");    // won't compile

const TARGETS_PER_ZONE: u8 = 1;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let d_provider = Delay::new(&clocks);
    let delay_ms: _ = {     // synchronous (busy wait) delays
        |ms| { d_provider.delay_millis(ms); }
    };

    let pl = MyPlatform::new(&clocks, 0x52);     // default I2C address

    let mut dev = VL53L5CX::new_maybe(pl)
        .expect("Could not initialize the sensor");

    info!("Init succeeded, driver version {}", dev.API_REVISION);

    //--- ranging loop
    //
    let mut ring: Ranging = dev.start_ranging( &RangingConfig::default() )
        .expect("Failed to start ranging");

    for round in 0..10 {
        // Using polling. Embassy will provide means to do this '.async'.

        while !ring.is_ready().unwrap() {   // poll; 'async' will allow sleep
            delay_ms(5);
        }

        let res = ring.get_data()
            .expect("Failed to get data");

        // 4x4 (default) = 16 zones
        // "Only the data of the first zone are printed" (what does that mean? From [vendor] example's comment... :/

        info!("Data #{}", round);

        info!(".target_status: {=[u8]}", res.target_status);
        info!(".distance_mm:   {}", res.distance_mm);   // "{=[i16}" not recognized as a display hint #defmt

        /***
        for i in 0..16_usize {
            info!("Zone: {}, Status: {=&[u8]}, Distance: {=&[u8]}mm",
                i,
                res.target_status,  //R [/_*TARGETS_PER_ZONE**_/ i as usize],
                res.distance_mm     //R [/_*TARGETS_PER_ZONE**_/ i as usize]
            );
        }***/
    }

    // Not really needed; Rust will '.drop()' it
    //dev.stop_ranging()?;

    // tbd. In Rust (and 'probe-rs'), can we end a main?
    info!("End of ULD demo");

    loop {
        delay_ms(100_000);
    }
}
