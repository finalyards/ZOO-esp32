//! ellie.rs
//!
//! WIP: small moves, Ellie!!

#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;

use embassy_executor::Spawner;
use esp_backtrace as _;

use esp_hal::{
    clock::ClockControl,
    // "The 'Delay' driver provides blocking delay functionalities using the `SYSTIMER` peripheral
    // for RISC-V devices [...].
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

#[main]
async fn main(_spawner: Spawner) {

    info!("Init!");
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    info!("Let's do delay");
    let delay = Delay::new(&clocks);
    delay.delay_micros(1000_u32);

    info!("Yee!");

    /***
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timer0 = OneShotTimer::new(timg0.timer0.into());
    let timers = [timer0];
    let timers = mk_static!([OneShotTimer<ErasedTimer>; 1], timers);
    esp_hal_embassy::init(&clocks, timers);

    loop {
        info!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }***/
}
