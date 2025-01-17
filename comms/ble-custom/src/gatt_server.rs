/*
* Wrap around any '#[gatt_server]' struct
*/
trait XGattServer {
    fn new_with_name(name: &str) -> Result<Self,Error>;

    fn ctic_by_handle(&mut self, handle: u16) -> Option<&mut MagicService>;
}
