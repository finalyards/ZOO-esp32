use trouble_host::prelude::*;

use crate::MyC;

include!("config.in");

// Custom data updates; could be e.g. measurement data
//
pub async fn svc_magic_task(server: &Server<'_, '_, MyC>, conn: &Connection<'_>) {
    let mut tick: u32 = 0;
    let mgk = server.my_service.magic;
    loop {
        tick = tick.wrapping_add(1);

        info!("[adv] notifying connection of change in 'magic' {}", tick);
        server.notify(&mgk, conn, &tick).await
            .map_err(|e| {
                info!("[adv] error notifying connection");
                break;
            });

        Timer::after_secs(1).await;
    }
}

// Magic service
//
#[gatt_service(uuid = "92996405-8C0E-4FA1-A417-67D36995B563")]  // MAGIC_SERVICE_UUID
pub struct MagicService {
    //#[characteristic(uuid = "93996405-8C0E-4FA1-A417-67D36995B563", read, notify, on_read = magic_on_read)]     // CHAR_MAGIC_UUID
    #[characteristic(uuid = "93996405-8C0E-4FA1-A417-67D36995B563", read, notify)]     // CHAR_MAGIC_UUID
    magic: u32
}

/*** disabled
// Q: What's the relation of these '[...]_on_{read|write}' to the 'conn_task'???
fn magic_on_read(_conn: &Connection) {
    info!("[gatt] Read event on magic");

    // tbd. how to see 'magic', from here???
    warn!("Unimplemented!");
}
***/
