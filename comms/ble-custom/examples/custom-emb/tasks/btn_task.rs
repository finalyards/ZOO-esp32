/*
* Translates the BOOT button (from the devkit) to an await-able 'Signal'.
*/
use defmt::{debug, Format, Formatter, write};

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,   // tbd. or can we use 'NoopRawMutex'; what are the selection criteria?  #later
    signal::Signal
};
use esp_hal::gpio::Input;

pub type BtnSignal = Signal<CriticalSectionRawMutex, ButtonState>;

pub static BTN_SIGNAL: BtnSignal = Signal::new();

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

    fn read(pin: &Input) -> Self {
        if pin.is_low() { ButtonState::Pressed }
        else { ButtonState::Depressed }
    }
}

impl Format for ButtonState {
    fn format(&self, f: Formatter) {
        write!(f, "{}", match self {
            ButtonState::Pressed => "pressed",
            ButtonState::Depressed => "not pressed",
        });
    }
}

/*
* Use by:
*   <<
*       spawner.spawn(btn_task(BOOT))
*           .unwrap();
*   <<
*/
#[embassy_executor::task]
pub async fn btn_task(mut pin: Input<'static>) -> ! {
    let mut first = true;

    loop {
        let state = ButtonState::read(&pin);

        BTN_SIGNAL.signal(state);
        debug!("{}: -> {}",
            if first { "Initial state" } else { "Change detected" },
            state
        );
        first = false;

        pin.wait_for_any_edge() .await;
    }
}
