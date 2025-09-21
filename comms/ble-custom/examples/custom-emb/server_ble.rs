/*
* BLE server
*/
#[allow(unused_imports)]
use defmt::{info, debug, warn, error};

use embassy_futures::{join::join, select::{select, select_array};
use trouble_host::prelude::*;

use crate::boot_btn_ble::BtnService;

const CONNECTIONS_MAX: usize = 1;       // max nbr of connections
const L2CAP_CHANNELS_MAX: usize = 3;    // max nbr of L2CAP channels    // tbd. pls explain...

const L2CAP_MTU: usize = 255;   // all ESP32's are fine with this length; see -> https://github.com/esp-rs/esp-hal/issues/2984

include!("./config.in");
    // AD_NAME
    // AD_NAME2

#[gatt_server]
pub struct Server {
    bb: BtnService,
}

impl Server<'_> {
    // Run the BLE stack.
    //
    pub async fn run<C>(controller: C, addr: Address) -> !
    where
        C: Controller,
    {
        let mut ress;   // here for the lifespan
        let stack;

        let Host {
            mut peripheral,
            runner, ..
        } = {
            ress = HostResources::<CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>::new();
            stack = trouble_host::new(controller, &mut ress)
                .set_random_address(addr);
            stack.build()
        };

        debug!("Starting GATT server");

        let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
            name: AD_NAME,
            appearance: &appearance::UNKNOWN,
        }))
            .unwrap();

        let _ = join(ble_task(runner), async {
            loop {
                debug!("Starting advertising");

                match advertise(AD_NAME2, &mut peripheral).await {
                    Ok(conn) => {
                        let a = gatt_events_task(&server, &conn);

                        let bs = select_array([
                            server.bb.notify_task(&server, &conn)
                        ]);

                        // Run until one task ends (usually 'gatt_events_task', due to the connection
                        // being closed); then return to advertising.
                        select(a, bs).await;
                    }
                    Err(e) => {
                        panic!("caught: {:?}", e);
                    }
                }
            }
        }).await;

        unreachable!();
    }
}

// tbd. comment
async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) {
    loop {
        if let Err(e) = runner.run().await {
            panic!("[ble_task] error: {:?}", e);
        }
        debug!("[ble_task] runner gave up; launching another");     // tbd. when does this happen; gain understanding!!
    }
}

// An advertiser to connect to a BLE Central    <-- tbd. better comment, once works?
async fn advertise<'a, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
) -> Result<Connection<'a>, BleHostError<C::Error>> {

    let mut buf = [0; 31];      // outside scope so lifespan lasts
    let adv_data: &[u8] = {
        AdStructure::encode_slice(
            &[
                AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                AdStructure::CompleteLocalName(name.as_bytes()),
            ],
            &mut buf,
        )?;
        &buf[..]
    };

    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data,
                scan_data: &[],
            },
        )
        .await?;

    info!("[adv] advertising");
    let conn = advertiser.accept().await?;

    info!("[adv] connection established");
    Ok(conn)
}

// Stream BLE(?) events until the connection closes.
//
async fn gatt_events_task(server: &Server<'_>, conn: &Connection<'_>) -> Result<(), Error> {

    loop {
        match conn.next().await {
            ConnectionEvent::Disconnected { reason } => {
                debug!("[gatt] disconnected: {:?}", reason);
                break;
            }
            ConnectionEvent::Gatt { data } => {

                // Process the event in the GATT server.
                match data.process(server).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("[gatt] error processing: {:?}", e);
                        break;
                    }
                }
            }
        }
    }
    debug!("[gatt] task finished");
    Ok(())
}
