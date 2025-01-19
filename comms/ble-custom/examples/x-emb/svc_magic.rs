use defmt::{error, info, debug};

use embassy_time::Timer;
use trouble_host::{
    prelude::*,
};

use crate::{
    MyC,
};

include!("./config.in");
// MAGIC_SERVICE_UUID
// CHAR_MAGIC_UUID

// Magic service
//
// An observable value (a counter) that gets updated by a task, regularly. This _could_ be a
// measurement that is received e.g. interrupt-based, and exposed to the BLE central.
//
#[gatt_service(uuid = "92996405-8C0E-4FA1-A417-67D36995B563")]  // MAGIC_SERVICE_UUID tbd.
pub struct MagicService {
    #[characteristic(uuid = "93996405-8C0E-4FA1-A417-67D36995B563", read, notify)]     // CHAR_MAGIC_UUID
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, read, value = "Magic (increasing) value")]
    magic: u32,
}

// Custom data updates; could be e.g. measurement data
//
impl MagicService {

    /*
    * Started at the beginning of a connection; dropped at the end of one.
    */
    pub async fn task_when_connected<S>(&self, conn: &Connection<'_>, srv: &S) {
        let mut value: u32 = 0;
        let ctic_magic = &self.magic;
        loop {
            Timer::after_secs(1).await;

            value = value.wrapping_add(1);
            debug!("[magic] value changed to: {}", value);

            match ctic_magic.notify(srv, conn, &value).await {
                Err(e) => {
                    error!("[magic] error notifying connection: {}", e);
                    break;  // exits the task
                },
                Ok(()) => {}
            };
        }
    }
}