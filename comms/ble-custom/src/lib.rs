//
// Common tasks (and other things) for any BLE custom peripheral.
//
// Based on:
//  - trouble > examples > ... > ble_bas_peripheral.rs
//
#![no_std]

#[cfg(feature="defmt")]
use defmt::{error, info, debug, warn};

use embassy_futures::select::select;
use esp_hal::efuse::Efuse;
use trouble_host::{
    HostResources,
    prelude::*
};

use core::future::Future;

/***R
// tbd. Write here, when L2CAP values matter. Likely only for certain kinds of BLE services??
//
const L2CAP_MTU: usize = 251;           // size of L2CAP packets (ATT MTU is this - 4)
const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 2;    // max number of L2CAP channels

type Resources<C: Controller> = HostResources<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>;
***/

/***R
// Implement this for a server tagged with '#[gatt_server ...]'. This way, 'BleCustom::run'
// can use it. Without the trait, '#[gatt_server ...]' just creates an ad-hoc 'struct'.
//
// #trouble: Would it be meaningful for '#[gatt_server ...]' (and '#[gatt_service ...]') to add
//      some commonly known trait for the struct?
//
// We wouldn't need this if #trouble had similar.
pub trait IsGattServer: Sized {
    fn new_default(name: &str) -> Result<Self, Error>;

    // Find a service
    fn ctic_from_handle<T: FixedGattValue>(&self, handle: u16) -> Option<Characteristic<T>>;
}***/

/***R
/*
* Wrap around something that wants to be a custom BLE service.
*
* Note: By keeping 'Self' a 'Controller', we can do without introducing '<C: Controller>' generics.
*/
pub trait BleCustom: Controller {
    type Server: IsGattServer;

    // Caller-provided methods:
    fn get_peripheral_name() -> &'static str;
    fn get_brand() -> &'static str;

    fn custom_tasks(server: &Self::Server, conn: &Connection<'_>) -> impl Future;

    // Public
    async fn run(self) -> Result<(), BleHostError<Self::Error>> where Self: Sized {
        let address = Efuse::mac_address();     // specific to each device
        let mut resources = Resources::new(PacketQos::None);

        // Note: In 'trouble's example, this is after the 'trouble_host::new', but no longer
        //  requires 'stack' from it. Can we create it already here?  (if not, 'trouble' API is not ideal)
        //
        let (_stack, mut peripheral, runner) = trouble_host::new(self, &mut resources)
            .set_random_address( Address::random(address) )
            .build();

        #[cfg(feature="defmt")]
        debug!("Starting advertising and GATT service");

        let brand: &str = Self::get_brand();

        let server = Self::Server::new_default(
            "abc"   // tbd. where shows?    // note: max 22 chars; otherwise 'Error'
        ).unwrap();
        info!("Server created");

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
        unreachable!();
    }
}
***/

// Handle GATT events until the connection closes.
//
// tbd. This should be NON-SPECIFIC TO ANY PARTICULAR SERVICES. Move those to the caller!!!
//
async fn conn_task<S: IsGattServer> (server: &S, conn: &Connection<'_>) -> Result<(), Error> {

    loop {
        match conn.next().await {
            ConnectionEvent::Disconnected { reason } => {
                #[cfg(feature="defmt")]
                info!("Disconnected: {:?}", reason);
                break;
            },
            ConnectionEvent::Gatt { data } = {
                // We can choose to handle event directly without an attribute table:
                //  <<
                //      let req = data.request();
                //      ..
                //      data.reply(conn, Ok(AttRsp::Error { .. }))
                //  <<
                // ..but best to process it in the GATT server and act on the outcome.
                //
                match data.process(server).await {
                    Ok(Some(GattEvent::Read(event))) => {
                        unimplemented!();   //...R ctic_read_table(event.handle()).
                    },
                    Ok(_) => {
                        #[cfg(feature="defmt")]
                        warn!("Unexpected event: {:?}", &event);
                    },
                    Err(e) => {
                        #[cfg(feature="defmt")]
                        error!("Error processing event: {:?}", e);
                    }
                }
            },
        }
    }

    #[cfg(feature="defmt")]
    info!("connection task finished");
    Ok(())
}

// Create an advertiser to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
) -> Result<Connection<'a>, BleHostError<C::Error>> {
    let mut adv_data = [0; 31];

    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
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
    debug!("Advertising");
    let conn = advertiser.accept().await?;
    #[cfg(feature="defmt")]
    debug!("Connection established");
    Ok(conn)
}
