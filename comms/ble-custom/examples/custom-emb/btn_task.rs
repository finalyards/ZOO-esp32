/*
* Awaits BOOT button press, and publishes those to a 'Signal'.
*
* Note: 'Signal' (vs. Channel) means values may get lost; only the last state is of interest.
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

impl ButtonState {
    pub fn is_pressed(&self) -> bool { match self {
        ButtonState::Pressed => true,
        ButtonState::Depressed => false
    }}

    fn from_pressed(pressed: bool) -> Self {
        if pressed { ButtonState::Pressed }
        else { ButtonState::Depressed }
    }
}

#[embassy_executor::task]
pub async fn btn_task(mut pin: /*move*/ Input<'static> , signal: &'static BtnSignal) -> ! {

    loop {
        pin.wait_for_any_edge() .await;
        debug!("Change detected: -> {}", if pin.is_low() { "low" } else { "hi" } );

        let st: ButtonState = ButtonState::from_pressed( pin.level() == Level::Low );
        signal.signal(st);
    }
}
