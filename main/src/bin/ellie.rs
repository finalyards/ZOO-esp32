//! ellie.rs
//!
//! WIP: small moves, Ellie!!

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
#[warn(unused_imports)]
use {
    defmt::{debug, info},
    defmt_rtt as _
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::{timg::TimerGroup, OneShotTimer},
};

// 'stable' (not nightly) way of creating statics
use {
    static_cell::StaticCell,
    esp_hal::timer::ErasedTimer
};

// 'stable' way to make this - 'static_cell::make_static' requires 'nightly'.
// tbd. HOPEFULLY there's a stable, non-macro way, soonish (< 2030)=?
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static S: StaticCell<$t> = StaticCell::new();
        #[deny(unused_attributes)]
        let x = S.init(($val));
        x
    }};
}

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initialize the SYSTIMER peripheral, and then Embassy:
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timers = [OneShotTimer::new(timg0.timer0.into())];

    let timers = mk_static!([OneShotTimer<ErasedTimer>; 1], timers);
    esp_hal_embassy::init(&clocks, timers);
    #[cfg(feature = "defmt")]
    info!("Embassy initialized!");

    // synchronous (busy wait) delays
    let delay_ms: _ = {
        let d = mk_static!(esp_hal::delay::Delay, esp_hal::delay::Delay::new(&clocks));
        |ms| { d.delay_millis(ms); }
    };

    loop {
        #[cfg(feature = "defmt")]
        info!("Bing!");
        Timer::after(Duration::from_millis(3_000)).await;   // async wait

        #[cfg(feature = "defmt")]
        debug!("Bong!");
        delay_ms(1000);     // sync wait
    }
}
