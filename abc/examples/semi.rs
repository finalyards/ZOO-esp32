/*
* Working with multiple boards
*
* We do this by using a 'flock' variant of the drivers.
*/
#![no_std]
#![no_main]

use anyhow::Result;

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    prelude::*,
    time::now
};

use semihosting::{
    io::Read,
    println,
    process
};

#[entry]
fn main() -> ! {
    init_defmt();
    init_heap();

    match main2() {
        Err(e) => {
            panic!("Failed with: {:?}", e);
        },

        Ok(()) => {
            process::exit(0);      // back to developer's command line
        }
    }
}

fn main2() -> Result<()> {
    println!("Hi from semihosting!");

    let mut stdio = semihosting::io::stdin()?;
    let mut buf = [0u8;1];

    loop {
        let n = stdio.read(&mut buf)?;
        match n {
            0 => {},
            1 => break,
            _ => {
                debug!("{=u8:#04x}",&buf[0]);
            }
        }
    }

    // Ask things interactively
    //prompt("Do you like this?");

    Ok(())
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
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

/*
* To use 'anyhow' under 'no_std', we do need a global allocator.
*   <<
*       To depend on 'Anyhow' in 'no_std' mode, disable our default enabled “std” feature in 'Cargo.toml'.
*       **A global allocator is required.**
*   <<
*/
fn init_heap() {
    use esp_alloc as _;
    use core::mem::MaybeUninit;

    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}