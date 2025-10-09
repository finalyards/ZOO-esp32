/*
* BLE specifics of the BOOT button.
*/
#[allow(unused_imports)]
use defmt::{error, info, debug};

use trouble_host::prelude::*;

use static_cell as _;   // 'host-macros' needs it

use crate::{
    BTN_SIGNAL,
};

const BTN_SERVICE_UUID: u128 = 0x_92996405_8C0E_4FA1_A417_67D36995B563;
const BTN_STATE_CHARACTERISTIC_UUID: u128 = 0x_719ba6d4_6bad_4ed5_bd4d_7e64d184d8bf;

// Boot button service
//
// An observable value (0 = depressed; 1 = pressed). This _could_ be a measurement that is received
// by your code, and exposed to the BLE central.
//
#[gatt_service(uuid = BTN_SERVICE_UUID)]
pub(crate) struct BtnService {
    #[characteristic(uuid = BTN_STATE_CHARACTERISTIC_UUID, read, notify)]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, read, value = "State of the BOOT button (1 = pressed)")]
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 1])]
    pub pressed: bool,
}

impl BtnService {
    // Called 'custom_task' in 'tro(u)ble' examples.
    //
    pub async fn notify_runner<P: PacketPool>(&self, conn: &GattConnection<'_, '_, P>) -> ! {
        let c12c = self.pressed;
        loop {
            let x: bool = BTN_SIGNAL.wait() .await .is_pressed();

            c12c.notify(conn, &x) .await
                .expect("notification to work");
        }
    }
}
