/*
* Steering a motor via a potentiometer
*/
#![no_std]
#![no_main]

use core::{
    result::Result
};

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    gpio::Io,
    prelude::*,
    time::now
};

const D_PROVIDER: Delay = Delay::new();

include!("./pins-gen.in");  // pins!

#[entry]
fn main() -> ! {
    init_defmt();

    match main2() {
        Err(e) => {
            panic!("Failed with: {}", e);
        },

        Ok(()) => {
            info!("End of demo");
            semihosting::process::exit(0);      // back to developer's command line
        }
    }
}

// To begin with, let's print values from the potentiometer
//
fn main2() -> Result<(),u8> {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    #[allow(non_snake_case)]
    let (POT, _, _) = pins!(io);

    let delay_ms = |ms| D_PROVIDER.delay_millis(ms);

    let mut cfg = AdcConfig::new();
    let mut adc1_pin = cfg.enable_pin(POT, Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, cfg);

    #[cfg(not(all()))]  // or? tbd.
    let (mut adc1, mut adc1_pin) = {
        let mut c = AdcConfig::new();
        let pin = c.enable_pin(POT, Attenuation::Attenuation11dB);
        let adc1 = Adc::new(peripherals.ADC1, c);
        (adc1, pin)
    };
    info!("ADC init succeeded");


    //--- loop
    //
    for _round in 0..100_000 {
        let val: u16 = nb::block!(adc1.read_oneshot(&mut adc1_pin))
            .unwrap();

        info!("ADC value: {}", val);
        delay_ms(10);
    }

    Ok(())
}

fn convert(raw: u16) -> u8 {

}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
*
* Note: 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
*       read operation". 'esp_hal::time::now' isn't, but sure seems to work.
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
