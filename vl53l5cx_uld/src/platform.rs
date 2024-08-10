/*
* The platform object, handling ULD <-> hardware interactions.
*/
#![allow(non_snake_case)]

use core::ffi::c_void;
use core::mem::{uninitialized, MaybeUninit};
use core::slice;
use crate::Platform;
use crate::uld_raw::{
    ST_OK,
    ST_ERROR as ST_ERR,
    VL53L5CX_Platform
};

/*
* Raw part of interfacing.
*
* These functions are called by the ULD (C) code, passing control back to Rust.
*
* Obviously: DO NOT CHANGE THE PROTOTYPES. They must match with what's in the 'platform.h' of ULD
*           (the prorotypes were originally created using 'bindgen' manually, but remaining in sync
*           is not enforced; should be fine..).
*
* Note: '#[no_mangle]' (which we need) and using generics ('P : Platform') are *incompatible*
*       with each other (for good reasons); we try to circumvent this by moving to Rust-land here,
*       and letting another layer do the generics. Note: using generics is just a "but I. Want."
*       of the author!!! 😿😿
*/

/// @brief Read a single byte
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of value to read
/// @param (uint8_t) *p_value : Where to store the value
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
pub extern "C" fn VL53L5CX_RdByte(
    pt: *mut VL53L5CX_Platform,
    addr: u16,          // VL index
    p_value: *mut u8
) -> u8 {
    let p: &mut dyn Platform = conv(pt);
    match p.rd_bytes(addr, unsafe { slice::from_raw_parts_mut(p_value, 1_usize) }) {
        Ok(()) => ST_OK,
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
    pt: *mut VL53L5CX_Platform,
    addr: u16,      // VL index
    v: u8
) -> u8 {
    let p: &mut dyn Platform = conv(pt);
    match p.wr_bytes(addr, &[v]) {
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
    pt: *mut VL53L5CX_Platform,
    addr: u16,
    p_values: *mut u8,
    size: u32   // size_t
) -> u8 {
    let p: &mut dyn Platform = conv(pt);
    match p.rd_bytes(addr, unsafe { slice::from_raw_parts_mut(p_values, size as usize) } ) {
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
    pt: *mut VL53L5CX_Platform,
    addr: u16,
    p_values: *mut u8,  // *u8 (const)
    size: u32   // actual values fit 16 bits; size_t
) -> u8 {
    let p: &mut dyn Platform = conv(pt);
    match p.wr_bytes(addr, unsafe { slice::from_raw_parts(p_values, size as usize) } ) {
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
    if (buf as usize %4) != 0 {
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
pub extern "C" fn VL53L5CX_WaitMs(pt: *mut VL53L5CX_Platform, time_ms: u32) -> u8 {
    assert!(time_ms <= 100, "Unexpected long wait: {}ms", time_ms);    // we know from the C code there's no >100

    let p: &mut dyn Platform = conv(pt);
    p.delay_ms(time_ms);
    0
}

/*
* Convert the pointer from ULD C API to Rust
*/
fn conv(pt: *mut VL53L5CX_Platform) -> &'static mut dyn Platform {

    unimplemented!()    // HAVE FUN!!!

    // The memory doesn't move. Just have 'mut *pt' treated as '&mut dyn Platform'.
    // In effect, we wish to bring a 'dyn Platform' "out of nothing", and for this the
    // 'MaybeUninitialized' may be the best method?
    //
    //let uninit = unsafe { *pt as MaybeUninit<dyn Platform> };

    //
    // "Both types must have the same size. Compilation will fail if this is not guaranteed."
    //

    //unsafe { &mut *(pt as *mut c_void as *mut dyn Platform) }

    //Runsafe { *pt as _ }
}
