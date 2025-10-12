#![no_std]
#![no_main]
extern crate alloc;

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

use embassy_time as _;      // show enabled in 'Cargo.toml'; we want the time stamp for 'defmt' logs

use esp_backtrace as _;

use embassy_executor::Spawner;
use esp_hal::{
    clock::CpuClock,
    efuse::Efuse,
    gpio::{AnyPin, Input, InputConfig, Output, OutputConfig, Pull},
    interrupt::software::SoftwareInterruptControl,
    peripherals::RMT,
    rng::{Trng, TrngSource},
    timer::{
        timg::TimerGroup
    }
};
use esp_radio::ble::{
    controller::BleConnector,
    Config
};

//use static_cell as _;   // so IDE shows it as active

#[allow(unused_imports)]
use trouble_host::{
    prelude::*,
    Address,
};

mod tasks;
//mod server;
//mod state;

//use server::Server;

include!(concat!(env!("OUT_DIR"), "/pins_snippet.in")); // pins!

use crate::{
    tasks::btn_task::{
        btn_task,
        BTN_SIGNAL
    },
    tasks::led_task::{
        led_task,
        LedState,
        LED_SIGNAL
    }
};

#[allow(non_snake_case)]
struct Pins<'a>{
    BOOT: AnyPin<'a>,
    RGB_LED: AnyPin<'a>,
}

esp_bootloader_esp_idf::esp_app_desc!();

/*
* In 'main', we prepare the hardware.
*/
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {

    let peripherals = esp_hal::init(
        esp_hal::Config::default()
            .with_cpu_clock(CpuClock::max())
    );
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(
        timg0.timer0,
        sw_int.software_interrupt0
    );

    let radio;  // life span
    let ble_controller = {
        radio = esp_radio::init().unwrap();
        BleConnector::new(&radio, peripherals.BT, Config::default().with_task_priority(10))
    }.unwrap();

    #[allow(non_snake_case)]
    let Pins{ BOOT, RGB_LED } = pins!(peripherals);

    #[allow(non_snake_case)]
    let BOOT = Input::new(BOOT, InputConfig::default()
        .with_pull(Pull::Up)
    );

    // Address is Random, as in -> https://embassy.dev/trouble/#_random_address
    let a: Address = Address::random(Efuse::mac_address());     // 6 bytes MAC
    #[cfg(false)]   // Using a fixed address can be useful for testing.
    let a: Address = Address::random(b"rand0m".into());

    debug!("Our address = {:?}", a);    // output as: "10:15:07:04:32:54" tbd.

    let trng_src = TrngSource::new(peripherals.RNG, peripherals.ADC1);  // must be _stay_ alive, for 'Trng' instances to function

    let btn_signal = &BTN_SIGNAL;
    let led_signal = &LED_SIGNAL;

    // Background tasks
    {
        spawner.spawn(btn_task(BOOT))
            .unwrap();

        spawner.spawn(led_task(peripherals.RMT, RGB_LED))
            .unwrap();
    }

    // Start the state circus
    {
        //let state_wheel = State::new(ble_controller, a, trng_src);

    }

    loop {
        let x = btn_signal.wait() .await;
        info!("Heard: {}", x);

        let color = match x.is_pressed() {
            true => LedState::State1,
            false => LedState::State2
        };
        info!("Signalling: {}", color);
        led_signal.signal(color);
    }
}

//R Server::run(ble_controller, a, trng).await
