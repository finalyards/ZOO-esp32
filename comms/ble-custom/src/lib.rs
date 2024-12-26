//
// Common tasks (and other things) for any BLE custom peripheral.
//
// Based on:
//  - trouble > examples > ... > ble_bas_peripheral.rs
//
#![no_std]

#[cfg(feature="defmt")]
use defmt::{error, info, debug};

use embassy_futures::select::select;
use esp_hal::efuse::Efuse;
use trouble_host::{
    HostResources,
    prelude::*
};

// tbd. Write here, when L2CAP values matter. Likely only for certain kinds of BLE services??
//
const L2CAP_MTU: usize = 251;           // size of L2CAP packets (ATT MTU is this - 4)
const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 2;    // max number of L2CAP channels

type Resources<C: Controller> = HostResources<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>;

// Implement this for a service tagged with '#[gatt_service ...]'. This way, 'BleCustom::run'
// can use it. Without the trait, '#[gatt_service ...]' just creates an ad-hoc Struct.
//
// #trouble: Would it be meaningful for '#[gatt_service ...]' to add some type for the Struct?
//
trait IsAGattServer {
    fn new_default(name: &str) -> Self;
}

/*
* Wrap around something that wants to be a custom BLE service.
*
* Note: By keeping 'Self' a 'Controller', we can do without introducing '<C: Controller>' generics.
*/
pub trait BleCustom: Controller {
    type Server: IsAGattServer;

    // Caller-provided methods:
    fn get_peripheral_name() -> &'static str;
    fn get_brand() -> &'static str;

    fn custom_tasks(server: &Self::Server, conn: &Connection<'_>);

    // Public
    async fn run(&self) -> Result<(), BleHostError<Self::Error>> {
        let address = Efuse::mac_address();     // specific to each device
        let mut resources = Resources::new(PacketQos::None);

        // Note: In 'trouble's example, this is after the 'trouble_host::new', but no longer
        //  requires 'stack' from it. Can we create it already here?  (if not, 'trouble' API is not ideal)
        //
        #[cfg(feature="defmt")]
        debug!("Creating the Server");
        let server = Self::Server::new_default(
            "abc"   // tbd. where shows?    // note: max 22 chars; otherwise 'Error'
        )
            .unwrap();

        let (_stack, mut peripheral, runner) = trouble_host::new(self, &mut resources)
            .set_random_address( Address::random(address) )
            .build();

        #[cfg(feature="defmt")]
        debug!("Starting advertising and GATT service");

        let brand: &str = Self::get_brand();

        let app_fut = async {
            loop {
                match advertise(brand, &mut peripheral).await {
                    Ok(conn) => {
                        // run these tasks only while connected to a central
                        let fut1 = conn_task(&server, &conn);
                        let fut2 = Self::custom_tasks(&server, &conn);

                        // Run until any of the tasks ends (usually because the connection has been
                        // closed); return to advertising.
                        // NOTE: going out of scope clears the other task.
                        select(fut1, fut2).await;
                    }
                    Err(e) => {
                        #[cfg(feature = "defmt")]
                        let e = defmt::Debug2Format(&e);
                        panic!("BLE advertising: {:?}", e);
                    }
                }
            }
        };
        select(ble_task(runner), app_fut) .await;
    }
}

// Rust note:
//  Traits cannot have private methods, so the rest ('ble_task' etc.) are bare functions.
//  This does mean we need to use generics on them.
//
async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) {
    loop {
        if let Err(e) = runner.run().await {
            #[cfg(feature = "defmt")]
            let e = defmt::Debug2Format(&e);
            panic!("[ble_task] error: {:?}", e);
        }
    }
}

// Handle Connection and Gatt events until the connection closes.
async fn conn_task<Server: IsAGattServer>(server: &Server, conn: &Connection<'_>) -> Result<(), Error> {
    let magic = server.my_service.magic;    // tbd. these from the enclosing entity
    let rgb = server.my_service.rgb;

    loop {
        match conn.next().await {
            ConnectionEvent::Disconnected { reason } => {
                #[cfg(feature="defmt")]
                info!("[gatt] disconnected: {:?}", reason);
                break;
            },
            ConnectionEvent::Gatt { event: GattEvent::Read{ value_handle } , .. } => {
                if value_handle == magic.handle {
                    let value = server.get(&magic);
                    #[cfg(feature="defmt")]
                    info!("[gatt] Read Event to 'magic' Characteristic: {:?}", value);
                } else {
                    #[cfg(feature="defmt")]
                    error!("[gatt] Unknown read: {}", value_handle);
                }
            },
            ConnectionEvent::Gatt { event: ev@ GattEvent::Write{ value_handle } , .. } => {
                if value_handle == rgb.handle {
                    let new_value = ev.data();

                    #[cfg(feature="defmt")]
                    info!("[gatt] Write Event to 'rgb' Characteristic: {:?}", new_value);
                    server.set(&rgb, new_value).await;
                } else {
                    #[cfg(feature="defmt")]
                    error!("[gatt] Unknown write: {}", value_handle);
                }
            },
        }
    }

    #[cfg(feature="defmt")]
    info!("[gatt] task finished");
    Ok(())
}

// Create an advertiser to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
) -> Result<Connection<'a>, BleHostError<C::Error>> {
    let mut adv_data = [0; 31];

    // tbd. Make that return a slice; use a buffer but get to right length right away.
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            //R AdStructure::ServiceUuids16(&[Uuid::Uuid16([0x0f, 0x18])]),     // tbd.
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
    #[cfg(feature="defmt")]
    info!("[adv] advertising");
    let conn = advertiser.accept().await?;
    #[cfg(feature="defmt")]
    info!("[adv] connection established");
    Ok(conn)
}
