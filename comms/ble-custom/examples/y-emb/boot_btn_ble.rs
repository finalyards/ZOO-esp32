/*
* BLE specifics of the BOOT button.
*/
#[allow(unused_imports)]
use defmt::{error, info, debug};
use trouble_host::prelude::{gatt_service, descriptors};

use crate::{
    boot_btn_task::{
        ButtonState,
    },
    BTN_SIGNAL
};

include!("./config.in");
    // BB_SERVICE_UUID
    // BB_STATE_CTIC_UUID

// Boot button service
//
// An observable value (0 = depressed; 1 = pressed). This _could_ be a measurement that is received
// by your code, and exposed to the BLE central.
//
#[gatt_service(uuid = BB_SERVICE_UUID)]
//#[gatt_service(uuid = Uuid::Uuid128(BB_SERVICE_UUID.to_be_bytes()))]    // |!|
pub(crate) struct BtnService {
    #[characteristic(uuid = BB_STATE_CTIC_UUID, read, notify)]
    //#[characteristic(uuid = Uuid::Uuid128(BB_STATE_CTIC_UUID.to_be_bytes()), read, notify)]
        #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, read, value = "State of the BOOT button (1 = pressed)")]
        #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 1])]
    pub(crate) /*<-- for now*/ state: bool,
}
    // |!| CONVERSIONS:
    //
    // For now, use without conversions gives:
    //  <<
    //      the trait `From<u128>` is not implemented for `Uuid`
    //  <<
    //
    // See -> https://github.com/embassy-rs/trouble/issues/248

pub async fn boot_btn_feed() -> bool {
    match BTN_SIGNAL.wait() .await {
        ButtonState::Pressed => { true },
        ButtonState::Depressed => { false },
    }
}
