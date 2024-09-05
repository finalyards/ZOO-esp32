/*
* Rust side gets results as enums, matrices etc. This module takes care of converting the ULD C API
* vector to those.
*
* Note: Many of the individual data are steered by features. These go all the way to the C level;
*       having an unneeded feature off means more slender driver code, less data to transfer.
*/

use crate::uld_raw::{
    VL53L5CX_ResultsData
};

pub struct ResultsData<const DIM:u8> {
    #[cfg(feature = "distance_mm")]
    pub distance_mm: [i16; 16 /*DIM*DIM*/],
    #[cfg(feature = "target_status")]
    pub target_status: [u8; 16 /*DIM*DIM*/],
}

impl<const DIM: u8> From<VL53L5CX_ResultsData> for ResultsData<DIM> {
    fn from(raw_results: VL53L5CX_ResultsData) -> Self {
        //unimplemented!()

        // Note: No need for first setting the matrices to '0'; is there a more idiomatic way
        //      to initialize 'struct' members, from references to slices?

        Self {
            #[cfg(feature = "distance_mm")]
            distance_mm: {
                let mut tmp: [i16;16] = [0;16];
                tmp.clone_from_slice(&raw_results.distance_mm[..16]);
                tmp
            },
            #[cfg(feature = "target_status")]
            target_status: {
                let mut tmp: [u8;16] = [0;16];
                tmp.clone_from_slice(&raw_results.target_status[..16]);
                tmp
            },
        }
    }
}

/***R
impl<const DIM: u8> From<VL53L5CX_ResultsData> for ResultsData<DIM> {
    fn from(raw_results: VL53L5CX_ResultsData) -> Self {
        unimplemented!()
    }
}
***/