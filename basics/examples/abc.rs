/*
* Just the basics - test defmt logging
*/
#![no_std]
#![no_main]

use anyhow::Result;

use core::mem::MaybeUninit;

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    prelude::*,
    time::now
};

use semihosting::process;

#[entry]
fn main() -> ! {
    init_defmt();
    init_heap();

    match main2() {
        Err(e) => panic!("Failed with: {:?}", e),
        Ok(()) => process::exit(0)      // back to developer's command line
    }
}

fn main2() -> Result<()> {
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
    const HEAP_SIZE: usize = 8 * 1024;     // 'esp_alloc' docs aim at 32K
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

const D_PROVIDER: Delay = Delay::new();
fn delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
