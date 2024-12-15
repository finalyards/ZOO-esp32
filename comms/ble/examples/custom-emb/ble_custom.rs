//
// Custom peripheral
//
// Provides:
//  - write to change the RGB LED on the device (with fading)
//  - read of a changing value (changes in the background)
//
// 'TrouBLE' code to check against:
//  - GitHub > ... > trouble > host/tests/service_attribute_macro.rs
//      https://github.com/embassy-rs/trouble/blob/main/host/tests/service_attribute_macro.rs
//
//      - shows use of 128-bit UUID's
//      - ..and custom service
//
use arrayvec::ArrayVec;
use bt_hci::controller::ExternalController;
use defmt::{error, info, warn};

use embassy_futures::select::select;
use embassy_time::Timer;
use esp_wifi::ble::controller::BleConnector;
use esp_wifi::EspWifiController;
use trouble_host::prelude::*;

// tbd. What are L2CAP's?  Perhaps that could be a feature in 'trouble' (if they are needed only
//      by some peripherals)?
//
const L2CAP_MTU: usize = 251;   // Size of L2CAP packets (ATT MTU is this - 4)
const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 2;    // Max number of L2CAP channels

type Resources<C> = HostResources<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>;

// GATT Server definition
#[gatt_server]
struct Server {
    my_service: MyService,
}

// Custom service
#[gatt_service(uuid = "438785e7-4942-4749-a072-dceb73fd6c87")]  // SERVICE_UUID
struct MyService {
    #[characteristic(uuid = "448785e7-4942-4749-a072-dceb73fd6c87", write, on_write = rgb_on_write)]    // CHAR_0_UUID
    rgb: [u8;3],
    #[characteristic(uuid = "458785e7-4942-4749-a072-dceb73fd6c87", read, notify, on_read = magic_on_read)]     // CHAR_1_UUID
    magic: u32
}

// Q: What's the relation of these '[...]_on_{read|write}' to the 'conn_task'???
fn magic_on_read(_conn: &Connection) {
    info!("[gatt] Read event on magic");

    // tbd. how to see 'magic', from here???
    warn!("Unimplemented!");
}

fn rgb_on_write(_conn: &Connection, data: &[u8]) -> Result<(), ()> {
    info!("[gatt] Write event on RGB: {:?}", data);

    // tbd. how to see 'rgb', from here???
    Ok(())
}

include!("./config.in");
// NAME
// BRAND
// SERVICE_UUID
// CHAR_{01}_UUID

/// Run the BLE stack.
// tbd. This can (likely?) be made into a task, since no longer templated.
//  - [ ] How to pass arguments to a task, at creation?
//
pub async fn run_stack(bluetooth: impl esp_hal::peripheral::Peripheral<P=esp_hal::peripherals::BT>, esp_wifi_ctrl: &'static EspWifiController<'_>) {

    let (_,_,_) = (SERVICE_UUID, CHAR_0_UUID, CHAR_1_UUID); // pretend to use them (cannot, in the macro)

    // tbd. take these as 'Config' parameter, from 'main'

    #[allow(non_snake_case)]
    /*const*/ let HOST_RANDOM_ADDRESS: [u8; 6] = {
        let tmp = env!("HOST_RANDOM_ADDRESS")
            .split('-')         // not 'const fn'
            .map( |ab| { u8::from_str_radix(&ab, 16).expect("hex") } )
            .collect::<ArrayVec<_,6>>();

        tmp.into_inner()
            .expect("exactly 6 values")
    };

    // Note: Moving initialization of 'Controller' here, from 'main()', makes a clearer distinction
    //      (in the author's mind) between general ESP32 initialization and BLE specific details.
    //      The 'trouble' examples having it differently (Dec'24).
    //
    let controller = {
        let connector = BleConnector::new(&esp_wifi_ctrl, bluetooth);
        ExternalController::<_, 20>::new(connector)
    };

    // Such address is not really "random" (that's just BLE parlance); it's "can be anything" and
    // used for separating advertising devices from each other (or something...).
    //
    //  tbd. Make it come from build environment
    //
    // reference -> https://github.com/embassy-rs/trouble/issues/195
    //
    let address = Address::random(HOST_RANDOM_ADDRESS);
    info!("Our address = {:?}", address);

    let mut resources = Resources::new(PacketQos::None);
    let (stack, mut peripheral, runner) = trouble_host::new(controller, &mut resources)
        .set_random_address(address)
        .build();

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(
        stack,
        GapConfig::Peripheral(PeripheralConfig {
            name: NAME,     // tbd. more describing (BLE official?) term for the value holder?
            appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
        }),
    )
        .unwrap();
    let ble_and_gatt_tasks_fut = select(ble_task(runner), gatt_task(&server));

    let app_fut = async {
        loop {
            match advertise(BRAND, &mut peripheral).await {
                Ok(conn) => {
                    // set up tasks when the connection is established to a central, so they don't run when no one is connected.
                    let fut1 = conn_task(&server, &conn);
                    let fut2 = mgk_task(&server, &conn);

                    // Run until any of the tasks ends (usually because the connection has been closed);
                    // return to advertising. NOTE: going out of scope clears the other task.
                    select(fut1, fut2).await;
                }
                Err(e) => {
                    //R #[cfg(feature = "defmt")]
                    let e = defmt::Debug2Format(&e);
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    };
    select(ble_and_gatt_tasks_fut, app_fut).await;
}

/// A background task that is required to run forever alongside any other BLE tasks.
///
/// ## Alternative
///
/// If you didn't require this to be generic for your application, you could statically spawn this with i.e.
///
/// ```rust [ignore]
///
/// #[embassy_executor::task]
/// async fn ble_task(mut runner: Runner<'static, SoftdeviceController<'static>>) {
///     runner.run().await;
/// }
///
/// spawner.must_spawn(ble_task(runner));
/// ```
// tbd. Do the '#[embassy_executor::task]' as instructed above.
async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) -> Result<(), BleHostError<C::Error>> {
    runner.run().await
}

/// Run the Gatt Server.
async fn gatt_task<C: Controller>(server: &Server<'_, '_, C>) -> Result<(), BleHostError<C::Error>> {
    server.run().await
}

/// Handle Connection and Gatt events until the connection closes.
async fn conn_task<C: Controller>(
    server: &Server<'_, '_, C>,
    conn: &Connection<'_>,
) -> Result<(), BleHostError<C::Error>> {
    let magic = server.my_service.magic;
    let rgb = server.my_service.rgb;

    loop {
        match conn.next().await {
            ConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            },
            ConnectionEvent::Gatt { event: GattEvent::Read{ value_handle } , .. } => {
                if value_handle == magic.handle {
                    let value = server.get(&magic);
                    info!("[gatt] Read Event to 'magic' Characteristic: {:?}", value);
                } else {
                    error!("[gatt] Unknown read: {}", value_handle);
                }
            },
            ConnectionEvent::Gatt { event: GattEvent::Write{ value_handle } , .. } => {
                if value_handle == rgb.handle {
                    let value = server.get(&rgb);
                    info!("[gatt] Write Event to 'rgb' Characteristic: {:?}", value);
                } else {
                    error!("[gatt] Unknown write: {}", value_handle);
                }
            },
        }
    }
    info!("[gatt] task finished");
    Ok(())
}

/// Create an advertiser to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
) -> Result<Connection<'a>, BleHostError<C::Error>> {
    let mut adv_data = [0; 31];

    // tbd. Make that return a slice; use a buffer but get to right length right away.
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[Uuid::Uuid16([0x0f, 0x18])]),     // tbd.!!!
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut adv_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &adv_data[..],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?;
    info!("[adv] connection established");
    Ok(conn)
}

/// Custom data updates (could be e.g. measurement data)
async fn mgk_task<C: Controller>(server: &Server<'_, '_, C>, conn: &Connection<'_>) {
    let mut tick: u32 = 0;
    let mgk = server.my_service.magic;
    loop {
        tick = tick.wrapping_add(1);

        info!("[adv] notifying connection of change in 'magic' {}", tick);
        if server.notify(&mgk, conn, &tick).await.is_err() {
            info!("[adv] error notifying connection");
            break;
        };
        Timer::after_secs(1).await;
    }
}
