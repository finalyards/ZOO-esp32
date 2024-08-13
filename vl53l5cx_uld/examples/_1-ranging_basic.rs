/*
* Based on vendor 'Example_1_ranging_basic.c'
*
* Initializes the ULD and starts a ranging to capture 10 frames, with:
*   - Resolution 4x4
*   - Ranging period 1Hz
*/
#![no_std]
#![no_main]

// "we also suppose that the number of target per zone is set to 1, and all output
// are enabled."
//
#[cfg(not(feature = "targets_per_zone_1"))]
fail "This example is said to rely on 'targets_per_zone_1' feature"

//assert_cfg::all!( feature=targets_per_zone_1 );     // tbd. a way to do this without a crate? #help

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
use uld::{Platform, VL53L5CX, Ranging, RangingConfig};

const VL53L5CX_NB_TARGET_PER_ZONE: u8 = 1;

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

    let dev = VL53L5CX::new(pl)
        .expect("Could not initialize the sensor");

    info!("Init succeeded, driver version {}", dev.API_REVISION);

    //--- ranging loop
    //
    let ring: Ranging = dev.start_ranging( RangingConfig::default() )?;

    for 0..10 in {
        // Using polling. Embassy will provide means to do this '.async'.

        while !ring.check_data_ready()? {
            // wait a bit, between polls
            delay_ms(5);
        }

        let res = ring.get_ranging_data()?;

        // 4x4 (default) = 16 zones
        // "Only the data of the first zone are printed" (what does that mean? From [vendor] example's comment... :/

        info!("Data #{}", dev.get_stream_count());

        for i in 0..16 {
            info!("Zone: {}, Status: {}, Distance: {}mm",
                i,
                res.target_status[ VL53L5CX_NB_TARGET_PER_ZONE*i ],
                res.distance_mm[ VL53L5CX_NB_TARGET_PER_ZONE*i ]
            );
        }
        info!("");
    };

    // Not really needed; Rust will '.drop()' it
    //dev.stop_ranging()?;

    // tbd. In Rust (and 'probe-rs'), can we end a main?
    info!("End of ULD demo");
}
