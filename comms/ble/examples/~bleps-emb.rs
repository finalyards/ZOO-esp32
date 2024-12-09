/*
* Interacting over BLE (Bluetooth Low Energy)
*
* Based on: esp-hal/examples/src/bin / wifi_embassy_ble.rs
*   -> https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/wifi_embassy_ble.rs
*
*   - but uses 'defmt' instead of 'esp-println'
*   - limited to C6/C3 MCU's (that the author has access to)
*   - some other dependency simplifications/edits
*   - selection of chip not by features, but by macro (that proxies build-time info)
*   - using 'StaticCell' directly, instead of via a macro
*/
#![no_std]
#![no_main]

use core::{
    cell::RefCell
};

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

// Note: some below rendered red, due to Rust Rover IDE bug. Ignore it
use bleps::{
    ad_structure::{
        create_advertising_data,
        AdStructure,
        BR_EDR_NOT_SUPPORTED,
        LE_GENERAL_DISCOVERABLE,
    },
    async_attribute_server::AttributeServer,
    asynch::Ble,
    attribute_server::NotificationData,
    gatt,
};
use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Pull},
    prelude::*,
    rng::Rng,
    time,
    timer::{
        systimer::{SystemTimer, Target},
        timg::TimerGroup
    },
};
use esp_wifi::{
    ble::controller::BleConnector,
    EspWifiController
};

const CHIP: &str = esp_hal::chip!();    //"esp32c6"

const LOCAL_NAME: &str = CHIP;

// Note: 'gatt!' macro doesn't allow use of constants, but we'd like to have these defined here.
//      // maybe bypass the macro, one day?=?
//
const UUID_SVC: &str = "438785e7-4942-4749-a072-dceb73fd6c87";

const UUID_C0: &str = "448785e7-4942-4749-a072-dceb73fd6c87";
const UUID_C1: &str = "458785e7-4942-4749-a072-dceb73fd6c87";
    // tbd. experiment with 16-bit (eg. "12bc") UUID's, at some point; are they ok on a custom service?

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) -> ! {
    init_defmt();

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let init: &'static EspWifiController = {
        use static_cell::StaticCell;
        static SC: StaticCell<EspWifiController<'static>> = StaticCell::new();

        SC.init_with( ||
            esp_wifi::init(timg0.timer0, Rng::new(peripherals.RNG), peripherals.RADIO_CLK)
                .unwrap()
        )
    };

    let button = match CHIP {
        "esp32" | "esp32s2" | "esp32s3" =>  Input::new(peripherals.GPIO0, Pull::Down),
        _ =>                                Input::new(peripherals.GPIO9, Pull::Down)
    };

    if CHIP == "esp32" {
        let timg1 = TimerGroup::new(peripherals.TIMG1);
        esp_hal_embassy::init(timg1.timer0);
    } else {
        let systimer = SystemTimer::new(peripherals.SYSTIMER).split::<Target>();
        esp_hal_embassy::init(systimer.alarm0);
    }

    let mut bluetooth = peripherals.BT;

    let connector = BleConnector::new(&init, &mut bluetooth);

    let now = || time::now().duration_since_epoch().to_millis();
    let mut ble = Ble::new(connector, now);
    info!("Connector created");

    let pin_ref = RefCell::new(button);
    let pin_ref = &pin_ref;

    loop {
        info_x(ble.init().await);
        info_x(ble.cmd_set_le_advertising_parameters().await);
        info_x(
            ble.cmd_set_le_advertising_data(
                create_advertising_data(&[
                    AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                    AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1809)]),
                    AdStructure::CompleteLocalName(LOCAL_NAME),
                ])
                    .unwrap()
            )
                .await
        );
        info_x(ble.cmd_set_le_advertise_enable(true).await);

        info!("started advertising");

        let mut read_0 = |_offset: usize, data: &mut [u8]| {
            data[..20].copy_from_slice(&b"Hello Bare-Metal BLE"[..]);
            17  // <-- tbd. what's this? (NOT the length of the string)
        };
        let mut write_0 = |offset: usize, data: &[u8]| {
            info!("RECEIVED: {} {:?}", offset, data);
        };

        // Characteristic 1 (multicolored LED):
        //
        // Array of three bytes: RGB
        //
        let mut write_1 = |offset: usize, data: &[u8]| {
            info!("RECEIVED: {} {:?}", offset, data);
        };

        // Note:
        //      The 'esp-hal' example uses almost the same UUID, with 1 char variation:
        //          "937312e0-2354-11eb-9f10-fbc30a62cf38"     // for both service and characteristic #0
        //          "957[...]"      // characteristic #1
        //          "987[...]"      // characteristic #2
        //
        // We do a bit similar, but use a different UUID for the service - it's likely a mistake
        // on the side of 'esp-hal'.
        //
        // Note #2: 'gatt!' macro insists to get UUIDs as "string literals" (kind of bummer). :/
        //
        // Sets:
        //  - 'gatt_attributes'
        //  - 'abc_handle'
        //
        gatt!([service {
            uuid: "438785e7-4942-4749-a072-dceb73fd6c87",   // "937312e0-2354-11eb-9f10-fbc30a62cf38",
            characteristics: [
                characteristic {
                    uuid: "448785e7-4942-4749-a072-dceb73fd6c87", // UUID_C0
                    read: read_0,
                    write: write_0,
                },
                characteristic {
                    name: "abc",     // gets magically turned to '{...}_handle' handler
                    uuid: "458785e7-4942-4749-a072-dceb73fd6c87",   // UUID_C1
                    notify: true,
                    write: write_1,
                },
            ],
        },]);
        let mut ga = gatt_attributes;

        let mut rng = bleps::no_rng::NoRng;
        let mut srv = AttributeServer::new(&mut ble, &mut ga, &mut rng);

        let counter = RefCell::new(0u8);
        let counter = &counter;

        let mut notifier = || {
            // TODO how to check if notifications are enabled for the characteristic?
            // maybe pass something into the closure which just can query the characteristic
            // value probably passing in the attribute server won't work?

            async {
                pin_ref.borrow_mut().wait_for_rising_edge().await;
                let mut data = [0u8; 13];
                data.copy_from_slice(b"Notification0");
                {
                    let mut counter = counter.borrow_mut();
                    data[data.len() - 1] += *counter;
                    *counter = (*counter + 1) % 10;
                }
                NotificationData::new(abc_handle, &data)
            }
        };

        srv.run(&mut notifier).await.unwrap();
    }
}

// "This adapter disables compression and uses the 'core::fmt' code on-device! You should prefer
// 'defmt::Format' over 'Debug' whenever possible."
//
fn info_x<X: core::fmt::Debug + Sized>(x: Result<X,bleps::Error>) {
    info!("{:?}", defmt::Debug2Format(&x));
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
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
