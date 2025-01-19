/*
* 'defmt' logging under Embassy.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

use embassy_executor::Spawner;

use esp_backtrace as _;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    init_defmt();
    info!("Init!");     // see that 'defmt' output works

    // No way (like 'semihosting') to exit back to command line?
    loop {}
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging. Also
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
