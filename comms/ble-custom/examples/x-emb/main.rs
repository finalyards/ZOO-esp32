/*
* Custom BLE server example
*
* Based on:
*   - https://github.com/embassy-rs/trouble/tree/main/examples/esp32
*   - https://github.com/jamessizeland/microbit-ble-gamepad/
*/
#![no_std]
#![no_main]
extern crate alloc;

#[allow(unused_imports)]
use defmt::{info};
use defmt_rtt as _;
use esp_alloc as _;
use esp_backtrace as _;
use embassy_sync as _;      // so that it shows as active in 'Cargo.toml' in the IDE; '#[gatt_server]' uses it

use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;
use esp_hal::{
    prelude::*,
    timer::{
        timg::TimerGroup
    }
};
use esp_wifi::{
    EspWifiController,
    ble::controller::BleConnector
};
use static_cell::StaticCell;
use trouble_host::prelude::*;

use ble_custom::run;

mod svc_magic;
use svc_magic::*;

mod svc_rgb;
use svc_rgb::*;

// Type of controller. 'trouble' example has them as generic arguments, but we can also just nail
// down the type (simpler to e.g. launch tasks). To support multiple kinds of platforms - but not
// concern here since ESP32 only - different type definitions under feature flags can be considered.
//
// tbd. Write what should be considered for the 'SLOT' - one example has '20'; 'trouble' tests 10.
//
pub(crate) type MyC = ExternalController<BleConnector<'static>, /*SLOTS*/ 10>;

include!("./config.in");
// NAME
// BRAND
// SERVICE_UUID
// CHAR_{...}_UUID

/*
* Collection of the 1..n services
*
* The '#[gatt_server(...)]' macro creates a plain 'struct' that is _not_ of any particular type.
*/
#[gatt_server]
struct MyServer {
    rgb_service: RgbService,
    magic_service: MagicService,
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
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
        let a = &*SC.init(
            esp_wifi::init(
                timg0.timer0,
                esp_hal::rng::Rng::new(peripherals.RNG),
                peripherals.RADIO_CLK,
            ).unwrap()
        );
        a
    };

    let systimer = {
        use esp_hal::timer::systimer::{SystemTimer, Target};
        SystemTimer::new(peripherals.SYSTIMER).split::<Target>()
    };
    esp_hal_embassy::init(systimer.alarm0);

    let controller: MyC = {
        let connector = BleConnector::new(&init, peripherals.BT);
        MyC::new(connector)
    };

    controller.run(spawner).await;
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
