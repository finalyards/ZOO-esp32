/*
* Accessing the BOOT button (attached to GPIO9), with built-in debounce filtering.
*/
use esp_hal::{
    delay::Delay,
    prelude::*,
    time::now
};

const PRESS_DEBOUNCE_MS: u16 = 20;  // tbd. make these time constants
const STATE_DEBOUNCE_MS: u16 = 10;

struct BootButton {
    h: Input
}

impl BootButton {

    fn new(h: Input) -> Self {
        Self { h }
    }

    /*
    * Complete once the button has been in 'state' for 'PRESS_DEBOUNCE' time (starting at the call).
    */
    async fn wait_until(&self, state: BBState) {
        unimplemented!();
        if self.h.is_high()

    }

    /*
    * Return the state of the button, once it has been unchanged for 'STATE_DEBOUNCE' time.
    */
    async fn state(&self) -> BBState {
        unimplemented!()
    }
}

#[derive(Copy, Clone)]
enum BBState {
    Pressed,
    Depressed,
}

impl From<bool> for BBState {
    fn from(state: bool) -> Self {
        match state {
            false => BBState::Pressed,
            true => BBState::Depressed,
        }
    }
}

impl core::ops::Not for BBState {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::Depressed => Self::Pressed,
            Self::Pressed => Self::Depressed,
        }
    }
}