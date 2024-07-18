// From 'esp-hal-template' https://github.com/jessebraham/esp-hal-template/blob/main/embassy/src/bin/firmware.rs

#![no_std]
#![no_main]

// Cannot do conditional on attributes, right?  Maybe can set this in the Makefile. #ohwell
#![feature(type_alias_impl_trait)]    // needs 'nightly'

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    rtc_cntl::Rtc,
    system::SystemControl,
    timer::{timg::TimerGroup, OneShotTimer},
};
use static_cell::{
    //*,
    make_static     // nightly
};

/*** #[rustversion::stable]
// Stable
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}***/

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Enable the RWDT watchdog timer:
    let mut rtc = Rtc::new(peripherals.LPWR, None);
    rtc.rwdt.set_timeout(2.secs());
    rtc.rwdt.enable();
    info!("RWDT watchdog enabled!");

    // Initialize the SYSTIMER peripheral, and then Embassy:
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timers = [OneShotTimer::new(timg0.timer0.into())];

    /*** tbd. just need one solution, for now.
    //#[rustversion::stable]
    let timers = mk_static!([OneShotTimer<Timer>; 1], timers);
    ***/

    //#[rustversion::nightly]
    let timers = make_static!(timers);  // nightly

    esp_hal_embassy::init(&clocks, timers);
    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;

    // Periodically feed the RWDT watchdog timer when our tasks are not running:
    loop {
        rtc.rwdt.feed();
        Timer::after(Duration::from_secs(1)).await;
    }
}