#![no_std]
#![no_main]
//likely needed: extern crate alloc;

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

use embassy_executor::Spawner;
use esp_hal::{
    clock::CpuClock,
    efuse::Efuse,
    gpio::{AnyPin, Input, InputConfig, /*Output, OutputConfig,*/ Pull},
    interrupt::software::SoftwareInterruptControl,
    rng::{/*Trng,*/ TrngSource},
    timer::{
        timg::TimerGroup
    }
};
use esp_radio::ble::{
    controller::BleConnector,
    Config
};

use embassy_time as _;      // show enabled in 'Cargo.toml'; we want the time stamp for 'defmt' logs
use esp_backtrace as _;

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
        .with_pull(Pull::Up)    // also the devkit has its own, external pull-up
    );

    // Address is Random, as in -> https://embassy.dev/trouble/#_random_address
    let a: Address = Address::random(Efuse::mac_address());     // 6 bytes MAC
    debug!("Our address: {:?}", a);    // "10:15:07:04:32:54"

    let trng_src = TrngSource::new(peripherals.RNG, peripherals.ADC1);  // must _stay_ alive, for 'Trng' instances to function

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

        let _ = ble_controller;
        let _ = trng_src;
    }

    for count in 0_u32.. {
        // Wait for a full button press; a release that is.
        //
        // Note: On first round, we fly just through (button already depressed), and get an initial
        //      color.
        //
        loop {
            let x = btn_signal.wait() .await;
            if !x.is_pressed() { break; }
        }

        let color = match count%4 {
            0 => LedState::State1,
            1|3 => LedState::State2,
            2 => LedState::State3,
            _ => unreachable!()
        };
        info!("Signalling: {}", color);
        led_signal.signal(color);
    };

    unreachable!()
}

//R Server::run(ble_controller, a, trng).await
