/*
* Just the basics - test defmt logging; semihosting exit
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    main,
};

use semihosting::process;

#[main]
fn main() -> ! {
    init_defmt();

    match main2() {
        Err(e) => panic!("Failed with: {:?}", e),
        Ok(()) => process::exit(0)      // back to developer's command line
    }
}

fn main2() -> Result<(),()> {
    info!("Hi!");

    Ok(())
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
*
* Note: 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
*       read operation". Our 'esp_hal::time::now' isn't, but sure seems to work.
*
* Reference:
*   - defmt book > ... > Hardware timestamp
*       -> https://defmt.ferrous-systems.com/timestamps#hardware-timestamp
*/
fn init_defmt() {
    use esp_hal::time::now;

    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
}
