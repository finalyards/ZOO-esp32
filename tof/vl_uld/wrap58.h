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

// 'bindgen' (0.69.4) skips these (frequently used in the vendor headers):
//  <<
//      #define VL53L5CX_POWER_MODE_SLEEP		((uint8_t) 0U)
//  <<
//
// By defining them as 'const' we get them on bindgen's radar. Note: only the entries actually used in the Rust API
// need to be provided this way.
//
// Note 2: While we're at it, we can group them into enums already here (in C side). 🌟🌟🌟

const char* API_REVISION = VL53L5CX_API_REVISION;     // "VL53L5CX_2.0.0"

/* disabled
const uint16_t DEFAULT_I2C_ADDRESS = VL53L5CX_DEFAULT_I2C_ADDRESS;   // 0x52 (u16)
    // Note: Even when some C types don't make sense (like here - this could be an 'uint8_t' - the author has refrained
    //      from changing them. Small moves, Ellie!
*/

enum Resolution {
    _4X4 = VL53L5CX_RESOLUTION_4X4,     // 16 (u8); default
    _8X8 = VL53L5CX_RESOLUTION_8X8      // 64 (u8)
};
enum TargetOrder {
    CLOSEST = VL53L5CX_TARGET_ORDER_CLOSEST,        // 1 (u8)
    STRONGEST = VL53L5CX_TARGET_ORDER_STRONGEST		// 2 (u8); default
};
enum RangingMode {
    CONTINUOUS = VL53L5CX_RANGING_MODE_CONTINUOUS,  // 1 (u8)
    AUTONOMOUS = VL53L5CX_RANGING_MODE_AUTONOMOUS	// 3 (u8); default
};
enum PowerMode {
    SLEEP = VL53L5CX_POWER_MODE_SLEEP,  // 0 (u8)
    WAKEUP = VL53L5CX_POWER_MODE_WAKEUP	// 1 (u8)
};
    // Using 'CamelCase' since Rust prefers that for enums.

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

// This comes from Rust 'targets_per_zone_{1..4}' features -> built into #define -> here back to Rust.
//
// There IS an argument using this, over Rust 'features' (which we currently do):
//      - since this already combines possible overlapping feature values
//      - and because it makes sure the library is in sync with ULD C API.
//
//const uint8_t TARGETS_PER_ZONE = VL53L5CX_NB_TARGET_PER_ZONE;     // 1..4
