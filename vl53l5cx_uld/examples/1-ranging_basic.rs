/*
* Based on vendor 'Example_1_ranging_basic.c'
*
* Initializes the ULD and starts a ranging to capture 10 frames, with:
*   - Resolution 4x4
*   - Ranging period 1Hz
*/
extern crate vl53l5cx_uld as uld;

// "we also suppose that the number of target per zone is set to 1, and all output
// are enabled."
//
assert_cfg::all!( feature=targets_per_zone_1 );     // tbd. a way to do this without a crate? #help

mod common;

use common::MyPlatform;
use core::ffi::CStr;

use uld::{Platform, VL53L5CX};

use esp_println::logger::init_logger_from_env;
use defmt::{info, debug};

fn main() {
    init_logger_from_env();     // reads 'ESP_LOGLEVEL' env.var.

    // Following vendor example
    //
    let pl = MyPlatform::new();

    let dev = VL53L5CX::new(pl)
        .expect("Could not initialize the sensor");

    info!("Init succeeded, ULD version {}", dev.API_REVISION);

    //--- ranging loop
    //
    dev.start_ranging()?;

    for 0..10 {
        // Using polling. Embassy will provide means to do this '.async'.

        dev.check_data_ready()?;
        // tbd.

        let res = dev.get_ranging_data()?;

        // 4x4 (default) = 16 zones
        // "Only the data of the first zone are printed"
        // (what does that mean? From [vendor] example's comment... :§).

        info!("Data #{}", dev.streamcount);

        for i in 0..16 {

            info!("Zone: {}, Status: {}, Distance: {}mm",
                i,
                res.target_status[ VL53L5CX_NB_TARGET_PER_ZONE*i ],
                res.distance_mm[ VL53L5CX_NB_TARGET_PER_ZONE*i ]
            );
        }
        info!("");

        // Wait a few ms to avoid too high polling
        pl.delay_ms(5);
    }

    dev.stop_ranging()?;

    info!("End of ULD demo");
}
