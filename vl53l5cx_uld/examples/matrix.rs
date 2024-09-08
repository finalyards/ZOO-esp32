/*
* Initializes the ULD, sets some parameters and starts a ranging to capture 10 frames, with custom:
*   - resolution
*   - frequency
*   - target order
*
* References:
*   - embedded_hal::i2c documentation
*       -> https://docs.rs/embedded-hal/latest/embedded_hal/i2c/index.html
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
    gpio::{self, Io, AnyOutput, Level},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

extern crate vl53l5cx_uld as uld;
mod common;
//mod pins;
mod defmt_timestamps;

use common::MyPlatform;
use uld::{
    VL53L5CX,
    ranging::{
        RangingConfig,
        TargetOrder::CLOSEST,
        Mode::AUTONOMOUS,
    },
    units::*
};

// Vendor ULD C example:
// "we also suppose that the number of target[s] per zone is set to 1, and all output[s] are enabled."
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

    defmt_timestamps::init();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    #[allow(non_snake_case)]
    let (pinSDA, pinSCL, pinPWR_EN, _pinI2C_RST) = {
        // changed via running './set-target.sh'
        (io.pins.gpio4, io.pins.gpio5, Some(io.pins.gpio0), gpio::NO_PIN)      // esp32c3
        //(io.pins.gpio22, io.pins.gpio23, Some(io.pins.gpio21), gpio::NO_PIN)    // esp32c6
    };
    /*** tbd. replace above with:   #help
    let (pinSDA, pinSCL, pinPWR_EN, pinI2C_RST) = pins::get_pins(&io);
    ***/

    let i2c_bus = I2C::new_with_timeout(
        peripherals.I2C0,
        pinSDA,
        pinSCL,
        400.kHz(),
        &clocks,
        None,   // 'esp-hal' documentation on what exactly the 'timeout' parameter steers is hazy. // author, Sep'24; esp-hal 0.20.1
    );

    let mut pwr_en = pinPWR_EN.map(|pin| AnyOutput::new(pin, Level::Low));       // None if you pull it up to IOVDD via a resistor (47K)

    let d_provider = Delay::new(&clocks);
    let delay_ms = |ms| d_provider.delay_millis(ms);

    let pl = MyPlatform::new(&clocks, i2c_bus);

    // Reset VL53L5CX by pulling down its power for a moment
    pwr_en.iter_mut().for_each(|pin| {
        pin.set_low();
        delay_ms(50);      // tbd. how long is suitable, by the specs?
        pin.set_high();
        info!("Target powered off and on again.");
    });

    let mut vl = VL53L5CX::new_and_init(pl)
        .unwrap();

    info!("Init succeeded, driver version {}", vl.API_REVISION);

    //--- ranging loop
    //
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(Ms(5),Hz(10)))
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c)
        .expect("Failed to start ranging");

    for round in 0..10 {
        while !ring.is_ready().unwrap() {   // poll; 'async' will allow sleep
            delay_ms(5);
        }

        let res = ring.get_data()
            .expect("Failed to get data");

        // 4x4 (default) = 16 zones
        info!("Data #{} (sensor {}°C)", round, res.silicon_temp_degc);

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

    info!("End of ULD demo");

    // If 'defmt' (or something) had a way for us to exit - so that 'probe-rs' would return control
    // to the command line - this is the place to do it. :) #wish
    loop {}
}
