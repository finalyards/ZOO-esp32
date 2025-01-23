#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

use esp_alloc as _;
use esp_backtrace as _;

use bt_hci::controller::ExternalController;

use embassy_executor::Spawner;
use embassy_sync::{
    signal::Signal
};
use esp_hal::{
    clock::CpuClock,
    gpio::{Input, Pull},
    timer::timg::TimerGroup
};
use esp_wifi::ble::controller::BleConnector;

mod boot_btn_task;
mod boot_btn_ble;
mod server_ble;

use crate::{
    boot_btn_task::{BtnSignal, btn_task},
    server_ble::Server
};

pub(crate) static BTN_SIGNAL: BtnSignal = Signal::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init({
        let mut x = esp_hal::Config::default();
        x.cpu_clock = CpuClock::max();
        x
    });
    esp_alloc::heap_allocator!(72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
        .unwrap();

    {
        let systimer = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(systimer.alarm0);
    }

    let controller: ExternalController<_, 10 /*SLOTS*/> = {
        let tmp = BleConnector::new(&init, peripherals.BT);
        ExternalController::new(tmp)
    };

    //---
    // Boot button task is being run constantly on the background (even when there's no BLE
    // connection). This is just a matter of taste - use 'AnyServiceTask' for running something
    // just when connected.
    {
        let btn_pin = Input::new(peripherals.GPIO9, Pull::Up);  // BOOT button

        spawner.spawn(btn_task(btn_pin, &BTN_SIGNAL))
            .unwrap();
    }

    //---

    Server::run(controller) .await
}
