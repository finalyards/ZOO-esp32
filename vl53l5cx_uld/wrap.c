/*
* Wrapper between vendor ULD functions and Rust.
*
*   Wrapped identifiers:    no 'vl53l5cx_' prefix
*   Vendor identifiers:     'vl53l5cx_...'
*
* Allows us to:
*   - add constness information to pointer usage
*
* Note: Function comments are based on vendor header comments. Those are under BSD-3-Clause license (but you'll likely
*       have the LICENSE.txt anyways, since this repo won't compile without injecting the vendor sources).
*
*       For vendor: I hope you approve of my reproducing these API documentation, for common good.
*/
#include "wrap.h"
//#include "vl53l5cx_api.h"
//#include "vl53l5cx_buffers.h"

// 'vl53l5cx_is_alive' skipped; we can replicate it in Rust.

//#define VL_CFG_CAST(p) ((VL53L5CX_Configuration*)(p))

/**
 * @brief Initializes the sensor, loading the firmware. "Takes a few hundred milliseconds".
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @return (uint8_t) status : 0 if initialization is OK.
 */
uint8_t init(
    VL53L5CX_Configuration *p_dev
) {
    return vl53l5cx_init(p_dev);
}

/**
 * @brief Changes (re-programs) the I2C address of the sensor.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint16_t) i2c_address : New I2C address
 * @return (uint8_t) status : 0 if new address is OK
 */
uint8_t set_i2c_address(
    VL53L5CX_Configuration *p_dev,     // only uses for I2C; could be 'const'
    uint16_t i2c_address        // why 16-bit?
) {
    return vl53l5cx_set_i2c_address(p_dev, i2c_address);
}

/**
 * @brief Get the current sensor power mode.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *p_out : written the current power mode
 * @return (uint8_t) status : 0 if power mode is OK
 */
uint8_t get_power_mode(
    VL53L5CX_Configuration *p_dev,     // only for I2C access
	uint8_t *p_out  // PowerMode
) {
	return vl53l5cx_get_power_mode(p_dev, p_out);
}

/**
 * @brief Set the sensor into Low Power mode, for example if the sensor is not used during a long time,
 * and back to normal ('WAKEUP'). Please ensure that the device is not streaming before calling the function.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) mode : Selected power mode
 * @return (uint8_t) status : 0 if power mode is OK
 */
uint8_t set_power_mode(
    VL53L5CX_Configuration * p_dev,
    uint8_t mode
) {
    return vl53l5cx_set_power_mode(p_dev, mode);
}

/**
 * @brief Starts a ranging session. When the sensor streams, host cannot change settings 'on-the-fly'.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @return (uint8_t) status : 0 if start is OK
 */
uint8_t start_ranging(
    VL53L5CX_Configuration *p_dev       // writes (at least) 'data_read_size', 'streamcount'; uses 'temp_buffer'
) {
    return vl53l5cx_start_ranging(p_dev);
}

/**
 * @brief Stops the ranging session. It must be used when the sensor streams, after calling 'vl53l5cx_start_ranging()'.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @return (uint8_t) status : 0 if stop is OK
 */
uint8_t stop_ranging(
    VL53L5CX_Configuration *p_dev     // I2C access; reads 'is_auto_stop_enabled'
) {
    return vl53l5cx_stop_ranging(p_dev);
}

/**
 * @brief Checks if new data is ready by polling I2C.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (bool_t) *p_isReady : set to 0 or 1
 * @return (uint8_t) status : 0 if I2C reading is OK
 */
uint8_t check_data_ready(
    VL53L5CX_Configuration *p_dev,    // uses 'temp_buffer'
	uint8_t *p_out
) {
	return vl53l5cx_check_data_ready(p_dev, p_out);
}

/**
 * @brief Gets the ranging data, using the selected output and resolution.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (VL53L5CX_ResultsData) *p_results : VL53L5 results structure
 * @return (uint8_t) status : 0 data are successfully gotten
 */
uint8_t get_ranging_data(
    VL53L5CX_Configuration *p_dev,      // writes 'streamcount'; uses 'temp_buffer'; reads 'data_read_size'
    VL53L5CX_ResultsData *p_results
) {
    return vl53l5cx_get_ranging_data(p_dev, p_results);
}

/**
 * @brief Gets the current resolution (4x4 or 8x8).
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *p_out : 16 for 4x4 mode, and 64 for 8x8 mode
 * @return (uint8_t) status : 0 if resolution is OK
 */
uint8_t get_resolution(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint8_t *p_out
) {
	return vl53l5cx_get_resolution(p_dev, p_out);
}

/**
 * @brief Sets a new resolution (4x4 or 8x8).
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) reso : VL53L5CX_RESOLUTION_4X4 or VL53L5CX_RESOLUTION_8X8
 * @return (uint8_t) status : 0 if set resolution is OK
 */
uint8_t set_resolution(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint8_t reso
) {
    return vl53l5cx_set_resolution(p_dev, reso);
}

/**
 * @brief Gets the current ranging frequency in Hz.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *p_out: ranging frequency in Hz
 * @return (uint8_t) status : 0 if ranging frequency is OK
 */
uint8_t get_ranging_frequency_hz(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint8_t *p_out
);

/**
 * @brief Sets a new ranging frequency in Hz. This setting depends on the resolution,
 * so SELECT YOUR RESOLUTION BEFORE using this function!
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) frequency_hz : ranging frequency in Hz
 *  - For 4x4, allowed values are (inclusive): 1..60
 *  - For 8x8, allowed values are (inclusive): 1..15
 * @return (uint8_t) status : 0 if ranging frequency is OK
 */
uint8_t set_ranging_frequency_hz(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint8_t frequency_hz
);

/**
 * @brief Gets the current integration time in ms.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint32_t) *p_out: integration time in ms
 * @return (uint8_t) status : 0 if integration time is OK
 */
uint8_t get_integration_time_ms(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint32_t *p_out
);

/**
 * @brief Sets a new integration time in ms. Integration time must
 * be computed to be lower than the ranging period, for a selected resolution.
 * Please note that this function has no impact on ranging mode CONTINUOUS.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint32_t) time_ms : integration time in ms. For all resolutions and frequency,
 * the minimum value is 2ms, and the maximum is 1000ms.
 * @return (uint8_t) status : 0 if set integration time is OK
 */
uint8_t set_integration_time_ms(
    VL53L5CX_Configuration *p_dev,      // uses 'temp_buffer'
    uint32_t integration_time_ms
);

/**
 * @brief Gets the current sharpener in percent. Sharpener can be changed to blur more or less zones depending on
 * the application.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint32_t) *p_out: sharpener in percent
 * @return (uint8_t) status : 0 if get sharpener is OK
 */
uint8_t get_sharpener_percent(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint8_t *p_out
);

/**
 * @brief Sets a new sharpener value in percent. Min value is 0 (disabled), and max is 99.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint32_t) sharpener_percent : value within range (inclusive): 0..99
 * @return (uint8_t) status : 0 if set sharpener is OK
 */
uint8_t set_sharpener_percent(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint8_t sharpener_percent
);

/**
 * @brief Gets the current target order (CLOSEST or STRONGEST).
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *p_out: output value
 * @return (uint8_t) status : 0 if get target order is OK
 */
uint8_t vl53l5cx_get_target_order(
    VL53L5CX_Configuration *p_dev,     // uses 'temp_buffer'
    uint8_t *p_out
);

/**
 * @brief Sets a new target order. By default, the sensor is configured with the STRONGEST
 * output.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) target_order : Required target order: STRONGEST or CLOSEST
 * @return (uint8_t) status : 0 if set target order is OK
 */
uint8_t set_target_order(
    VL53L5CX_Configuration *p_dev,     // uses 'temp_buffer'
    uint8_t target_order
);

/**
 * @brief Get the ranging mode. The default mode is AUTONOMOUS.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *p_out : current ranging mode
 * @return (uint8_t) status : 0 if get ranging mode is OK
 */
uint8_t get_ranging_mode(
    VL53L5CX_Configuration *p_dev,     // uses 'temp_buffer'
    uint8_t *p_out
);

/**
 * @brief Set the ranging mode. The default mode is AUTONOMOUS.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) ranging_mode : CONTINUOUS or AUTONOMOUS
 * @return (uint8_t) status : 0 if set ranging mode is OK
 */
uint8_t set_ranging_mode(
    VL53L5CX_Configuration *p_dev,     // uses 'temp_buffer'
    uint8_t ranging_mode
);

/**
 * @brief Enable the VCSEL charge pump. Optimizes the power consumption of the device.
 * To be used only if AVDD = 3.3V
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 */
#if 0
uint8_t enable_internal_cp(
    VL53L5CX_Configuration *p_dev       // uses 'temp_buffer'
);

/**
 * @brief Disable the VCSEL charge pump. Optimizes the power consumption of the device.
 * To be used only if AVDD = 3.3V
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 */
uint8_t disable_internal_cp(
    VL53L5CX_Configuration *p_dev       // uses 'temp_buffer'
);
#endif  // charge pump

#if 0   // VHV
/**
 * @brief Get the number of frames between 2 temperature compensation[s?].
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint32_t) *p_out : Number of frames before next temperature
 * compensation. 0 if the feature is disabled (default configuration).
 */
uint8_t get_VHV_repeat_count(
    VL53L5CX_Configuration *p_dev,      // uses 'temp_buffer'
    uint32_t *p_out
);

/**
 * @brief Set a periodic temperature compensation. By setting a repeat count different
 * to 0 the firmware automatically runs a temperature calibration every N frames.
 * By default, the repeat count is set to 0 (disabled).
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint32_t) count : Number of frames between temperature compensations.
 * Set to 0 to disable the feature (default configuration).
 */
uint8_t set_VHV_repeat_count(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    uint32_t count
);
#endif  // VHV

#if 0   // DCI
/**
 * @brief Can be used to read 'extra data' from DCI. Using a known index, the function fills the structure
 * passed in argument.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *data : This field can be a casted structure, or a simple
 * array. Please note that the FW only accepts data aligned to 32 bits.
 * @param (uint32_t) index : Index of required value
 * @param (uint16_t)*data_size : size of structure or array
 * @return (uint8_t) status : 0 if OK
 */
uint8_t dci_read_data(
    VL53L5CX_Configuration *p_dev,  // uses 'temp_buffer'
    void *data,
    uint32_t index,
    uint16_t data_size
);

/**
 * @brief Writes 'extra data' to DCI. The data can be simple data, or structure.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *data : structure, or a simple array. Please note that the FW only accepts data aligned to 32 bits.
 * @param (uint32_t) index : Index of required value.
 * @param (uint16_t)*data_size : structure or array size
 * @return (uint8_t) status : 0 if OK
 */
uint8_t dci_write_data(
    VL53L5CX_Configuration *p_dev,     // uses 'temp_buffer'
    void *data,
    uint32_t index,
    uint16_t data_size
);

/**
 * @brief Replace 'extra data' in DCI.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure
 * @param (uint8_t) *data : structure, or a simple array. Please note that the FW only accept data aligned to 32 bits.
 * @param (uint32_t) index : Index of required value
 * @param (uint16_t)*data_size : structure or array size
 * @param (uint8_t) *new_data : Contains the new fields
 * @param (uint16_t) new_data_size : New data size
 * @param (uint16_t) new_data_pos : New data position into the buffer
 * @return (uint8_t) status : 0 if OK
 */
uint8_t dci_replace_data(
    VL53L5CX_Configuration *p_dev,     // uses 'temp_buffer'
    void *data,
    uint32_t index,
    uint16_t data_size,
    uint8_t *new_data,
    uint16_t new_data_size,
    uint16_t new_data_pos
);
#endif     //DCI
