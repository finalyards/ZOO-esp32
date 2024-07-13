/*
* The platform object, handling ULD <-> hardware interactions.
*/
#![allow(non_snake_case)]

#![allow(non_camel_case_types)]
struct I2C_Addr(u8);

use core::slice;
use std::hint;
use std::time::Duration;

// The state. Passed opaquely through the ULD (C) layers.
//
pub struct Platform {

}

impl Platform {

    fn rd_byte(&self) -> u8 {
        unimplemented!()
    }

    fn wr_byte(&mut self, a: I2C_Addr, v: u8) -> () {     // tbd. can we fail; return 'Result'?
        unimplemented!()
    }

    fn rd_bytes(&mut self, a: I2C_Addr, vs: &mut [u8]) -> () {     // tbd. can we fail; return 'Result'?
        unimplemented!()
    }

    fn wr_bytes(&mut self, a: I2C_Addr, vs: &[u8]) -> () {     // tbd. can we fail; return 'Result'?
        unimplemented!()
    }

    // optional; not used by ULD API
    fn reset_sensor(&mut self) -> () {
        unimplemented!()
    }
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
extern "C" pub fn VL53L5CX_RdByte(
    p_platform: *mut Platform,
    address: u16,       // note: it's weird address is 'u16'; practical I2C addresses are 7/8 bit
    p_value: *mut u8
) -> u8 {
    let p: &Platform = p_platform;
    p.rd_byte()
}

/// @brief write one single byte
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of value to read
/// @param (uint8_t) value : value to write
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
extern "C" pub fn VL53L5CX_WrByte(
    p_platform: *mut Platform,
    address: u16,
    value: u8
) -> u8 {
    let p: &Platform = p_platform;
    p.wr_byte( I2C_Addr(address as u8), v);
    unimplemented!()    // return value
}

/// @brief read multiples bytes
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of values to read
/// @param (uint8_t) *p_values : Buffer for bytes to read
/// @param (uint32_t) size : Size of 'p_values' buffer
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
extern "C" pub fn VL53L5CX_RdMulti(
    p_platform: *mut Platform,
    address: u16,
    p_values: *mut u8,
    size: u32   // size_t
) -> u8 {
    let p: &Platform = p_platform;
    p.rd_bytes( I2C_Addr(address as u8), unsafe { slice::from_raw_parts_mut(p_values, size as usize) } );
    unimplemented!()    // return value
}

/// @brief write multiples bytes
/// @param (Platform*) p_platform : platform structure
/// @param (uint16_t) address : I2C location of values to write.
/// @param (uint8_t) *p_values : bytes to write
/// @param (uint32_t) size : Size of 'p_values'
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
extern "C" pub fn VL53L5CX_WrMulti(
    p_platform: *mut Platform,
    address: u16,
    p_values: *mut u8,
    size: u32   // size_t
) -> u8 {
    let p: &Platform = p_platform;
    p.wr_bytes( I2C_Addr(address as u8), unsafe { slice::from_raw_parts(p_values, size as usize) } );
    unimplemented!()    // return value
}

/// @brief Perform a hardware reset of the sensor. Not used in the API; can be used by the host.
///         Not needed if the user don't want to reset the sensor.
/// @param (Platform*) p_platform : platform structure
/// @return (uint8_t) status : 0 if OK
#[no_mangle]
extern "C" pub fn VL53L5CX_Reset_Sensor(p_platform: *mut Platform) -> u8 {
    //let p: &Platform = p_platform;
    //p.reset_sensor();
    unimplemented!()    // return value
}

// NOTE: It wasn't at all clear from the original (ST.com) docs, what "swapping a buffer" means.
//      The description below was deduced from a sample (working) implementation.
//
/// @brief Swap each 4-byte grouping, pointed to by 'buffer', so that ABCD becomes DCBA.
/// @param (uint8_t*) buffer : Buffer to swap
/// @param (uint16_t) size : Buffer size in bytes; always multiple of 4.
#[no_mangle]
extern "C" pub fn VL53L5CX_SwapBuffer(buffer: *mut u8, size: u16 /*size_t*/) {

    // Note: Since we don't actually _know_, whether 'buffer' is 4-byte aligned (to be used as '*mut u32'),
    // The original doc mentions a blurry "generally uint32_t" (what does THAT mean?!!)
    //
    if (buffer as u32 %3) != 0 {
        panic!("Buffer to swap byte order not 'u32' aligned");
    }

    let s: &mut[u32] = unsafe { slice::from_raw_parts(buffer as *mut u32, size/4 as u32) };

    for i in 0..size/4 {
        s[i] = u32::swap_bytes(s[i])
    }

    /***
    * Implementation by Simon D.Levy:
    *   -> https://github.com/simondlevy/VL53L5CX/blob/4ddc868082dd17126a5c2160ff0060073c34cd4d/src/st/vl53l5cx_api.cpp#L10
    *
    for(uint32_t i = 0; i < size; i = i + 4) {

        uint32_t tmp = (
                buffer[i]<<24)
            |(buffer[i+1]<<16)
            |(buffer[i+2]<<8)
            |(buffer[i+3]);

        memcpy(&(buffer[i]), &tmp, 4);
    ***/
}

/// @brief Wait an amount of time.
/// @param (Platform*) p_platform : platform structure
/// @param (uint32_t) time_ms : Time to wait in ms
/// @return (uint8_t) status : 0 if wait is finished
#[no_mangle]
extern "C" pub fn VL53L5CX_WaitMs(_p_platform: *mut Platform, time_ms: u32) -> u8 {

    if time_ms > 100 {
        panic!("Unexpected long wish for wait: {}ms", time_ms);     // could be 'warn!' (but we know from the code there's no >100)
    }

    // In the code, ST uses this with 1,10 and 100 (ms) values. It seems like it's giving time for
    // the sensor to do something. We cannot do 'async' waits from a regular function, so need to
    // resort to busy-wait.
    //
    let t_until: Duration = Instant::now() + Duration::from_millis(time_ms);

    while Instant::now() < t_until {
        hint::spin_loop()       // converts to 'crate::arch::riscv32::pause()' on RISC-V
    }
    0
}
