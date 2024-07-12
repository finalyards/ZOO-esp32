//!
//! lib.rs
//!
//! WIP: API for using the VL53L5CX
//!
#![no_std]

use defmt::debug;
use defmt_rtt as _;

//use core::ffi::{/*c_int,*/ c_size_t};     // this would be the nice way to use C types
//#[allow(non_camel_case_types)]
//type c_size_t = usize;

pub struct VL53L5CX {
    
}

impl VL53L5CX {
    pub fn new() -> Self {
        Self{}
    }

    pub fn say(&self) {
        debug!("Hello!")
    }
}

// VL53L5CX C API
//
// References:
//  - ...
//
#[link(name = "snappy")]
extern {
    fn snappy_max_compressed_length(source_length: usize) -> usize;
}

pub fn dummy() {
    unsafe {
        snappy_max_compressed_length(0 as usize);
        ()
    }
}

