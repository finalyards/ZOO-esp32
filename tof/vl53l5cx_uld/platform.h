/*
* Plain interfaces cannot be copyrighted, so placed this code verbatim.
*
* To create a Rust version of this, 'bindgen' was run manually (once), then the output edited:
*   <<
*   	$ bindgen platform.h --allowlist-function 'VL53L5CX_.+' \
*   	    --opaque-type=VL53L5CX_Platform \
*   	    --use-core \
*   	  | sed s/TimeMs/time_ms/ \
*   	  | sed s/RegisterAdress/register_address/ > src/platform.rs
*   <<
*/
#pragma once
#include <stdint.h>

#include "tmp/config.h"

/**
 * @brief 'VL53L5CX_Platform' is an opaque structure, provided by the customer's app.
 *      Gets placed "as-is" (as value) into 'VL53L5CX_Configuration', in the vendor code,
 *      and a pointer to that "slot" is passed to customer-provided functions.
 *
 *      What I could do:
 *          Change the definition of 'VL53L5CX_Configuration' so that the 'platform' field
 *          is an extension (empty array) as the *last* entry (now it's the first one).
 *          + would allow us to define its contents within Rust
 *          - would need patching the C sources (doable)
 *
 *      Or:
 *          Define the *contents* of the 'VL.._Platform' here (in '.h'), so that both Rust
 *          and vendor driver can read it (it's no longer opaque to the vendor driver, but
 *          that doesn't matter).
 *          + no patching of vendor C sources needed (for this)
 *          - need to juggle between '.h' and Rust, if fields are changed
 *
 *      Or:
 *          Skip trying to sync .h/Rust, and just consume enough *space* here, that the Rust
 *          'Platform' contents fit in it.
 *          + neat (though hack); is maintainable because we can automatically check ('sizeof') that the space is enough
 */
typedef struct {
    _Alignas(8) uint8_t _[20];      // 20+ bytes; 8 aligned; in practice 'clang' (18.1.3) makes it 24 bytes wide
} VL53L5CX_Platform;

/**
 * @brief Read a single byte.
 * @param (VL53L5CX_Platform*) p_platform : platform structure
 * @param (uint16_t) addr : index of value to read
 * @param (uint8_t) *p_out : where result is placed
 * @return (uint8_t) status : 0 if OK
 */
uint8_t VL53L5CX_RdByte(
		VL53L5CX_Platform *p_platform,
		uint16_t addr,
		uint8_t *p_out);

/**
 * @brief Write one single byte.
 * @param (VL53L5CX_Platform*) p_platform : platform structure
 * @param (uint16_t) addr : index of value to read
 * @param (uint8_t) value : value to write
 * @return (uint8_t) status : 0 if OK
 */
uint8_t VL53L5CX_WrByte(
		VL53L5CX_Platform *p_platform,
		uint16_t addr,
		uint8_t value);

/**
 * @brief Read multiples bytes
 * @param (VL53L5CX_Platform*) p_platform : platform structure
 * @param (uint16_t) addr : index of values to read
 * @param (uint8_t) *p_out : stores the read data
 * @param (uint32_t) size : size of '*p_out'
 * @return (uint8_t) status : 0 if OK
 */
uint8_t VL53L5CX_RdMulti(
		VL53L5CX_Platform *p_platform,
		uint16_t addr,
		uint8_t *p_out,
		uint32_t size);

/**
 * @brief Write multiples bytes
 * @param (VL53L5CX_Platform*) p_platform : platform structure
 * @param (uint16_t) addr : index of values to write.
 * @param (uint8_t) *p_values : bytes to write
 * @param (uint32_t) size : size of '*p_values'
 * @return (uint8_t) status : 0 if OK
 */
uint8_t VL53L5CX_WrMulti(
		VL53L5CX_Platform *p_platform,
		uint16_t addr,
		uint8_t *p_values,
		uint32_t size);

/**
 * @brief Swap the order of bytes, within the buffer, such that for each 4-byte group: ABCD -> DCBA
 * @param (uint8_t*) buffer : Buffer to swap, "generally uint32_t" (:O)
 * @param (uint16_t) size : Buffer size to swap (in bytes, thus multiple of 4)
 */
void VL53L5CX_SwapBuffer(
		uint8_t *buffer,
		uint16_t size);

/**
 * @brief Wait some time (100ms is longest this will be used on).
 * @param (VL53L5CX_Platform*) p_platform : platform structure
 * @param (uint32_t) ms : time to wait
 * @return (uint8_t) status : 0 if wait is finished
 */
uint8_t VL53L5CX_WaitMs(
		VL53L5CX_Platform *p_platform,
		uint32_t ms);
