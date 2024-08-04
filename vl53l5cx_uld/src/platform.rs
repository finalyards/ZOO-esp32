/*
* The platform object, handling ULD <-> hardware interactions.
*/
#![allow(non_snake_case)]

use crate::Result;  // 'core::result::Result<_,u8>'
use core::slice;

use crate::uld_raw::{ST_OK, ST_ERROR as ST_ERR};

// The state. Provided by the application (opaque to us, and the vendor ULD).
//
trait Platform {

    // provided by the app
    fn delay_blocking_ms(ms: u32);
    fn rd_bytes(&mut self, addr: u16, buf: &mut [u8]) -> Result<()>;
    fn wr_bytes(&mut self, addr: u16, vs: &[u8]) -> Result<()>;

    // Note: 'reset_sensor' is omitted. The author doesn't see a point in having it below app level.
}

/*
* Raw part of interfacing.
*
* These functions are called by the ULD (C) code, passing control back to Rust.
*
* Obviously: DO NOT CHANGE THE PROTOTYPES. They must match with what's in the 'platform.h' of ULD
*           (the prorotypes were originally created using 'bindgen' manually, but remaining in sync
*           is not enforced; should be fine..).
*/

/// @brief Read a single byte
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of value to read
/// @param (uint8_t) *p_value : Where to store the value
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
pub extern "C" fn VL53L5CX_RdByte(
    pt: &mut impl Platform,
    addr: u16,          // VL index
    p_value: *mut u8
) -> u8 {
    match pt.rd_bytes(addr,p_value as &mut [u8;1]) {
        Ok(v) => ST_OK,
        Err(_) => ST_ERR
    }
}

/// @brief write one single byte
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of value to read
/// @param (uint8_t) value : value to write
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
pub extern "C" fn VL53L5CX_WrByte(
    pt: &mut impl Platform,
    addr: u16,      // VL index
    v: u8
) -> u8 {
    match pt.wr_bytes(addr,v as &[u8;1]) {
        Ok(()) => ST_OK,
        Err(_) => ST_ERR
    }
}

/// @brief read multiples bytes
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of values to read
/// @param (uint8_t) *p_values : Buffer for bytes to read
/// @param (uint32_t) size : Size of 'p_values' buffer
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
pub extern "C" fn VL53L5CX_RdMulti(
    pt: &mut impl Platform,
    addr: u16,
    p_values: *mut u8,
    size: u32   // size_t
) -> u8 {
    match pt.rd_bytes(addr, unsafe { slice::from_raw_parts_mut(p_values, size as usize) } ) {
        Ok(()) => ST_OK,
        Err(_) => ST_ERR
    }
}

/// @brief write multiples bytes
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of values to write.
/// @param (uint8_t) *p_values : bytes to write
/// @param (uint32_t) size : Size of 'p_values'
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
pub extern "C" fn VL53L5CX_WrMulti(
    pt: &mut impl Platform,
    addr: u16,
    p_values: *mut u8,  // *u8 (const)
    size: u32   // actual values fit 16 bits; size_t
) -> u8 {
    match pt.wr_bytes(addr, unsafe { slice::from_raw_parts(p_values, size as usize) } ) {
        Ok(()) => ST_OK,
        Err(_) => ST_ERR
    }
}

// NOTE: Vendor docs don't really describe what the "4-byte grouping" means, but their 'protocol.c'
//      comments provide the details.
//
/// @brief Swap each 4-byte grouping, pointed to by 'buffer', so that ABCD becomes DCBA.
/// @param (uint8_t*) buf : Buffer to swap
/// @param (uint16_t) size : Buffer size in bytes; always multiple of 4.
#[no_mangle]
pub extern "C" fn VL53L5CX_SwapBuffer(buf: *mut u8, size: u16 /*size in bytes; not words*/) {

    // Note: Since we don't actually _know_, whether 'buffer' is 4-byte aligned (to be used as '*mut u32'),
    // The original doc mentions a blurry "generally uint32_t" (not very helpful).
    //
    if (buf as u32 %3) != 0 {
        panic!("Buffer to swap byte order not 'u32' aligned");
    }

    let words: usize = (size as usize)/4;
    let s: &mut[u32] = unsafe { slice::from_raw_parts_mut(buf as *mut u32, words) };

    for i in 0..words {
        s[i] = u32::swap_bytes(s[i])
    }
}

/// @brief Wait an amount of time.
/// @param (Platform*) p_platform : platform structure
/// @param (uint32_t) time_ms : Time to wait in ms
/// @return (uint8_t) status : 0 if wait is finished
#[no_mangle]
pub extern "C" fn VL53L5CX_WaitMs(pt: &mut impl Platform, time_ms: u32) -> u8 {

    if time_ms > 100 {
        panic!("Unexpected long wish for wait: {}ms", time_ms);     // we know from the C code there's no >100
    }

    // Application provides a normal, blocking delay.
    //
    pt.delay_blocking_ms(time_ms);
    0
}
