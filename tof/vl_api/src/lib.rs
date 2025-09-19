#![no_std]
#![allow(non_snake_case)]
extern crate alloc;

#[cfg(feature = "single")]
mod ranging;
#[cfg(feature = "flock")]
mod ranging_flock;

mod uld_platform;
mod vl53;

#[cfg(feature = "single")]
pub use ranging::{SoloResults, Ranging};

#[cfg(feature = "flock")]
pub use {
    ranging_flock::{FlockResults, RangingFlock},
    vl53::VLsExt      // tbd. how to provide such methods properly?  Compare with 'fugit'.
};

pub use vl53::{
    VL53,
};

// Elements we pass through from the ULD level. Careful here: ideally all API is under our direct control!
pub use vl_uld::{
    API_REVISION as ULD_VERSION,
    DEFAULT_I2C_ADDR,
    I2cAddr,
    Mode,
    RangingConfig,
    Result as UldResult,
    ResultsData,    // leaked (intentionally) via '{Flock|Solo}Results'
    TargetOrder,
    units,
};
