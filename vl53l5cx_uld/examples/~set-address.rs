/*
* Utility to change the I2C address of a VL53L5CX chip.
*
* Used in collaboration with the 'set-address.sh' host side script.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{self, Io, AnyOutput, Level},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use semihosting::process::exit;

extern crate vl53l5cx_uld as uld;
mod common;
mod defmt_timestamps;

use common::MyPlatform;
use uld::{
    VL53L5CX
};

// './set-address.sh' changes these lines
const OLD_ADDR: u8 = 0x52;
const NEW_ADDR: u8 = 0x58;

#[entry]
fn main() -> ! {
    #[cfg(not(feature = "semihosting_args"))]
    let (old_addr, new_addr): (u8, u8) = (OLD_ADDR, NEW_ADDR);
    #[cfg(feature = "semihosting_args")]
    let (old_addr, new_addr): (u8, u8) = get_args();

    for a in [old_addr, new_addr] {
        assert!(a%2 ==0, "Bad I2C addresses - vendor C API uses 8-bit addresses, where the LSB must be 0. {:#04x}", a);
    }

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    defmt_timestamps::init();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    #[allow(non_snake_case)]
    let (pinSDA, pinSCL, pinPWR_EN, _) = {
        (io.pins.gpio4, io.pins.gpio5, Some(io.pins.gpio0), gpio::NO_PIN)      // esp32c3
        //(io.pins.gpio22, io.pins.gpio23, Some(io.pins.gpio21), gpio::NO_PIN)    // esp32c6
    };

    let i2c_bus = I2C::new_with_timeout(
        peripherals.I2C0,
        pinSDA,
        pinSCL,
        400.kHz(),
        &clocks,
        None,   // default is fine
    );

    let mut pwr_en = pinPWR_EN.map(|pin| AnyOutput::new(pin, Level::Low));

    let d_provider = Delay::new(&clocks);
    let delay_ms = |ms| d_provider.delay_millis(ms);

    // Reset VL53L5CX by pulling down its power for a moment
    pwr_en.iter_mut().for_each(|pin| {
        pin.set_low();
        delay_ms(50);      // tbd. how long is suitable, by the specs?
        pin.set_high();
        info!("Target powered off and on again.");
    });

    /*** R???
    // Check both addresses, to see whether there's already a VS out there
    //
    // Note: This uses a feature that doesn't need the device to have been initialized (i.e. no
    //      firmware needs to run on it).
    {
        for a in [old_addr, new_addr, 0x02, 0x04] {
            let pl = MyPlatform::new(&clocks, i2c_bus, a);
            let vl = VL53L5CX::new(pl);     // created; not initialized

            match vl.ping() {
                Ok(true) => { debug!("{:#04x}: seems to hold a VL53L5CX"); },
                Ok(false) => { debug!("{:#04x}: _may_ hold a VL53L5CX (dev id, rev id weren't as expected)") },
                Err(_) => { debug!("{:#04x}: no VL53L5CX") }
            }
        }
    }***/

    let pl = MyPlatform::new(&clocks, i2c_bus, old_addr);

    // main stuff
    {
        info!("Initializing the device at {:#04x}", old_addr);

        let mut vl = VL53L5CX::new_and_init(pl)
            .unwrap();

        info!("Changing address to -> {:#04x}", new_addr);

        match vl.set_i2c_address(new_addr) {
            Ok(()) => {
                info!("Address changed");
            },
            Err(e) => {
                error!("Address change failed (code {=u8})", e);
                exit(1);
            }
        }
    }

    exit(0);
}

#[cfg(feature = "semihosting_args")]
fn get_args() -> (u8,u8) {
    let a = args::<50>().expect("to have args: {old-address:u8} {new-address:u8}");
    debug!("!!! {}", a);
    unimplemented!();

    /* stash
    let raw = "0x1f";
    let without_prefix = raw.trim_start_matches("0x");
    let z = u8::from_str_radix(without_prefix, 16);
    println!("{:?}", z);
    */
}
