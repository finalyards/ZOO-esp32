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
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let d_provider = Delay::new(&clocks);
    let delay_ms: _ = {     // synchronous (busy wait) delays
        |ms| { d_provider.delay_millis(ms); }
    };

    debug!("Nada");

    let mut ding: bool = true;
    loop {
        delay_ms(1000);
        debug!("{}", if ding {"Ding!"} else {"Dong!"} ); ding = !ding;
    }
}
