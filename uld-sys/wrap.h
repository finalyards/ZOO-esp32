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

/// @brief Status of operations. In practice, the value may also be a mixture of multiple values,
///     even to absurdity. There is no sense in (reliably) analyzing these deeper than knowing that
///     0 means ok and anything else an error (or multiple errors).
///
///     Note: The merging of statuses is complicated by the application code ('RdMulti', 'MsWait' etc.)
///         also getting their (arbitrary) error codes OR'ed with others, e.g. a timeout.
///
const uint8_t ST_OK = VL53L5CX_STATUS_OK;           // 0
const uint8_t ST_ERROR = VL53L5CX_STATUS_ERROR;	    // |255

const uint8_t ST_TIMEOUT_ERROR = VL53L5CX_STATUS_TIMEOUT_ERROR;     // |1
const uint8_t CORRUPTED_FRAME = VL53L5CX_STATUS_CORRUPTED_FRAME;    // |2
const uint8_t CRC_CSUM_FAILED = VL53L5CX_STATUS_CRC_CSUM_FAILED;	// |3
const uint8_t XTALK_FAILED = VL53L5CX_STATUS_XTALK_FAILED;          // |4
const uint8_t MCU_ERROR = VL53L5CX_MCU_ERROR;                       // |66 (0x42)
const uint8_t INVALID_PARAM = VL53L5CX_STATUS_INVALID_PARAM;        // |127 (0x7f)

// BH means Block Header
//
/*
const uint32_t _START_BH =       VL53L5CX_START_BH;			    //	((uint32_t)0x0000000DU)
const uint32_t _METADATA_BH =		VL53L5CX_METADATA_BH;		    //	((uint32_t)0x54B400C0U)
const uint32_t _COMMONDATA_BH =		VL53L5CX_COMMONDATA_BH;		    //	((uint32_t)0x54C00040U)
const uint32_t _AMBIENT_RATE_BH =		VL53L5CX_AMBIENT_RATE_BH;	//	((uint32_t)0x54D00104U)
const uint32_t _SPAD_COUNT_BH =		VL53L5CX_SPAD_COUNT_BH;		    //	((uint32_t)0x55D00404U)
const uint32_t _NB_TARGET_DETECTED_BH =	VL53L5CX_NB_TARGET_DETECTED_BH; //	((uint32_t)0xDB840401U)
const uint32_t _SIGNAL_RATE_BH =		VL53L5CX_SIGNAL_RATE_BH;		    //	((uint32_t)0xDBC40404U)
const uint32_t _RANGE_SIGMA_MM_BH =		VL53L5CX_RANGE_SIGMA_MM_BH;	//	((uint32_t)0xDEC40402U)
const uint32_t _DISTANCE_BH =		VL53L5CX_DISTANCE_BH;		    //	((uint32_t)0xDF440402U)
const uint32_t _REFLECTANCE_BH =		VL53L5CX_REFLECTANCE_BH;		    //	((uint32_t)0xE0440401U)
const uint32_t _TARGET_STATUS_BH =		VL53L5CX_TARGET_STATUS_BH;	//	((uint32_t)0xE0840401U)
const uint32_t _MOTION_DETECT_BH =		VL53L5CX_MOTION_DETECT_BH;	//	((uint32_t)0xD85808C0U)
*/

// IDX likely internal (library/sensor indices)
/*
const uint16_t _METADATA_IDX =		    VL53L5CX_METADATA_IDX;		    //	((uint16_t)0x54B4U)
const uint16_t _SPAD_COUNT_IDX =		    VL53L5CX_SPAD_COUNT_IDX;		    //	((uint16_t)0x55D0U)
const uint16_t _AMBIENT_RATE_IDX =		 VL53L5CX_AMBIENT_RATE_IDX;		//((uint16_t)0x54D0U)
const uint16_t _NB_TARGET_DETECTED_IDX =	 VL53L5CX_NB_TARGET_DETECTED_IDX;	//((uint16_t)0xDB84U)
const uint16_t _SIGNAL_RATE_IDX =		 VL53L5CX_SIGNAL_RATE_IDX;		//((uint16_t)0xDBC4U)
const uint16_t _RANGE_SIGMA_MM_IDX =		 VL53L5CX_RANGE_SIGMA_MM_IDX;		//((uint16_t)0xDEC4U)
const uint16_t _DISTANCE_IDX =		    VL53L5CX_DISTANCE_IDX;		    //	((uint16_t)0xDF44U)
const uint16_t _REFLECTANCE_EST_PC_IDX =	 VL53L5CX_REFLECTANCE_EST_PC_IDX;	//((uint16_t)0xE044U)
const uint16_t _TARGET_STATUS_IDX =		 VL53L5CX_TARGET_STATUS_IDX;		//((uint16_t)0xE084U)
const uint16_t _MOTION_DETEC_IDX =		 VL53L5CX_MOTION_DETEC_IDX;		//((uint16_t)0xD858U)
*/