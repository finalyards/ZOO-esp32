//! 0-nada.rs
//!
//! No access to the sensor. Just testing the building and running workflow is in place.
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{debug, info, error};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

use static_cell::StaticCell;

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

struct

fn main() {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // synchronous (busy wait) delays
    let delay_ms: _ = {
        let d = mk_static!(esp_hal::delay::Delay, esp_hal::delay::Delay::new(&clocks));
        |ms| { d.delay_millis(ms); }
    };

    debug!("Nada");

    let mut ding: bool = true;
    loop {
        delay_ms(1000);
        debug!("{}", if ding {"Ding!"} else {"Dong!"} ); ding = !ding;
    }
}
