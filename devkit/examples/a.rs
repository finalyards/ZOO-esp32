/*
* Showcase reading the BOOT button.
*/
#![no_std]
#![no_main]

use anyhow::Result;

#[allow(unused_imports)]
use defmt::{info, error};
use defmt_rtt as _;

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Pull},
    prelude::*,
    time,
};

use semihosting::process;

mod common;
use common::init_defmt;

#[entry]
fn main() -> ! {
    init_defmt();

    match main2() {
        Err(e) => panic!("Failed with: {:?}", e),
        Ok(()) => process::exit(0)      // back to developer's command line
    }
}

fn main2() -> Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let bb = Input::new(peripherals.GPIO9, Pull::Down);  // BOOT button

    let mut st: bool = bb.is_high();
    info!("{}", st );
    loop {
        let t0 = time::now();
        loop {
            if bb.is_high() == st { blocking_delay_us(100); continue }
            else { break; }
        }
        st = !st;

        let dt = time::now() - t0;
        info!("{} ({})", st, dt);
    }
}

const D_PROVIDER: Delay = Delay::new();
fn blocking_delay_us(us: u32) {
    D_PROVIDER.delay_micros(us);
}
