/*
* The actual ULD level API. Manually crafted.
*/
//#![allow(non_camel_case_types)]

use crate::uld_raw as raw;

use raw::VL53L5CX_Configuration;
use raw::VL53L5CX_API_REVISION;


// Consts we decide to expose.
//
// Other than these don't need to be brought even to 'raw'.

/**
* @brief ULD driver version, e.g. "VL53L5CX_2.0.0"
*/
pub const API_REVISION: &String = raw::VL32L5CX_API_REVISION;




/**
* Document tbd.
*/
pub fn vl53l5cx_is_alive(cfg: &VL53L5CX_Configuration) -> bool {
    let mut buf: u8 = 0;    // written 1 (alive) or 0
    let st: u8 = uld_raw::vl53l5cx_is_alive(cfg, &mut buf);
    assert_eq!(st,0, "problems, code {}", st);
    buf != 0
}

/**
 * @brief Resets the sensor, feeds the firmware and sets it to default parameters.
 */
pub fn vl53l5cx_init(cfg: &mut VL53L5CX_Configuration) -> Status {

    // Due to how the C code is, 'st' can be pretty much anything (within 8 bits).
    // It's not meaningful to check for its actual values, other than for 0/non-zero (and debugging
    // purposes, as numbers).
    //
    let st: u8 = uld_raw::vl53l5cx_init(cfg);

    Status::from(st)
}

/**
* @brief Change the I2C address of the receiving chip.
*
* Note: Unlike in the vendor library, it is up to the caller to update the 'cfg' structure, if they
*       maintain the address there.
**/
pub fn vl53lcx_set_i2c_address(cfg: &VL53L5CX_Configuration, addr: I2C_Address) -> Status {

    let st: u8 = uld_raw::vl53l5cx_set_i2c_address(cfg, addr.0 as u16);
    Status::from(st)
}

/**
 * @brief Get the power mode
 **/
pub fn vl53lcx_get_power_mode(cfg: &VL53L5CX_Configuration) -> Result<PowerMode> {
    let buf: u8 = 0;
    let st: u8 = uld_raw::vl53l5cx_get_power_mode(cfg, &buf);

    Result::from(st, || PowerMode::from(buf))
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

/*** REMOVE eventually
const DEFAULT_I2C_ADDRESS: I2C_Address = I2C_Address(0x52);

enum RESOLUTION {  // for 'set_resolution()'
    _4x4 = unimplemented!(),     // 16U
    _8x8 = unimplemented!()     // 64U
}

enum TARGET_ORDER {  // for '...'
    CLOSEST = unimplemented!(),     // 1U
    STRONGEST = unimplemented!(),     // 2U
}

enum RANGING_MODE {  // for '...'
    CONTINUOUS = unimplemented!(),     // 1U
    AUTONOMOUS = unimplemented!(),     // 3U
}

enum POWER_MODE {  // for 'set_power_mode()'
    SLEEP = unimplemented!(),     // 0U
    WAKEUP = unimplemented!(),     // 1U (default)
}
***/
