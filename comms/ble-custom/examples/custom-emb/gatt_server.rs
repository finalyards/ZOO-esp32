/*
* BLE server
*/
#[allow(unused_imports)]
use defmt::{info, debug, warn, error, panic};

use embassy_futures::{
    join::join,
    select::{select, select_array}
};
use trouble_host::prelude::*;

use crate::btn_gatt::BtnService;

include!("./config.in");
    // AD_NAME
    // AD_NAME2

#[gatt_server]
pub struct Server {
    btn_service: BtnService,
}

// Run the BLE stack.
//
pub async fn run<'a,C/*,P*/>(host: Host<'a,C,/*P*/ DefaultPacketPool>) -> !
    where C: Controller, /*P: PacketPool*/
{
    let Host {
        mut peripheral,
        runner, ..
    } = host;

    debug!("Starting GATT server");

    let srv = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: CONFIG.NAME,
        appearance: &appearance::UNKNOWN,
    }))
        .unwrap();

    let _ = join(ble_task(runner), async {
        loop {
            debug!("Starting advertising");

            match advertise(CONFIG.AD_NAME, &mut peripheral, &srv).await {
                Ok(conn) => {
                    let a = gatt_events_until_disconnect(&srv, &conn);

                    let bs = [
                        srv.btn_service.notify_runner(&conn)
                    ];

                    // Pump them, until one ends ('gatt_events_until_disconnect').
                    select(a, select_array(bs) ).await;
                }
                Err(e) => {
                    let e = defmt::Debug2Format(&e);
                    panic!("caught: {:?}", e);
                }
            }
        }
    }).await;

    unreachable!();
}

// tbd. comment
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            let e = defmt::Debug2Format(&e);
            panic!("[ble_task] error: {:?}", e);
        }
        debug!("[ble_task] runner gave up; launching another");     // tbd. when does this happen
    }
}

// An advertiser to connect to a BLE Central    <-- tbd. better comment, once works?
async fn advertise<'values, 'server, C: Controller /*, P: PacketPool*/>(
    name: &'values str,
    peripheral: &mut Peripheral<'values, C, /*P*/ DefaultPacketPool>,
    srv: &'server Server<'values>
) -> Result<GattConnection<'values, 'server, /*P*/ DefaultPacketPool>, BleHostError<C::Error>> {

    let mut buf = [0; 31];      // outside for lifespan
    let adv_data: &[u8] = {
        let len = AdStructure::encode_slice(
            &[
                AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                AdStructure::CompleteLocalName(name.as_bytes()),
            ],
            &mut buf,
        )?;
        &buf[..len]
    };

    let advertiser: Advertiser<C,DefaultPacketPool> = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data,
                scan_data: &[],
            },
        )
        .await?;

    info!("[adv] advertising");
    let conn = advertiser.accept().await?
        .with_attribute_server(srv)?;

    info!("[adv] connection established");
    Ok(conn)
}

// Stream BLE(?) events until the connection closes.
//
// This handles the 'read' and 'write' of different services.
//
// tbd. Consider providing a 'HashMap'-based resolver that gets a handle and whether it's read/write,
//      then does the server-thing without us needing to know the internals.
//
async fn gatt_events_until_disconnect<P : PacketPool>(srv: &Server<'_>, conn: &GattConnection<'_,'_,P>) -> Result<bt_hci::param::Status, Error> {
    let btn_svc_pressed = srv.btn_service.pressed;

    let why = loop {
        match conn.next().await {
            // tbd. explain
            GattConnectionEvent::Disconnected { reason } => {
                debug!("[gatt] disconnected: {:?}", reason);
                break reason;
            }

            // tbd. explain
            GattConnectionEvent::PairingComplete { security_level, ..} => {
                debug!("[gatt] pairing complete: {:?}", security_level);
            }
            GattConnectionEvent::PairingFailed(err) => {
                warn!("[gatt] pairing error: {:?}", err);
            }

            // tbd. explain
            GattConnectionEvent::Gatt { event } => {
                let /*mut*/ reply_res: Result<Reply<P>,Error> = if !conn.raw().security_level()?.encrypted() {
                    event.reject(AttErrorCode::INSUFFICIENT_ENCRYPTION)

                } else {    // sane security
                    match &event {
                        GattEvent::Read(a) if a.handle() == btn_svc_pressed.handle => {
                            let v = srv.get(&btn_svc_pressed);
                            debug!("[gatt] reading 'bb.pressed': {}", v);
                        },
                        GattEvent::Write(a) if a.handle() == btn_svc_pressed.handle => {
                            debug!("[gatt] writing 'bb.pressed': {:?}", a.data())
                        },
                        _x => {
                            //warn!("[gatt] unexpected gatt event: {:?}", x);     // no 'Format' for '...::GattEvent<..'
                            warn!("[gatt] unknown error)");     // TEMP
                        }
                    }
                    event.accept()
                };

                match reply_res {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("[gatt] error processing event: {:?}", e),
                }
            },
            _ => {}     // ignore other GATT connection events
        }
    };
    debug!("[gatt] disconnected: {:?}", why);
    Ok(why)
}
