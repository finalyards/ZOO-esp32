/*
* By using such wrapper, we can counter-act some corner cases of 'bindgen'.
*/
#include "vl53l5cx_api.h"

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
    _4X4 = VL53L5CX_RESOLUTION_4X4,     // 16 (uint8)
    _8X8 = VL53L5CX_RESOLUTION_8X8      // 64 (uint8)
};
enum TARGET_ORDER {
    CLOSEST = VL53L5CX_TARGET_ORDER_CLOSEST,        // 1 (uint8)
    STRONGEST = VL53L5CX_TARGET_ORDER_STRONGEST		// 2 (uint8)
};
enum RANGING_MODE {
    CONTINUOUS = VL53L5CX_RANGING_MODE_CONTINUOUS,  // 1 (uint8)
    AUTONOMOUS = VL53L5CX_RANGING_MODE_AUTONOMOUS	// 3 (uint8)
};
enum POWER_MODE {
    SLEEP = VL53L5CX_POWER_MODE_SLEEP,  // 0 (uint8)
    WAKEUP = VL53L5CX_POWER_MODE_WAKEUP	// 1 (uint8)
};
