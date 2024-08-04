/*
* By using such wrapper, we can counter-act some corner cases of 'bindgen' (it skips '#define' stuff),
* but also affect the 'const'ness-tuning _on the C side_, without needing to patch the vendor source.
*
*   Wrapped identifiers:    no 'VL53L5CX_' prefix
*   Vendor identifiers:     'VL53L5CX_...'
*/
#pragma once
#include "vl53l5cx_api.h"
#include "vl53l5cx_buffers.h"

// We don't do standard headers, so... (from '/usr/include/clang/18/include/__stddef_size_t.h'):
typedef __SIZE_TYPE__ size_t;

/** disabled
// stdbool.h
typedef _Bool bool;
#define true 1
#define false 0
**/

// 'bindgen' (0.69.4) skips these (frequently used in the vendor headers):
//  <<
//      #define VL53L5CX_POWER_MODE_SLEEP		((uint8_t) 0U)
//  <<
//
// By defining them as 'const' we get them on bindgen's radar. Note: only the entries actually used in the Rust API
// need to be provided this way.
//
// Note 2: While we're at it, we can group them into enums already here (in C side). 🌟🌟🌟

const char* const API_REVISION = VL53L5CX_API_REVISION;     // "VL53L5CX_2.0.0"

const uint16_t DEFAULT_I2C_ADDRESS = VL53L5CX_DEFAULT_I2C_ADDRESS;   // 0x52 (u16)
    // Note: Even when some C types don't make sense (like here - this could be an 'uint8_t' - the author has restrained
    //      from changing them. Small moves, Ellie!

/// @brief Status of operations.
///
///     Note that official documentation only mentions these cases:
///
///         |||
///         |---|---|
///         |0|No error|
///         |127|invalid value (from the application)|
///         |255|major error (usually timeout in I2C)|
///         |other|"combination of multiple errors"|
///
///     This means listing anything else in the API would not really make sense.
///
///     Note: Also the app side code ('RdMulti', 'MsWait' etc.) affects the codes.
///
const uint8_t ST_OK = VL53L5CX_STATUS_OK;                       // 0
const uint8_t ST_ERROR = VL53L5CX_STATUS_ERROR;	                // |255
    // not passed
    //const uint8_t ST_TIMEOUT_ERROR = VL53L5CX_STATUS_TIMEOUT_ERROR;     // |1
    //const uint8_t CORRUPTED_FRAME = VL53L5CX_STATUS_CORRUPTED_FRAME;    // |2
    //const uint8_t CRC_CSUM_FAILED = VL53L5CX_STATUS_CRC_CSUM_FAILED;	// |3
    //const uint8_t XTALK_FAILED = VL53L5CX_STATUS_XTALK_FAILED;          // |4
    //const uint8_t MCU_ERROR = VL53L5CX_MCU_ERROR;                       // |66 (0x42)
    //const uint8_t INVALID_PARAM = VL53L5CX_STATUS_INVALID_PARAM;    // |127 (0x7f)

const size_t MAX_RESULTS_SIZE = VL53L5CX_MAX_RESULTS_SIZE;

//---
// Function prototypes
//
uint8_t init(
    VL53L5CX_Configuration *p_dev
);
uint8_t set_i2c_address(
    VL53L5CX_Configuration *p_dev,
    uint16_t i2c_address        // why 16-bit?
);
uint8_t get_power_mode(
    VL53L5CX_Configuration *p_dev,
	uint8_t *p_out
);
uint8_t set_power_mode(
    VL53L5CX_Configuration *p_dev,
    uint8_t mode
);
uint8_t start_ranging(
    VL53L5CX_Configuration *p_dev
);
uint8_t stop_ranging(
    VL53L5CX_Configuration *p_dev
);
uint8_t check_data_ready(
    VL53L5CX_Configuration *p_dev,
	uint8_t *p_isReady
);
uint8_t get_ranging_data(
    VL53L5CX_Configuration *p_dev,
    VL53L5CX_ResultsData *p_results
);

#if 0
uint8_t get_resolution(
    VL53L5CX_Configuration *p_dev,
    uint8_t *p_resolution
);
uint8_t vl53l5cx_set_resolution(
    VL53L5CX_Configuration *p_dev,
    uint8_t resolution
);

/**
 * @brief This function gets the current ranging frequency in Hz. Ranging
 * frequency corresponds to the time between each measurement.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) *p_frequency_hz: Contains the ranging frequency in Hz.
 * @return (uint8_t) status : 0 if ranging frequency is OK.
 */

uint8_t vl53l5cx_get_ranging_frequency_hz(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				*p_frequency_hz);

/**
 * @brief This function sets a new ranging frequency in Hz. Ranging frequency
 * corresponds to the measurements frequency. This setting depends of
 * the resolution, so please select your resolution before using this function.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) frequency_hz : Contains the ranging frequency in Hz.
 * - For 4x4, min and max allowed values are : [1;60]
 * - For 8x8, min and max allowed values are : [1;15]
 * @return (uint8_t) status : 0 if ranging frequency is OK, or 127 if the value
 * is not correct.
 */

uint8_t vl53l5cx_set_ranging_frequency_hz(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				frequency_hz);

/**
 * @brief This function gets the current integration time in ms.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint32_t) *p_time_ms: Contains integration time in ms.
 * @return (uint8_t) status : 0 if integration time is OK.
 */

uint8_t vl53l5cx_get_integration_time_ms(
		VL53L5CX_Configuration		*p_dev,
		uint32_t			*p_time_ms);

/**
 * @brief This function sets a new integration time in ms. Integration time must
 * be computed to be lower than the ranging period, for a selected resolution.
 * Please note that this function has no impact on ranging mode continous.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint32_t) time_ms : Contains the integration time in ms. For all
 * resolutions and frequency, the minimum value is 2ms, and the maximum is
 * 1000ms.
 * @return (uint8_t) status : 0 if set integration time is OK.
 */

uint8_t vl53l5cx_set_integration_time_ms(
		VL53L5CX_Configuration		*p_dev,
		uint32_t			integration_time_ms);

/**
 * @brief This function gets the current sharpener in percent. Sharpener can be
 * changed to blur more or less zones depending of the application.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint32_t) *p_sharpener_percent: Contains the sharpener in percent.
 * @return (uint8_t) status : 0 if get sharpener is OK.
 */

uint8_t vl53l5cx_get_sharpener_percent(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				*p_sharpener_percent);

/**
 * @brief This function sets a new sharpener value in percent. Sharpener can be
 * changed to blur more or less zones depending of the application. Min value is
 * 0 (disabled), and max is 99.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint32_t) sharpener_percent : Value between 0 (disabled) and 99%.
 * @return (uint8_t) status : 0 if set sharpener is OK.
 */

uint8_t vl53l5cx_set_sharpener_percent(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				sharpener_percent);

/**
 * @brief This function gets the current target order (closest or strongest).
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) *p_target_order: Contains the target order.
 * @return (uint8_t) status : 0 if get target order is OK.
 */

uint8_t vl53l5cx_get_target_order(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				*p_target_order);

/**
 * @brief This function sets a new target order. Please use macros
 * VL53L5CX_TARGET_ORDER_STRONGEST and VL53L5CX_TARGET_ORDER_CLOSEST to define
 * the new output order. By default, the sensor is configured with the strongest
 * output.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) target_order : Required target order.
 * @return (uint8_t) status : 0 if set target order is OK, or 127 if target
 * order is unknown.
 */

uint8_t vl53l5cx_set_target_order(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				target_order);

/**
 * @brief This function is used to get the ranging mode. Two modes are
 * available using ULD : Continuous and autonomous. The default
 * mode is Autonomous.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) *p_ranging_mode : current ranging mode
 * @return (uint8_t) status : 0 if get ranging mode is OK.
 */

uint8_t vl53l5cx_get_ranging_mode(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				*p_ranging_mode);

/**
 * @brief This function is used to set the ranging mode. Two modes are
 * available using ULD : Continuous and autonomous. The default
 * mode is Autonomous.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) ranging_mode : Use macros VL53L5CX_RANGING_MODE_CONTINUOUS,
 * VL53L5CX_RANGING_MODE_CONTINUOUS.
 * @return (uint8_t) status : 0 if set ranging mode is OK.
 */

uint8_t vl53l5cx_set_ranging_mode(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				ranging_mode);

/**
 * @brief This function is used to disable the VCSEL charge pump
 * This optimizes the power consumption of the device
 * To be used only if AVDD = 3.3V
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 */
uint8_t vl53l5cx_enable_internal_cp(
		VL53L5CX_Configuration          *p_dev);


/**
 * @brief This function is used to disable the VCSEL charge pump
 * This optimizes the power consumption of the device
 * To be used only if AVDD = 3.3V
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 */
uint8_t vl53l5cx_disable_internal_cp(
 	      VL53L5CX_Configuration          *p_dev);

/**
 * @brief This function is used to get the number of frames between 2 temperature
 * compensation.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint32_t) *p_repeat_count : Number of frames before next temperature
 * compensation. Set to 0 to disable the feature (default configuration).
 */
uint8_t vl53l5cx_get_VHV_repeat_count(
		VL53L5CX_Configuration *p_dev,
		uint32_t *p_repeat_count);

/**
 * @brief This function is used to set a periodic temperature compensation. By
 * setting a repeat count different to 0 the firmware automatically runs a
 * temperature calibration every N frames.
 * default the repeat count is set to 0
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint32_t) repeat_count : Number of frames between temperature
 * compensation. Set to 0 to disable the feature (default configuration).
 */
uint8_t vl53l5cx_set_VHV_repeat_count(
		VL53L5CX_Configuration *p_dev,
		uint32_t repeat_count);

/**
 * @brief This function can be used to read 'extra data' from DCI. Using a known
 * index, the function fills the casted structure passed in argument.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) *data : This field can be a casted structure, or a simple
 * array. Please note that the FW only accept data of 32 bits. So field data can
 * only have a size of 32, 64, 96, 128, bits ....
 * @param (uint32_t) index : Index of required value.
 * @param (uint16_t)*data_size : This field must be the structure or array size
 * (using sizeof() function).
 * @return (uint8_t) status : 0 if OK
 */

uint8_t vl53l5cx_dci_read_data(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				*data,
		uint32_t			index,
		uint16_t			data_size);

/**
 * @brief This function can be used to write 'extra data' to DCI. The data can
 * be simple data, or casted structure.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) *data : This field can be a casted structure, or a simple
 * array. Please note that the FW only accept data of 32 bits. So field data can
 * only have a size of 32, 64, 96, 128, bits ..
 * @param (uint32_t) index : Index of required value.
 * @param (uint16_t)*data_size : This field must be the structure or array size
 * (using sizeof() function).
 * @return (uint8_t) status : 0 if OK
 */

uint8_t vl53l5cx_dci_write_data(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				*data,
		uint32_t			index,
		uint16_t			data_size);

/**
 * @brief This function can be used to replace 'extra data' in DCI. The data can
 * be simple data, or casted structure.
 * @param (VL53L5CX_Configuration) *p_dev : VL53L5CX configuration structure.
 * @param (uint8_t) *data : This field can be a casted structure, or a simple
 * array. Please note that the FW only accept data of 32 bits. So field data can
 * only have a size of 32, 64, 96, 128, bits ..
 * @param (uint32_t) index : Index of required value.
 * @param (uint16_t)*data_size : This field must be the structure or array size
 * (using sizeof() function).
 * @param (uint8_t) *new_data : Contains the new fields.
 * @param (uint16_t) new_data_size : New data size.
 * @param (uint16_t) new_data_pos : New data position into the buffer.
 * @return (uint8_t) status : 0 if OK
 */

uint8_t vl53l5cx_dci_replace_data(
		VL53L5CX_Configuration		*p_dev,
		uint8_t				*data,
		uint32_t			index,
		uint16_t			data_size,
		uint8_t				*new_data,
		uint16_t			new_data_size,
		uint16_t			new_data_pos);
#endif