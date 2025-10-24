/*
* Steer the built-in LED, for showing state.
*
* We create states, with their own little animations, and let the outside world change the visual
* style by a 'Signal'.
*/
use defmt::debug;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,   // tbd. or can we use 'NoopRawMutex'; what are the selection criteria?  #later
    signal::Signal
};
use esp_hal::{
    gpio::{AnyPin, Level},
    peripherals::RMT,
    rmt::{Rmt, TxChannelConfig, TxChannelCreator},
    time::{Instant, Rate},
};

use ble_custom::rgb_led::RgbLed as SLed;
use palette::{Srgb, named::*};

// Way to steer
pub static LED_SIGNAL: Signal<CriticalSectionRawMutex, LedState> = Signal::new();

//const CYCLE_MS = 3_000;   // tbd.

#[derive(Copy, Clone)]
#[derive(defmt::Format)]
pub enum LedState {
    Off,
    State1,
    State2,
    State3,
}

impl LedState {
    fn as_color(&self) -> Srgb<u8> {
        match self {
            Self::Off => Srgb::default(),
            Self::State1 => RED,
            Self::State2 => GREEN,
            Self::State3 => BLUE,

            //Self::State1 => RGB{ r:255, g:128, b:128 },
            //Self::State2 => RGB{ r:128, g:255, b:128 },
            //Self::State3 => RGB{ r:128, g:128, b:255 }

            //Self::State1 => RGB{ r:255, g:255, b:255 },
            //Self::State2 => RGB{ r:128, g:128, b:128 },
            //Self::State3 => RGB{ r:64, g:64, b:64 }
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
pub async fn led_task(#[allow(non_snake_case)] p_RMT: RMT<'static>, pin: AnyPin<'static>) -> ! {

    //#[cfg(feature = "esp32h2")]
    //compile_error!("Not prepared for this MCU.");
    const FREQ: Rate = Rate::from_mhz(80);

    let rmt = Rmt::new(p_RMT, FREQ)
        .unwrap()
        .into_async();

    let channel = rmt
        .channel0
        .configure_tx(
            pin,
            TxChannelConfig::default()
                .with_clk_divider(4)        // -> 20MHz
                .with_idle_output_level(Level::Low)   // between pulses, steering is low
                .with_carrier_modulation(false)
                .with_idle_output(true)
        )
        .unwrap();

    // NOTE: BE CAREFUL WITH BRIGHTNESS!!! The RGB LED is VERY POWERFUL, since there's no resistor
    //      and no diffuser in the devkits.
    //
    //      HINT! Use at most 0.10 (10%); add a plastic dome on top of the LED; DO NOT LOOK directly
    //          into it. Were shades.
    //
    const BRIGHTNESS: f32 = 4_f32 * 0.01;

    let mut sled = SLed::new_with_channel(channel, BRIGHTNESS);

    let mut st: LedState = LedState::Off;

    loop {
        let _t0 = Instant::now();

        sled.set( &st.as_color() ) .await;

        debug!("Setting the LED took {}", _t0.elapsed());    // DEBUG; tbd. collect
            // sync:
            // async: 97..102 us

        #[cfg(true)]
        {
            st = LED_SIGNAL.wait().await;
        }

        #[cfg(false)]
        {
            st = match st {
                LedState::State1 => LedState::State2,
                _ => LedState::State1
            };
            Timer::after_millis(200).await;
        }
    }
}
