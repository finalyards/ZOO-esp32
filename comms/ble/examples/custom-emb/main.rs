/*
* Battery Level example
*
* Based strongly on:
*   trouble > examples > esp32
*       -> https://github.com/embassy-rs/trouble/tree/main/examples/esp32
*/
#![no_std]
#![no_main]

mod ble_custom;
use ble_custom::run_stack;

#[allow(unused_imports)]
use defmt::{info};
use defmt_rtt as _;

use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    prelude::*,
    timer::{
        timg::TimerGroup
    }
};
use esp_wifi::EspWifiController;
use static_cell::StaticCell;

#[esp_hal_embassy::main]
async fn main(_s: Spawner) {
    init_defmt();

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });
    esp_alloc::heap_allocator!(72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let init: &EspWifiController<'static> = {
        static SC: StaticCell<EspWifiController> = StaticCell::new();
        let tmp = &*SC.init(
            esp_wifi::init(
                timg0.timer0,
                esp_hal::rng::Rng::new(peripherals.RNG),
                peripherals.RADIO_CLK,
            ).unwrap()
        );
        tmp
    };

    let systimer = {
        use esp_hal::timer::systimer::{SystemTimer, Target};
        SystemTimer::new(peripherals.SYSTIMER).split::<Target>()
    };
    esp_hal_embassy::init(systimer.alarm0);

    run_stack(peripherals.BT, &init).await;
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
    use esp_hal::time::now;

    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
}
