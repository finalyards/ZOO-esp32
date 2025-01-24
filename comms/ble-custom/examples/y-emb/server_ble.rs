/*
* BLE server
*/
#[allow(unused_imports)]
use defmt::{info, debug, warn, error};

use embassy_futures::{join, select};
use esp_hal::efuse::Efuse;
use trouble_host::prelude::*;

use crate::boot_btn_ble::BtnService;

const CONNECTIONS_MAX: usize = 1;   // max nbr of connections
const L2CAP_CHANNELS_MAX: usize = 2; // max nbr of L2CAP channels (Signal + att)

const L2CAP_MTU: usize = 255;   // all ESP32's are fine with this length; see -> https://github.com/esp-rs/esp-hal/issues/2984

include!("./config.in");
    // AD_NAME

#[gatt_server]
pub struct Server {
    bb: BtnService,
}

impl Server<'_> {
    // Run the BLE stack.
    //
    pub async fn run<C>(controller: C) -> !
    where
        C: Controller,
    {
        // Using a fixed address can be useful for testing.
        #[cfg(not(all()))]
        let address: Address = Address::random(b"rand0m".into());
        #[cfg(all())]
        let address: Address = Address::random(Efuse::mac_address());  // 6 bytes MAC

        info!("Our address = {:?}", address.addr);   // tbd. in hex

        // here for the lifespan
        let mut ress;
        let stack;

        let Host {
            mut peripheral,
            runner, ..
        } = {
            ress = HostResources::<CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>::new();
            stack = trouble_host::new(controller, &mut ress)
                .set_random_address(address);
            stack.build()
        };

        info!("Starting advertising and GATT service");
        let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
            name: AD_NAME,
            appearance: &appearance::UNKNOWN,
        }))
            .unwrap();

        let _ = join::join(ble_task(runner), async {
            loop {
                match advertise("Trouble Example", &mut peripheral).await {
                    Ok(conn) => {
                        // These tasks are only run while there is a connection.
                        let a = gatt_events_task(&server, &conn);

                        let bs = select::select_array([
                            server.bb.notify_task(&server, &conn)
                            //R any_notify( boot_btn_feed, &server.bb.state, &server, &conn )
                        ]);

                        // Run until any task ends (usually 'gatt_events_task', due to the connection
                        // being closed); then return to advertising state.
                        select::select(a, bs).await;
                    }
                    Err(e) => {
                        panic!("caught: {:?}", e);
                    }
                }
            }
        });
        unreachable!()
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

/***R
async fn any_notify<X, Fut>(feed: fn() -> Fut, ctic: &Characteristic<X>, server: &Server<'_>, conn: &Connection<'_>) -> !
where X: GattValue,
    Fut: Future<Output = X>
{
    loop {
        let x: X = feed() .await;
        ctic.notify(server, conn, &x) .await
            .expect("notification to work")
    }
}***/

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
    let ctic_bb = server.bb.state;

    loop {
        match conn.next().await {
            ConnectionEvent::Disconnected { reason } => {
                debug!("[gatt] disconnected: {:?}", reason);
                break;
            }
            ConnectionEvent::Gatt { data } => {

                // To simplify things, process the event in the GATT server; match on its output.
                match data.process(server).await {
                    // Server processing emits
                    Ok(Some(GattEvent::Read(event))) => {
                        if event.handle() == ctic_bb.handle {
                            let value = server.get(&ctic_bb);
                            debug!("[gatt] Read event of BB Characteristic: {:?}", value);
                        }
                    }
                    /***
                    Ok(Some(GattEvent::Write(event))) => {
                        if event.handle() == level.handle {
                            info!("[gatt] Write event to BB Characteristic: {:?}", event.data());
                        }
                    }
                    ***/
                    Ok(_) => {
                        warn!("[gatt] unexpected event (skipped)");
                    }
                    Err(e) => {
                        error!("[gatt] error processing event: {:?}", e);
                        break;
                    }
                }
            }
        }
    }
    debug!("[gatt] task finished");
    Ok(())
}
