/*
*/
#![no_std]
#![no_main]

use anyhow::Result;

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    prelude::*,
    time::now
};

use semihosting::{
    println,
    process
};

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
    println!("Hi from semihosting!");     // works, but gets smudged with 'defmt' output

    // Args
    {
        // tbd. tried before; didn't work
    }

    // Time
    //  >> WARN probe_rs::cmd::run::normal_run_mode: Target wanted to run semihosting operation 0x11 with parameter 0x0,but probe-rs does not support this operation yet. Continuing...
    #[cfg(not(all()))]
    {
        use semihosting::experimental::time::SystemTime;

        let x = SystemTime::now();
        debug!("time is: {}", x.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());
    }

    // FS
    // Reading a file
    //  >> WARN probe_rs::cmd::run: Target wanted to open file foo, but probe-rs does not support this operation yet. Continuing...
    #[cfg(not(all()))]
    {
        let mut f = semihosting::fs::File::open(c"foo.txt")?;
        /***let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        assert_eq!(contents, "Hello, world!");
        ***/
    }

    // FS
    // Writing to a file
    //  >> WARN probe_rs::cmd::run: Target wanted to open file a.txt, but probe-rs does not support this operation yet. Continuing...
    //  >> WARN probe_rs::cmd::run::normal_run_mode: Target wanted to run semihosting operation 0x13 with parameter 0x0,but probe-rs does not support this operation yet. Continuing...
    {
        semihosting::fs::write(c"a.txt", "abc")?;
    }

    // STDIO
    // Reading from the terminal (aiming at interactive control: LEFT, RIGHT, ENTER etc.!!!)
    //
    //WARN probe_rs::cmd::run: Target wanted to open file :tt with mode 114, but probe-rs does not support this operation yet. Continuing...
    #[cfg(not(all()))]
    {
        let mut stdio = semihosting::io::stdin()?;
        let mut buf = [0u8; 1];

        loop {
            let n = stdio.read(&mut buf)?;
            debug!("{} read: {}", n, &buf[0]);
            match n {
                0 => {},
                1 => break,
                _ => {
                    debug!("{=u8:#04x}",&buf[0]);
                }
            }
            delay_ms(500);
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
    use esp_alloc as _;
    use core::mem::MaybeUninit;

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
