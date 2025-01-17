use trouble_host::prelude::*;

use crate::MyC;

// RGB service
//
#[gatt_service(uuid = "438785e7-4942-4749-a072-dceb73fd6c87")]  // RGB_SERVICE_UUID
pub struct RgbService {
    // tbd. What happens if we don't do 'on_read'/'on_write'?  What are they for?
    //#[characteristic(uuid = "448785e7-4942-4749-a072-dceb73fd6c87", write, on_write = rgb_on_write)]    // CHAR_RGB_UUID
    #[characteristic(uuid = "448785e7-4942-4749-a072-dceb73fd6c87", write, value = [0,0,0x40])]    // CHAR_RGB_UUID
    rgb: [u8;3],
}

/*** disabled
// Q: What's the relation of these '[...]_on_{read|write}' to the 'conn_task'???
fn rgb_on_write(_conn: &Connection, data: &[u8]) -> Result<(), ()> {
    info!("[gatt] Write event on RGB: {:?}", data);

    // tbd. how to see 'rgb', from here???
    Ok(())
}
***/
