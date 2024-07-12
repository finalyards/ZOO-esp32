//!
//! lib.rs
//!
//! WIP: API for using the VL53L5CX
//!
#![no_std]

use esp_println::println;

struct VL53L5CX {
    
}

impl VL53L5CX {
    pub fn new() -> Self {
        Self{}
    }

    pub fn say(&self) {
        debug!("Hello!")
    }
}
