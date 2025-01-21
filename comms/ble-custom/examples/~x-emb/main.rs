/*
* Custom BLE server example
*
* Based on:
*   - https://github.com/embassy-rs/trouble/tree/main/examples/esp32
*   - https://github.com/jamessizeland/microbit-ble-gamepad/
*       Note: That repo (as of Jan'25) needs some tuning to compile on latest 'trouble' GitHub.
*/
#![no_std]
#![no_main]
//Rextern crate alloc;

#[allow(unused_imports)]
use defmt::{info};
use defmt_rtt as _;

use esp_alloc as _;
use esp_backtrace as _;
use embassy_sync as _;      // so that it shows as active in 'Cargo.toml' in the IDE; '#[gatt_server]' uses it

use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_futures::select::select;
use esp_hal::{
    prelude::*,
    timer::{
        systimer::SystemTimer,
        timg::TimerGroup
    }
};
use esp_hal::efuse::Efuse;
use esp_hal::peripherals::Peripherals;
use esp_wifi::{
    EspWifiController,
    ble::controller::BleConnector
};
use static_cell::StaticCell;
use trouble_host::prelude::*;

use ble_custom::{
    IsGattServer,
    BleCustom
};

mod svc_magic;
use svc_magic::MagicService;

//~mod svc_rgb;
//~use svc_rgb::RgbService;

const SLOTS: usize = 20;        // tbd. what it steers?  'trouble' example has 20; tests 10

include!("./config.in");
// NAME
// BRAND
// RGB_SERVICE_UUID
// CHAR_RGB_{...}_UUID

const L2CAP_MTU: usize = 251;       // size of L2CAP packets (ATT MTU is this - 4)

const CONNECTIONS_MAX: usize = 1;   // max number of connections

const L2CAP_CHANNELS_MAX: usize = 3;    // L2CAP channels: magic + RGB + ATT

//const MAX_ATTRIBUTES: usize = 10;       // tbd. how is this counted?

/*
* Collection of the services
*
* The '#[gatt_server]' macro creates a plain 'struct' that is _not_ of any particular type.
*/
#[gatt_server]
struct MyServer {
    //R services: (/*RgbService,*/ MagicService)
    //svc_rgb: RgbService,
    svc_magic: MagicService,
}

/***R
impl XGattServer for MyServer {
    fn new_with_name(name: &str) -> Result<Self,Error> {
        Self::new_default(name)
    }

    fn ctic_by_handle(&mut self, handle: u16) -> Option<&mut MagicService> {
        let svcs = [/*&mut self.svc_rgb,*/ &mut self.svc_magic].iter();

        let x = svcs.find(|&svc| svc.handle == handle);
        x
    }
}***/

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    init_defmt();

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
        let tmp = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(tmp.alarm0);
    }

    MyWand::new(init, peripherals.BT)
        .run().await;
}

struct MyWand {}

impl MyWand {

    fn new(init: EspWifiController, bluetooth: u8) {
        type MyC = ExternalController<BleConnector<'static>, SLOTS>;
        type Resources = HostResources<MyC, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>;

        let controller: MyC = {
            let connector = BleConnector::new(&init, bluetooth);
            MyC::new(connector)
        };

        // Using a fixed "random" address can be useful for testing. In real scenarios, one can
        // use e.g. the MAC 6-byte address.
        #[cfg(all())]
        let address = Address::random(Efuse::mac_address());
        #[cfg(not(all()))]
        let address = Address::random([0x41, 0x5A, 0xE3, 0x1E, 0x83, 0xE7]);

        info!("BLE address to connect to: {:?}", address);

        let mut ress = Resources::new(PacketQos::None);     // tbd. document why '::None' is suitable
        let (stack, mut peripheral, runner) = trouble_host::new(controller, &mut ress)
            .set_random_address(address)
            .build();

        let server = MyServer::new_with_config(GapConfig::Peripheral(PeripheralConfig {
            name: NAME,
        }))
            .unwrap();

        controller.run(stack, peripheral, runner).await;
    }

    async fn run(self) {
        let _ = join(ble_task(runner), async {
            loop {
                match advertise("Trouble Example", &mut peripheral).await {
                    Ok(conn) => {
                        // set up tasks when the connection is established to a central, so they don't run when no one is connected.
                        let a = gatt_events_task(&server, &conn);
                        let b = custom_task(&server, &conn, stack);
                        // run until any task ends (usually because the connection has been closed),
                        // then return to advertising state.
                        select(a, b).await;
                    }
                    Err(e) => {
                        #[cfg(feature = "defmt")]
                        let e = defmt::Debug2Format(&e);
                        panic!("[adv] error: {:?}", e);
                    }
                }
            }
        })
            .await;
    }
}

async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) {
    loop {
        if let Err(e) = runner.run().await {
            #[cfg(feature = "defmt")]
            let e = defmt::Debug2Format(&e);
            panic!("[ble_task] error: {:?}", e);
        }
    }
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
