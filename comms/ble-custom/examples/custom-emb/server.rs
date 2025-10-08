/*
* GATT server
*/
#[allow(unused_imports)]
use defmt::{info, debug, warn, error, panic};

use embassy_futures::{
    join::join,
    select::{select, select_array}
};

use rand_core::{RngCore, CryptoRng};
use trouble_host::prelude::*;

use crate::{
    BleConnector,
    btn_gatt::BtnService
};

const NAME: &'static str = "ZOO";               // tbd. where does this show?
const AD_NAME: &'static str = "custom example";  // advertised name

const CONNECTIONS_MAX: usize = 1;       // max nbr of connections
const L2CAP_CHANNELS_MAX: usize = 2;    // max nbr of L2CAP channels    // tbd. pls explain...

#[gatt_server]
pub(crate) struct Server {
    btn_service: BtnService,
}
    // Expands to:
    //  <<
    //      pub struct Server<'values> {
    //          pub server: trouble_host::prelude::AttributeServer<
    //              'values,
    //              embassy_sync::blocking_mutex::raw::NoopRawMutex,
    //              trouble_host::prelude::DefaultPacketPool,
    //              _ATTRIBUTE_TABLE_SIZE,      // trouble_host::gap::GAP_SERVICE_ATTRIBUTE_COUNT + BtnService::ATTRIBUTE_COUNT
    //              _CCCD_TABLE_SIZE,           // 0 + BtnService::CCCD_COUNT
    //              _CONNECTIONS_MAX,           // 1
    //          >,
    //          [...]   // services
    //      }
    //  <<
    //

impl Server<'_> {
    /*
    * Entry from 'main'; launches the server and pumps it.
    *
    * This side does 'trouble' specific things.
    */
    pub(crate) async fn run<RNG>(ble_controller: BleConnector<'_>, a: Address, mut trng: RNG) -> !
    where
        RNG: RngCore + CryptoRng,
    {
        use trouble_host::prelude::{
            HostResources,
        };

        let controller = ExternalController::<_,20 /*SLOTS*/>::new(ble_controller);

        let (mut ress, stack);   // for lifespan

        let Host {
            peripheral,
            runner,
            ..
        } = {
            ress = HostResources::<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX>::new();

            stack = trouble_host::new(controller, &mut ress)
                .set_random_address(a)
                .set_random_generator_seed(&mut trng);
            stack.build()
        };

        let gs = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
            name: NAME,
            appearance: &appearance::sensor::GENERIC_SENSOR,    // tbd. document what it affects
        }))
            .unwrap();

        run2/*::<_,DefaultPacketPool>*/(gs, peripheral, runner) .await;
    }
}

// Run the BLE stack.
//
async fn run2<'a,C/*,P*/>(gs: Server<'a>, mut peripheral: Peripheral<'a,C,DefaultPacketPool/*P*/>, mut runner: Runner<'a,C,DefaultPacketPool/*P*/>) -> !
    where C: Controller, //P: PacketPool, M: RawMutex
{
    // Note: Copilot says that 'defmt' logging may have problems with 'async' blocks. If so,
    //      use 'async fn'. "This gives better type inference, IDE support, and stack traces." //their words

    let loop1 = async { loop {
        if let Err(e) = runner.run().await {
            let e = defmt::Debug2Format(&e);
            panic!("[ble_task] error: {:?}", e);
        }
        debug!("[ble_task] runner gave up; launching another");     // tbd. when does this happen
    }};

    let Server{ ref btn_service, .. } = gs;

    let loop2 = async { loop {
        debug!("Starting advertising");

        match advertise(AD_NAME, &mut peripheral, &gs).await {
            Ok(conn) => {
                let a = gatt_events_until_disconnect(&gs, &conn);

                let bs = [
                    btn_service.notify_runner(&conn)
                ];

                // Pump them, until one ends ('gatt_events_until_disconnect').
                select(a, select_array(bs) ).await;
            }
            Err(e) => {
                let e = defmt::Debug2Format(&e);
                panic!("caught: {:?}", e);
            }
        }
    }};

    join(loop1, loop2) .await;

    unreachable!();
}

// An advertiser to connect to a BLE Central    <-- tbd. better comment, once works?
async fn advertise<'values, 'server, C: Controller /*, P: PacketPool, M: RawMutex*/>(
    name: &'values str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool/*P*/>,
    gs: &'server Server<'values>
) -> Result<GattConnection<'values, 'server, DefaultPacketPool/*P*/>, BleHostError<C::Error>> {

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

    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data,
                scan_data: &[],
            },
        )
        .await?;

    // Below - IF we were to bring 'P: PacketPool' as generic.
    //  <<
    //   note: expected reference `&AttributeServer<'_, _, P, _, _, _>`
    //                found reference `&gatt_server::Server<'values>`
    //  <<
    // Not sure why this doesn't work (no conversion from 'Server' to '&AttributeServer');
    // 'trouble' examples has it, and it works, there.
    //
    debug!("[adv] advertising...");

    let conn: GattConnection<DefaultPacketPool> = advertiser.accept().await?
        .with_attribute_server(&gs)?;

    debug!("[adv] connection established");
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
