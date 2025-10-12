/*
* Steer the built-in LED, for showing state.
*
* Use of the smart LED adapted from
*   -> https://github.com/esp-rs/esp-hal-community/tree/main/esp-hal-smartled/examples
*
* FUTURE PLANS:
*   One could create a pulse sequence ahead of time, and pass it to RMT. This way, the peripheral
*   is in charge of changing the lights, not this (CPU) script. THIS IS WORTH EXPERIMENTING, and
*   it would not change this file's interface. #help
*
* IMPLEMENTATION NOTE:
*   The code uses async 'smart_led' API ('SmartLedsAdapterAsync'). It's not clear, whether this
*   provides any tangible benefit, when only a single (smart) LED is being steered.
*
*   Try making this both blocking and async, once the above animation has been implemented. Measure
*   delays and make a suggestion, which one to use.
*
*   For reference, see `robamu`'s comment (Oct 2024) |1|:
*
*       > Writing all 46 LEDs with async API takes around 3-4 ms, blocking API takes ~2 ms
*
*       |1|: https://github.com/esp-rs/esp-hal-community/issues/4#issuecomment-2408920933
*/
use defmt::{debug, write};
use static_assertions::const_assert;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,   // tbd. or can we use 'NoopRawMutex'; what are the selection criteria?  #later
    signal::Signal
};
use esp_hal::{
    gpio::AnyPin,
    peripherals::RMT,
    rmt::{PulseCode, Rmt},
    time::{Instant, Rate},
};
use esp_hal_smartled::{
    buffer_size_async,
    SmartLedsAdapterAsync,
};

use smart_leds::{
    brightness,
    colors,
    gamma,
    RGB8,
    SmartLedsWriteAsync,
};

// Way to steer
pub static LED_SIGNAL: Signal<CriticalSectionRawMutex, LedState> = Signal::new();

// !! WARNING !!
//      Even the '10' used in 'esp-hal-smartled' samples feels WAY TOO BRIGHT for the author:
//      Recommended:
//          - do NOT USE VALUES >= 10
//          - stick a diffusing, transparent or white rubber on the devkit's "smart LED"; the
//              light is VERY POINTY without, which is BAD FOR ONE'S EYES. These devices are intended
//              to be used with a diffuser, through a sheet of plastic, or something.
//          - do NOT look straight in the light!
//
const STRENGTH: u8 = 2;    // ..255

//const CYCLE_MS = 3_000;   // tbd.

const_assert!(STRENGTH <= 10);

#[derive(Copy, Clone)]
#[derive(defmt::Format)]
pub enum LedState {
    Off,
    State1,
    State2,
}

impl LedState {
    fn as_rgb(&self) -> RGB8 {
        match self {
            Self::Off => RGB8::default(),
            Self::State1 => colors::GREEN,
            Self::State2 => colors::RED,
        }
    }
}

impl Default for LedState {
    fn default() -> Self {
        LedState::Off
    }
}

/*
* Use by:
*   <<
*       spawner.spawn(led_task(rmt: RMT, ...))
*           .unwrap();
*   <<
*/
#[embassy_executor::task]
#[allow(non_snake_case)]
pub async fn led_task(p_RMT: RMT<'static>, pin: AnyPin<'static>) -> ! {

    // Configure RMT (Remote Control Transceiver) globally
    // <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/rmt.html>
    let rmt: Rmt<'_, esp_hal::Async> = {
        let freq: Rate = Rate::from_mhz(80);    // not for ESP32-H2; use 32
        Rmt::new(p_RMT, freq)
    }
        .unwrap()
        .into_async();

    // `SmartLedsAdapterAsync` implements the 'SmartLedsWriteAsync' trait |1| which is hw agnostic.
    //  |1|: https://github.com/smart-leds-rs/smart-leds-trait/blob/master/src/lib.rs
    //
    let rmt_buf = [PulseCode::default(); buffer_size_async(1)];
    let mut smart_led = {
        SmartLedsAdapterAsync::<25>::new(rmt.channel0, pin, rmt_buf)
    };

    let mut st: LedState = LedState::Off;

    // Developer note:
    //  - 'brightness' and 'gamma' come from the 'smart-leds' crate, and are rather simple wrappers
    //      around iterators.
    //  - '.write' is from 'esp-hal-smartled'; it converts RGB values to PulseCode's and transmits
    //      them, in chunks, to the RTM. If '.await', there's possibility to do something between
    //      the chunk transmits.
    //
    loop {
        let t0 = Instant::now();

        let x = brightness( gamma([st.as_rgb()].into_iter()), STRENGTH ) ;
        smart_led.write(x) .await
            .unwrap();

        debug!("Setting the LED took {:ms}", t0.elapsed());     // 0.192 us

        st = LED_SIGNAL.wait() .await;
    }
}
