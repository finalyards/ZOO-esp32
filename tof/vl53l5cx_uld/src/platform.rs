/*
* The platform object, handling ULD <-> hardware interactions.
*/
#![allow(non_snake_case)]

#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{trace, warn};

use core::{
    ffi::c_void,
    slice
};

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
*           (the prototypes were originally created using 'bindgen' manually, but remaining in sync
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
    with(pt, |p|
        match p.rd_bytes(addr, unsafe { slice::from_raw_parts_mut(p_value, 1_usize) }) {
            Ok(()) => ST_OK,
            Err(_) => ST_ERR
    })
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
    with(pt, |p| match p.wr_bytes(addr, &[v]) {
        Ok(()) => ST_OK,
        Err(_) => ST_ERR
    })
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
    with(pt, |p| match p.rd_bytes(addr, unsafe { slice::from_raw_parts_mut(p_values, size as usize) } ) {
        Ok(()) => ST_OK,
        Err(_) => ST_ERR
    })
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
    with(pt, |p| match p.wr_bytes(addr, unsafe { slice::from_raw_parts(p_values, size as usize) } ) {
        Ok(()) => ST_OK,
        Err(_) => ST_ERR
    })
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
    assert!(buf as usize %4 <= 0, "Buffer to swap byte order not 'u32' aligned");   // '<= 0' to avoid an IDE warning

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

    with(pt, |p| {
        p.delay_ms(time_ms);
        ST_OK
    })
}

/*
* NOTE! *FINALLY* thinking like Rust!! Using 'with' and a closure, we don't need to worry about
*   lifespans of the converted pointer!
*/
pub(crate)  // open for 'set_i2c_address()' so that the I2C address can be changed, on the fly!!!
fn with<T, F: Fn(&mut dyn Platform) -> T>(pt: *mut VL53L5CX_Platform, f: F) -> T {

    let x: &mut dyn Platform = {    // re-interpret what's in '*pt' as '&mut dyn Platform'
        let pt: *mut &mut dyn Platform = pt as *mut c_void as _;
        unsafe{ *pt }
    };

    /*** Something we didn't try:
    let pt = unsafe {
        core::ptr::NonNull::new_unchecked(pt).cast::<&mut dyn Platform>().as_ptr()
    };
    unsafe { *pt }
    ***/

    f(x)
}

#[cfg(not(all()))]  // Q: do we use this?
impl dyn Platform {
    /**
    * Check, whether there's a VL53L5CX device in the given I2C address. Calling does not require
    * the device to have been initialized (so is quick to check in starting).
    *
    * Modeled akin to the vendor ULD 'vl53l5cx_is_alive()'.
    *
    * Returns:
    *   Ok(true)    All fine, looks like a VL53L5CX
    *   Ok(false)   Maybe.. I2C reply received, but 'dev_id', 'rev_id' values are unrecognized
    *   Err(_)      Did not reach a device in the I2C address
    */
    pub fn ping(&mut self) -> Result<bool, ()> {
        let mut buf: [u8; 2] = [u8::MAX; 2];

        self.wr_bytes(0x7fff, &[0x00])?;
        self.rd_bytes(0, &mut buf)?;   // [dev_id, rev_id]
        self.wr_bytes(0x7fff, &[0x02])?;

        match (buf[0],buf[1]) {
            (0xf0, 0x02) => Ok(true),   // what vendor ULD C driver expects to find
            (a,b) => {
                #[cfg(feature = "defmt")]
                warn!("Unexpected pair: {} != {}", (a,b), (0xf0,0x02));
                Ok(false)
            }
        }
    }
}
