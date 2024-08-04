/*
* The actual ULD level API. Manually crafted.
*/
//#![allow(non_camel_case_types)]

use core::ffi::CStr;

use crate::uld_raw as raw;

use raw::VL53L5CX_Configuration;
use raw::VL53L5CX_API_REVISION;     // b"VL53L5CX_2.0.0\0";

pub use {
    raw::{
        VL53L5CX_NB_TARGET_PER_ZONE as NB_TARGET_PER_ZONE,
        VL53L5CX_MAX_RESULTS_SIZE as MAX_RESULTS_SIZE,
        VL53L5CX_FW_NBTAR_RANGING as FW_NBTAR_RANGING,

        RESOLUTION as Resolution,
        TARGET_ORDER as TargetOrder,
        RANGING_MODE as RangingMode,
        POWER_MODE as PowerMode
    }
};

use crate::Result::{self, Ok, Error};

// Consts we decide to expose.
//
// Other than these don't need to be brought even to 'raw' (see 'wrap.h').

/**
* @brief ULD driver version, e.g. "VL53L5CX_2.0.0"
*/
pub const API_REVISION: &str = &CStr::from(raw::VL53L5CX_API_REVISION).to_str();


/**
* Document tbd.
*/
pub fn vl53l5cx_is_alive(cfg: &VL53L5CX_Configuration) -> Result<bool> {
    let mut buf: u8 = 0;    // written 1 (alive) or 0
    match raw::vl53l5cx_is_alive(cfg, &mut buf) {
        ST_OK => Ok(buf != 0),
        st => Error(st)
    }
}

/**
 * @brief Resets the sensor, feeds the firmware and sets it to default parameters.
 */
pub fn vl53l5cx_init(cfg: &mut VL53L5CX_Configuration) -> Result<()> {

    match raw::vl53l5cx_init(cfg) {
        ST_OK => Ok(()),
        st => Error(st)
    }
}

/*** Just saying NOPE.
/**
* @brief Change the I2C address of the receiving chip.
*
* Note: Unlike in the vendor library, it is up to the caller to update the 'cfg' structure, if they
*       maintain the address there.
**/
pub fn vl53lcx_set_i2c_address(cfg: &VL53L5CX_Configuration, addr: I2C_Address) -> Status {

    let st: u8 = raw::vl53l5cx_set_i2c_address(cfg, addr.0 as u16);
    Status::from(st)
}
***/

/**
 * @brief Get the power mode
 **/
pub fn vl53lcx_get_power_mode(cfg: &VL53L5CX_Configuration) -> Result<PowerMode> {
    let buf: u8 = 0;
    match raw::vl53l5cx_get_power_mode(cfg, &buf) {
        ST_OK => Ok( PowerMode::from(buf) ),
        st => Error(st)
    }
}

// tbd.
//vl53l5cx_set_power_mode(cfg,PowerMode) -> Result<()>
//vl53l5cx_start_ranging(cfg) -> Result<()>
//vl53l5cx_stop_ranging(cfg) -> Result<()>
//vl53l5cx_check_data_ready(cfg) -> Result<bool>
//vl53l5cx_get_ranging_data(cfg) -> Result<ResultsData>
//vl53l5cx_get_resolution(cfg) -> Result<Resolution#E>
//vl53l5cx_set_resolution(cfg, Resolution#E) -> Result<()>
//vl53l5cx_get_ranging_frequency_hz(cfg) -> Result<u8>  // num or enum?
//vl53l5cx_get_integration_time_ms(cfg) -> Result<u32>
//vl53l5cx_set_integration_time_ms(cfg,u32) -> Result<())>
//vl53l5cx_get_sharpener_percent(cfg) -> Result<u8>     // 0..100
//vl53l5cx_set_sharpener_percent(cfg,u8) -> Result<()>  // 0..99 allowed
//vl53l5cx_get_target_order(cfg) -> Result<TargetOrder#E>
//vl53l5cx_set_target_order(cfg,TargetOrder#E) -> Result<())>
//vl53l5cx_get_ranging_mode(cfg) -> Result<RangingMode#E>
//vl53l5cx_set_ranging_mode(cfg,RangingMode#E) -> Result<())>
//vl53l5cx_enable_internal_cp(cfg) -> Result<()>
//vl53l5cx_disable_internal_cp(cfg) -> Result<()>
//vl53l5cx_get_VHV_repeat_count(cfg) -> Result<u32>
//vl53l5cx_set_VHV_repeat_count(cfg,u32) -> Result<()>

//vl53l5cx_dci_read_data(cfg,index: u32, bytes: u16) -> Result<array of u8>
//vl53l5cx_dci_write_data(cfg,index: u32, data: &[u8], bytes: u16) -> Result<()>

//vl53l5cx_dci_replace_data // no benefit over separate read + write


//---

impl PowerMode {
    fn from(v: u8) {
        Self(v)
    }
}

//#[test]
//...that 'API_REVISION' is without terminating '\0'.
