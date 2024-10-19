/*
* Steering a DC motor's speed (no feedback)
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
    delay::Delay,
    gpio::Io,
    prelude::*,
    time::now
};
#[cfg(feature="mcpwm")]
use esp_hal::{
    mcpwm::{
        operator::PwmPinConfig,
        timer::PwmWorkingMode,
        McPwm,
        PeripheralClockConfig
    },
};

include!("./pins-gen.in");  // pins!

#[entry]
fn main() -> ! {
    init_defmt();
    main2()
        .map_err(|e| panic!("Failed with: {}", e))
        .unwrap();

    info!("End of demo");
    semihosting::process::exit(0);      // back to developer's command line
}

/*
* ESP32-C3 does not have a dedicated Motor Control PWM. We use a 'LEDC', instead.
*
* Based on:
*   - "ESP32 Embedded Rust at the HAL: PWM Buzzer"
*       -> https://dev.to/theembeddedrustacean/esp32-embedded-rust-at-the-hal-pwm-buzzer-5b2i
*/
#[cfg(not(feature="mcpwm"))]    // esp32c3
fn main2() -> Result<(),u8> {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    #[allow(non_snake_case)]
    let (IN1, IN2, _BTN) = pins!(io);

    let mut buzzer = LEDC::new(
        peripherals.LEDC,
        &clocks,
        &mut system.peripheral_clock_control,
    );

    buzzer.set_global_slow_clock(LSGlobalClkSource::APBClk);
    
    let pwm = Pwm::new(peripherals.TIM2, IN1, IN2);

    //First, let's run the motor with 10% PWM
    //
    info!("Running in 10%");

    pwm.run(10);
    delay_ms(1000);
    pwm.stop();

    Ok(())
}

#[cfg(feature="mcpwm")]    // esp32c6
fn main2() -> Result<(),u8> {
    unimplemented!()
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

const D_PROVIDER: Delay = Delay::new();
fn delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}


/*
* Provide control to the drv8871 speeds; forward or backward.
*
* Based on the 'esp-hal' 'examples/mcpwm.rs'.
*/
//R Uses 'timer0' and 'operator0' of the MCPWM0 peripheral to output a 50% duty signal at 20 kHz.
struct PWM {

}

impl PWM {
    fn new() -> Self {

    }

    fn run(prc: i8) {

    }

    fn stop(&mut self) { self.run(0); }
}

impl Drop for PWM {
    fn drop(&mut self) { self.stop(); }
}

/***    Sample for "mcpwm":

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let pin = io.pins.gpio0;

    // initialize peripheral
    cfg_if::cfg_if! {
        if #[cfg(feature = "esp32h2")] {
            let freq = 40.MHz();
        } else {
            let freq = 32.MHz();
        }
    }

    let clock_cfg = PeripheralClockConfig::with_frequency(freq).unwrap();

    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);

    // connect operator0 to timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    // connect operator0 to pin
    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(pin, PwmPinConfig::UP_ACTIVE_HIGH);

    // start timer with timestamp values in the range of 0..=99 and a frequency of
    // 20 kHz
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(99, PwmWorkingMode::Increase, 20.kHz())
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);

    // pin will be high 50% of the time
    pwm_pin.set_timestamp(50);

    loop {}
}
***/