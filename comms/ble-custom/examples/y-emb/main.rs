#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

//use esp_alloc as _;
use esp_backtrace as _;

//use bt_hci::controller::ExternalController;

use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    signal::Signal
};
use esp_hal::{
    clock::CpuClock,
    gpio::{Input, Pull},
    //timer::timg::TimerGroup
};
//use esp_wifi::ble::controller::BleConnector;
//use semihosting;

mod boot_btn_task;
use boot_btn_task::{ButtonState, btn_task};

static BTN_SIGNAL: Signal<CriticalSectionRawMutex, ButtonState> = Signal::new();
    // Note: Using 'CriticalSectionRawMutex' to please the compiler. Wanted to use 'NoopRawMutex',
    //      since we only have one executor.

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut x = esp_hal::Config::default();
        x.cpu_clock = CpuClock::max();
        x
    });
    //esp_alloc::heap_allocator!(72 * 1024);
    //let timg0 = TimerGroup::new(peripherals.TIMG0);

    /***let init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
        .unwrap();***/

    //---

    let btn_pin = Input::new(peripherals.GPIO9, Pull::Up);  // BOOT button

    spawner.spawn(btn_task(btn_pin, &BTN_SIGNAL))
        .unwrap();

    info!("Press the BOOT button to see logs");

    loop {
        match BTN_SIGNAL.wait() .await {
            ButtonState::Pressed => {
                info!("pressed");
            },
            ButtonState::Depressed => {
                info!("released");
            }
        }
    }
}
