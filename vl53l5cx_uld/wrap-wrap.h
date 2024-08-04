/*
* Enum definitions, for generating 'src/uld_raw.rs'.
*
* Without this, 'bindgen' does not pick these to its menu, due to the way the values
* are '#define'd in the vendor headers. We only use them on the Rust side.
*/
#pragma once
#include "wrap.h"

enum Resolution {
    _4X4 = VL53L5CX_RESOLUTION_4X4,     // 16 (u8)
    _8X8 = VL53L5CX_RESOLUTION_8X8      // 64 (u8)
};
enum TargetOrder {
    CLOSEST = VL53L5CX_TARGET_ORDER_CLOSEST,        // 1 (u8)
    STRONGEST = VL53L5CX_TARGET_ORDER_STRONGEST		// 2 (u8)
};
enum RangingMode {
    CONTINUOUS = VL53L5CX_RANGING_MODE_CONTINUOUS,  // 1 (u8)
    AUTONOMOUS = VL53L5CX_RANGING_MODE_AUTONOMOUS	// 3 (u8)
};
enum PowerMode {
    SLEEP = VL53L5CX_POWER_MODE_SLEEP,  // 0 (u8)
    WAKEUP = VL53L5CX_POWER_MODE_WAKEUP	// 1 (u8)
};
    // Using 'CamelCase' since Rust prefers that for enums.

