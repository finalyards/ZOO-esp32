/*
* By using such wrapper, we can counter-act some corner cases of 'bindgen'.
*/
#include "vl53l5cx_api.h"
#include "vl53l5cx_buffers.h"

// 'bindgen' (0.69.4) skips these (frequently used in the vendor headers):
//  <<
//      #define VL53L5CX_POWER_MODE_SLEEP		((uint8_t) 0U)
//  <<
//
// By defining them as 'const' we get them on bindgen's radar. Note: only the entries actually used in the Rust API
// need to be provided this way.
//
// Note 2: While we're at it, we can group them into enums already here (in C side). 🌟

const uint16_t DEFAULT_I2C_ADDRESS = VL53L5CX_DEFAULT_I2C_ADDRESS;   // 0x52 (uint16)
    //
    // Note: Even when some C types don't make sense (like here - this could be an 'uint8_t' - the author has restrained
    //      from changing them. Small moves, Ellie!

enum RESOLUTION {
    _4X4 = VL53L5CX_RESOLUTION_4X4,     // 16 (u8)
    _8X8 = VL53L5CX_RESOLUTION_8X8      // 64 (u8)
};
enum TARGET_ORDER {
    CLOSEST = VL53L5CX_TARGET_ORDER_CLOSEST,        // 1 (u8)
    STRONGEST = VL53L5CX_TARGET_ORDER_STRONGEST		// 2 (u8)
};
enum RANGING_MODE {
    CONTINUOUS = VL53L5CX_RANGING_MODE_CONTINUOUS,  // 1 (u8)
    AUTONOMOUS = VL53L5CX_RANGING_MODE_AUTONOMOUS	// 3 (u8)
};
enum POWER_MODE {
    SLEEP = VL53L5CX_POWER_MODE_SLEEP,  // 0 (u8)
    WAKEUP = VL53L5CX_POWER_MODE_WAKEUP	// 1 (u8)
};

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

