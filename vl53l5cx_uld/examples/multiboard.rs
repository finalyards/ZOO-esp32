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
    gpio::{Io, Level},
    i2c::{self, I2C, Instance},
    prelude::*,
    Blocking
};

#[cfg(feature="next_api")]
use esp_hal::{
    gpio::Output
};
#[cfg(not(feature="next_api"))]
use esp_hal::{
    clock::ClockControl,
    gpio::{AnyOutput},
    peripherals::Peripherals,
    system::SystemControl,
};
#[cfg(all())]
use esp_hal::{
    gpio::{PeripheralInput, PeripheralOutput, GpioPin},
    peripheral::Peripheral
};

#[cfg(feature="next_api")]
const D_PROVIDER: Delay = Delay::new();

extern crate vl53l5cx_uld as uld;
mod common;

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
    #[cfg(feature="next_api")]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    #[cfg(not(feature="next_api"))]
    let (peripherals, system, clocks);
    #[cfg(not(feature="next_api"))]
    {
        peripherals = Peripherals::take();
        system = SystemControl::new(peripherals.SYSTEM);
        clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    }

    common::init();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Problems with pin types:
    //  - 'I2C::new' cannot take 'AnyFlex' for the SDA/SCL pins
    //
    #[cfg(feature="next_api")]
    #[allow(non_snake_case)]
    #[cfg(not(all()))]    // C3
    let (pinSDA, pinSCL, mut pinPWR_EN, pinsLPn) = (
        io.pins.gpio4,
        io.pins.gpio5,
        Some(Output::new(io.pins.gpio0, Level::Low)),
        &mut [Output::new(io.pins.gpio1, Level::Low), Output::new(io.pins.gpio2, Level::Low)]
    );
    /* NOT because:
    *   <<
    *       error[E0562]: `impl Trait` is not allowed in the type of variable bindings
    *       [...]
    *       note: `impl Trait` is only allowed in arguments and return types of functions and methods
    *   <<
    *let (pinSDA, pinSCL, mut pinPWR_EN, mut pinsLPn)
    *    :(impl Peripheral<P: PeripheralOutput + PeripheralInput>,
    *      impl Peripheral<P: PeripheralOutput + PeripheralInput>,
    *      Option<Output>,
    *      &[Output]) = (
    *    io.pins.gpio18,
    *    io.pins.gpio19,
    *    Some(Output::new(io.pins.gpio20, Level::Low)),
    *    [Output::new(io.pins.gpio21, Level::Low), Output::new(io.pins.gpio22, Level::Low)]
    *); */
    let (pinSDA, pinSCL, mut pinPWR_EN, mut pinsLPn): (GpioPin<18>, GpioPin<19>, Option<Output>, [Output;2]) = (
        io.pins.gpio18,
        io.pins.gpio19,
        Some(Output::new(io.pins.gpio20, Level::Low)),
        [Output::new(io.pins.gpio21, Level::Low), Output::new(io.pins.gpio22, Level::Low)]
    );

    /*** WORKS***
    let (pinSDA, pinSCL, mut pinPWR_EN, mut pinsLPn) = (
        io.pins.gpio18,
        io.pins.gpio19,
        Some(Output::new(io.pins.gpio20, Level::Low)),
        [Output::new(io.pins.gpio21, Level::Low), Output::new(io.pins.gpio22, Level::Low)]
    ); ***/

    #[cfg(not(feature="next_api"))]
    #[allow(non_snake_case)]
    #[cfg(not(all))]    // C3
    let (pinSDA, pinSCL, mut pinPWR_EN, pinsLPn) = (
        io.pins.gpio4,
        io.pins.gpio5,
        Some(AnyOutput::new(io.pins.gpio0, Level::Low)),
        [AnyOutput::new(io.pins.gpio1, Level::Low), AnyOutput::new(io.pins.gpio2, Level::Low)]
    );

    #[cfg(feature="next_api")]
    let i2c_bus = I2C::new(
        peripherals.I2C0,
        pinSDA,
        pinSCL,
        400.kHz()
    );
    #[cfg(not(feature="next_api"))]
    let i2c_bus = I2C::new_with_timeout(
        peripherals.I2C0,
        pinSDA,
        pinSCL,
        400.kHz(),
        &clocks,
        None
    );

    #[cfg(feature="next_api")]
    let delay_ms = |ms| D_PROVIDER.delay_millis(ms);
    #[cfg(not(feature="next_api"))]
    let d_provider = Delay::new(&clocks);
    #[cfg(not(feature="next_api"))]
    let delay_ms = |ms| d_provider.delay_millis(ms);

    // Reset VL53L5CX's by pulling down their power for a moment
    pinPWR_EN.iter_mut().for_each(|pin| {
        pin.set_low();
        delay_ms(20);      // tbd. how long is suitable, by the specs?
        pin.set_high();
        info!("Target powered off and on again.");
    });

    // Disable all boards, by default
    pinsLPn.iter_mut().for_each(|p| p.set_low());
    pinsLPn[0].set_high();      // enable one

    #[cfg(feature="next_api")]
    let pl = MyPlatform::new(i2c_bus, DEFAULT_I2C_ADDR);
    #[cfg(not(feature="next_api"))]
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

#[cfg(not(all()))]
fn change_addr<'a,T>(i2c: &mut I2C<'a,T,Blocking>, old_addr: u8, new_addr: u8) -> Result<(),i2c::Error>
    where T: Instance
{
    let mut wr = |a: u8, index: u16, v: u8| -> Result<(),_> {
        let mut buf: [u8;3] = [0;3];
        buf[0..2].copy_from_slice(&index.to_be_bytes());
        buf[2] = v;

        i2c.write(a, &buf)
    };
    wr(old_addr, 0x7fff, 0x00)?;
    wr(old_addr, 0x4, new_addr >> 1)?;
    wr(new_addr, 0x7fff, 0x02)?;
    Ok(())
}
