/*
* Awaits BOOT button presses (available on common devkits), and publishes those to a channel.
*/
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    signal::Signal
};

pub enum ButtonState {
    Pressed,
    Depressed
}

pub fn listen() {

}
