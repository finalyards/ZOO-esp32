/*
* Test writing to UART
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    gpio::Io,
    prelude::*,
    time::now
};

use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    for i in 0..10 {
        let t = now();

        println!("{} {}", i, t);
    }
    debug!("Printed 0..9 to UART");
    loop {}
}
