/*
* 'defmt' logging together with interactive host/MCU channels.
*
* Using 'rtt-target' for this.
*
* NEEDS v.0.6 of 'rtt-target'. See -> https://github.com/probe-rs/rtt-target/blob/master/rtt-target/src/lib.rs#L119-L122
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    prelude::*,
    time::now
};

#[entry]
fn main() -> ! {
    let channels = rtt_target::rtt_init_default!();    // 1024 bytes for out ("Terminal"); 16 bytes for in
    let (ch_up, mut ch_down) = (channels.up.0, channels.down.0);
    rtt_target::set_defmt_channel(ch_up);

    init_defmt();
    info!("Init!");     // see that 'defmt' output works

    let mut buf = [0_u8;10];

    // Read keys from the host terminal; show their codes
    loop {
        let n = ch_down.read(&mut buf);
        if n>0 {
            info!("Key(s): {}", &buf[0..n]);
        }
    };

    //semihosting::process::exit(0);
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging. Also
*
* Note: 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
*       read operation". Out 'esp_hal::time::now' isn't, but sure seems to work.
*
* Reference:
*   - defmt book > ... > Hardware timestamp
*       -> https://defmt.ferrous-systems.com/timestamps#hardware-timestamp
*/
fn init_defmt() {
    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
}
