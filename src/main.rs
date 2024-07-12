//! main.rs
//!
//! WIP: interfacing with the ST C library, both ways.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;

/***
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::{timg::TimerGroup, ErasedTimer, OneShotTimer},
};***/

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    esp_println::println!("Init!");
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let x = VL53::new();
    x.say();

    /***
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timer0 = OneShotTimer::new(timg0.timer0.into());
    let timers = [timer0];
    let timers = mk_static!([OneShotTimer<ErasedTimer>; 1], timers);
    esp_hal_embassy::init(&clocks, timers);

    loop {
        esp_println::println!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }***/
}
