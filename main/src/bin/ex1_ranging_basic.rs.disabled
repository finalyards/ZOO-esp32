/*
* Formulated by vendor sample 'Example 1'
* Rust side inspired by -> https://github.com/jessebraham/esp-hal-template/blob/main/embassy/src/bin/firmware.rs
*/
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::Io,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::{timg::TimerGroup, ErasedTimer, OneShotTimer},
};
use static_cell::make_static;

//R use lis3dh_async::{Lis3dh, Range, SlaveAddr};

use vl53l5cx::{VL53L5CX, Result};

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initialize the SYSTIMER peripheral, and then Embassy
    //
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timers = [OneShotTimer::new(timg0.timer0.into())];
    let timers = make_static!(timers);
    esp_hal_embassy::init(&clocks, timers);
    info!("Embassy initialized");

    // Prepare I2C use
    //
    // Inspiration taken from -> https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/embassy_i2c.rs
    //
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c0 = I2C::new_async(
        peripherals.I2C0,
        io.pins.gpio4,
        io.pins.gpio5,
        400.kHz(),
        &clocks,
    );

    /*** (keep as pattern of use)
    let mut lis3dh = Lis3dh::new_i2c(i2c0, SlaveAddr::Alternate).await.unwrap();
    lis3dh.set_range(Range::G8).await.unwrap();

    loop {
        let norm = lis3dh.accel_norm().await.unwrap();
        esp_println::println!("X: {:+.5}  Y: {:+.5}  Z: {:+.5}", norm.x, norm.y, norm.z);

        Timer::after(Duration::from_millis(100)).await;
    }
    ***/

    // tbd. provide the pin (or lack thereof) where INT is routed
    // - provide I2C address (unless default); and other config
    //
    let dev: VL53L5CX = VL53L5CX::new(i2c0, 0x12_u8);

    example1(dev).await
}

async fn example1(dev: VL53L5CX) -> Result {

    assert!( dev.is_alive(), "Device not detected at address {}", addr);

    let ver = dev.get_info().API_REVISION;
    info!("ULD API revision: {}", ver);

    let h = dev.start_ranging();
    for _count in 0..10 {

        // If INT connected
        let data = h.get_data() .await;

        info!("Data no: {}", unimplemented!());      // 'Dev.streamcount' in C

        // We only print one (strongest/closest, based on config) target, per zone.
        for i in 0..16 {
            let status = data.target_status[i][0];
            let dist = data.distance_mm[i][0];
            info!("Zone: {}, Status: {}, Distance: {}mm", i, status, dist )
        }
    }
    //h.stop();   // can be omitted; dropping suffices

    Ok()
}
