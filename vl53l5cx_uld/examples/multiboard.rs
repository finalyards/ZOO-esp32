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
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, AnyOutput, /*AnyFlex,*/ Level},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

extern crate vl53l5cx_uld as uld;
mod common;
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

const DEFAULT_I2C_ADDR: u8 = VL53L5CX::FACTORY_I2C_ADDR;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    defmt_timestamps::init();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    /***
    #[allow(non_snake_case)]
    let (pinSDA, pinSCL, mut pinPWR_EN, pinsLPn)
        : (AnyFlex,AnyFlex,Option<AnyOutput>,&[AnyOutput]) = { (
            AnyFlex::new(io.pins.gpio4),
            AnyFlex::new(io.pins.gpio5),
            Some(AnyOutput::new(io.pins.gpio0, Level::Low)),
            &[AnyOutput::new(io.pins.gpio1, Level::Low), AnyOutput::new(io.pins.gpio2, Level::Low)]
        // esp32c3
    )};
    ***/

    // Problems with pin types:
    //  - 'I2C::new' cannot take 'AnyFlex' for the SDA/SCL pins
    //
    #[allow(non_snake_case)]
    let (pinSDA, pinSCL, mut pinPWR_EN, pinsLPn) = (
        io.pins.gpio4,
        io.pins.gpio5,
        Some(AnyOutput::new(io.pins.gpio0, Level::Low)),
        &mut [AnyOutput::new(io.pins.gpio1, Level::Low), AnyOutput::new(io.pins.gpio2, Level::Low)]
        // esp32c3
    );

    let i2c_bus = I2C::new_with_timeout(
        peripherals.I2C0,
        pinSDA,
        pinSCL,
        400.kHz(),
        &clocks,
        None
    );

    let d_provider = Delay::new(&clocks);
    let delay_ms = |ms| d_provider.delay_millis(ms);

    // Reset VL53L5CX by pulling down its power for a moment
    pinPWR_EN.iter_mut().for_each(|pin| {
        pin.set_low();
        delay_ms(50);      // tbd. how long is suitable, by the specs?
        pin.set_high();
        info!("Target powered off and on again.");
    });

    // Pick which board to operate on
    pinsLPn[0].set_low();
    pinsLPn[1].set_high();   // offline

    info!("Selecting board {}", 0_u8);

    let pl = MyPlatform::new(&clocks, i2c_bus, DEFAULT_I2C_ADDR);

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
