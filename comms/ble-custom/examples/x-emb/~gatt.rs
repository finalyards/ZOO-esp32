use ble_custom::IsGattServer;

// Custom service(s)
//
// Each service should cater for one profile of the device. Below, having 'magic' and 'rgb' together
// is __artificial__ since they do not rely on each other! In reality, one would rather place them
// in different services.
//
// The '#[gatt_service(...)]' macro creates a plain 'struct' that is _not_ of any particular type.
//
#[gatt_service(uuid = "438785e7-4942-4749-a072-dceb73fd6c87")]  // SERVICE_UUID
struct MyService {
    // tbd. What happens if we don't do 'on_read'/'on_write'?  What are they for?
    //#[characteristic(uuid = "448785e7-4942-4749-a072-dceb73fd6c87", write, on_write = rgb_on_write)]    // CHAR_RGB_UUID
    #[characteristic(uuid = "448785e7-4942-4749-a072-dceb73fd6c87", write, value = [0,0,0x40])]    // CHAR_RGB_UUID
    rgb: [u8;3],
    //#[characteristic(uuid = "458785e7-4942-4749-a072-dceb73fd6c87", read, notify, on_read = magic_on_read)]     // CHAR_MAGIC_UUID
    #[characteristic(uuid = "458785e7-4942-4749-a072-dceb73fd6c87", read, notify)]     // CHAR_MAGIC_UUID
    magic: u32
}

/*** disabled
// Q: What's the relation of these '[...]_on_{read|write}' to the 'conn_task'???
fn magic_on_read(_conn: &Connection) {
    info!("[gatt] Read event on magic");

    // tbd. how to see 'magic', from here???
    warn!("Unimplemented!");
}

fn rgb_on_write(_conn: &Connection, data: &[u8]) -> Result<(), ()> {
    info!("[gatt] Write event on RGB: {:?}", data);

    // tbd. how to see 'rgb', from here???
    Ok(())
}
***/

/*
* Collection of the 1..n services
*
* The '#[gatt_server(...)]' macro creates a plain 'struct' that is _not_ of any particular type.
*/
#[gatt_server]
struct Server {
    my_service: MyService,
}
