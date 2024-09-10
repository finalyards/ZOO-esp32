/*
* Based on vendor 'Example_1_ranging_basic.c'
*
* Initializes the ULD and starts a ranging to capture 10 frames, with 4x4 resolution.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

extern crate vl53l5cx_uld as uld;
mod common;

use common::MyPlatform;
use uld::{VL53L5CX, Ranging, RangingConfig};

// Vendor ULD C example:
// "we also suppose that the number of target per zone is set to 1, and all output are enabled."
//
// Note: 'Cargo.toml' may use 'required_features' to make sure we'd not get build with a bad combo.
//      This one is just a 2nd tier check.
//
#[cfg(not(feature = "targets_per_zone_1"))]
panic!("Cancel the build!");    // won't compile

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let i2c_bus = I2C::new(
        peripherals.I2C0,
        io.pins.gpio4,  // SDA
        io.pins.gpio5,  // SCL
        400.kHz(),
        &clocks,
        None        // option: interrupt handler
    );

    let pl = MyPlatform::new(&clocks, i2c_bus);

    let mut vl = VL53L5CX::new_and_init(pl).unwrap();

    info!("Init succeeded, driver version {}", vl.API_REVISION);

    let d_provider = Delay::new(&clocks);
    let delay_ms = |ms| d_provider.delay_millis(ms);

    //--- ranging loop
    //
    let mut ring: Ranging = vl.start_ranging( &RangingConfig::default() )
        .expect("Failed to start ranging");

    for round in 0..10 {
        // Using polling. Embassy will provide means to do this '.async'.

        while !ring.is_ready().unwrap() {   // poll; 'async' will allow sleep
            delay_ms(5);
        }

        let res = ring.get_data()
            .expect("Failed to get data");

        // 4x4 (default) = 16 zones
        info!("Data #{}", round);

        #[cfg(feature = "target_status")]
        info!(".target_status: {=[u8]}", res.target_status);

        #[cfg(feature = "distance_mm")]
        info!(".distance_mm:   {}", res.distance_mm);   // "{=[i16]}" cannot be used as a display hint #defmt
    }

    // Not really needed; Rust would stop it automatically
    //ring.drop();

    info!("End of ULD demo");

    // 'defmt' has had something like 'exit()' for tests, but doesn't seem to (0.3.8) have any more.
    // What we would like here is for 'probe-rs run' to exit to the command line.
    //exit();
    loop { delay_ms(999) }
}
