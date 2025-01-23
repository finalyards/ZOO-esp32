/*
* Awaits BOOT button presses (available on common devkits), and publishes those to a channel.
*/
#[allow(unused_imports)]
use defmt::{debug};

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    signal::Signal
};
use esp_hal::gpio::{Input, Level};

pub type BtnSignal = Signal<CriticalSectionRawMutex, ButtonState>;

#[derive(Copy, Clone)]
pub enum ButtonState {
    Pressed,
    Depressed
}

// Note: Once generators are available in embedded Rust, this could be one.
//      -> https://dev-doc.rust-lang.org/beta/std/ops/trait.Generator.html
//
#[embassy_executor::task]
pub async fn btn_task(mut pin: /*move*/ Input<'static> , signal: &'static BtnSignal) {

    loop {
        pin.wait_for_any_edge() .await;
        debug!("Change detected: -> {}", if pin.is_low() { " low" } else { "hi" } );

        let st: ButtonState = match pin.level() {
            Level::Low => ButtonState::Pressed,
            Level::High => ButtonState::Depressed
        };
        signal.signal(st);
    }
}
