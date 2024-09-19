/*
* Working with multiple boards
*
* - initializing their I2C addresses, so all boards can be used on the same bus
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
};

const D_PROVIDER: Delay = Delay::new();

extern crate vl53l5cx_uld as uld;
mod common;

include!("./pins.in");  // pins!

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

const DEFAULT_I2C_ADDR: u8 = VL53L5CX::FACTORY_I2C_ADDR;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    common::init();

    // tbd. This will get into 'pins.in|rs' and be a macro.

    let (pinSDA, pinSCL, mut pinPWR_EN, mut pinsLPn) = pins!(io);

    let i2c_bus = I2C::new(
        peripherals.I2C0,
        pinSDA,
        pinSCL,
        400.kHz()
    );

    let delay_ms = |ms| D_PROVIDER.delay_millis(ms);

    // Reset VL53L5CX's by pulling down their power for a moment
    pinPWR_EN.iter_mut().for_each(|pin| {
        pin.set_low();
        delay_ms(50);      // tbd. how long is suitable, by the specs?
        pin.set_high();
        info!("Target powered off and on again.");
    });

    // Disable all boards, but [0]
    pinsLPn.iter_mut().for_each(|p| p.set_low());
    match pinsLPn.first_mut() {
        Some(p0) => { p0.set_high(); },      // enable one
        _ => ()          // only one board and its LPn is already pulled up
    }

    let pl = MyPlatform::new(i2c_bus, DEFAULT_I2C_ADDR);

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

    // With 'semihosting' feature enabled, execution can return back to the developer's command line.
    semihosting::process::exit(0);
}
