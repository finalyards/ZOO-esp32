//!
//! lib.rs
//!
//! API for using the VL53L5CX
//!
#![no_std]

/*** #later
//R pub mod api;
mod result;

pub use result::Result;

pub use vl53l5cx_uld::{PowerMode, TargetOrder};

use defmt::debug;
use defmt_rtt as _;

/*
* Interface to a single L{7} sensor
*/
pub struct VL53L5CX {
    //
}

impl VL53L5CX {
    // tbd. initialize the I2C within here; the ULD doesn't need to know about the address of that
    //      chip (does it?)
    // tbd. pin (or lack thereof) for catching INT
    pub fn new() -> Self {
        Self{}
    }

    pub fn is_alive(&self) -> bool {
        unimplemented!()
    }

    // Q: Do we want to have an 'init', separate from 'new'?
    #[cfg(considering)]
    pub fn init(&self) -> Result {
        unimplemented!()
    }

    pub fn start_ranging() -> RangingState {
        unimplemented!()
    }

    //--- Auxiliary functions
    //
    // tbd. Could be in a separate file, and 'impl super::VL53L5CX'
    pub fn get_info() -> GeneralInfo {
        unimplemented!()
    }
}

// Use this with defaults tbd.
struct DeviceConfig {
    power_mode: PowerMode,      // default is "wake-up"
    sharpener_prc: u8,
    target_order: TargetOrder,
    #[allow(non_snake_case)]
    VHV_repeat_count: u8
}

struct RangingState {

}

impl RangingState {
    pub fn with_resolution(&mut self, e: Resolution) -> {

    }

    // "'set_resolution()' must be called before 'set_ranging_frequency_hz()'"
    //
    pub fn with_ranging_freq_hz(&mut self, hz: u8) -> {

    }

    // NOTE: "Integration time is [...] only available in AUTONOMOUS ranging mode".
    //      Thus, we could represent it in the 'RangingParameters', for autonomous_custom.
    // vl53l5cx_set_integration_time_ms()
    //
    pub fn with_ranging_mode(&mut self, e: RangingMode) {

    }

    // Not necessarily even exposing 'check_data_ready()' - we can hide the complexity
    //  - just do 'async'.
    //
    #[cfg(disabled)]    // tbd. consider
    pub fn poll_are_we_there_yet() -> bool {
        // v7_check_data_ready()
    }

    pub fn get_data() -> RangingData {
        // v7_getranging_data()
    }

    pub fn stop() {
        // v7_stop_ranging()
    }
}

// Use this with defaults tbd.
struct RangingConfig {
    pub resolution: Resolution,
    freq_hz: u8,        // default: 1 tbd.
    mode: RangingMode
}

pub struct ResultData {

}

pub struct GeneralInfo {
    #[allow(non_snake_case)]
    API_REVISION: (),     // version of the ULD host side C driver
}
***/
