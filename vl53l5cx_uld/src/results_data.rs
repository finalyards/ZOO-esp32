/*
* Rust side gets results as enums, matrices etc. This module takes care of converting the ULD C API
* vector to those.
*
* Note: Many of the individual data are steered by features. These go all the way to the C level;
*       having an unneeded feature off means more slender driver code, less data to transfer.
*
* References:
*   - vendor's UM2884 > Chapter 5 ("Ranging results"); Rev 5, Feb'24; PDF 18pp.
*       -> https://www.st.com/resource/en/user_manual/um2884-a-guide-to-using-the-vl53l5cx-multizone-timeofflight-ranging-sensor-with-a-wide-field-of-view-ultra-lite-driver-uld-stmicroelectronics.pdf
*/
use core::convert::identity;
#[cfg(feature = "defmt")]
use defmt::{warn};

use crate::uld_raw::{
    VL53L5CX_ResultsData
};

pub struct ResultsData<const DIM: usize> {      // gets implemented for DIM=4, DIM=8
    #[cfg(feature = "target_status")]
    pub target_status: [[TargetStatus; DIM]; DIM],
    #[cfg(feature = "nb_targets_detected")]
    pub targets_detected: [[u8; DIM]; DIM],     // 1..{X in 'targets_per_zone_X' feature}

    #[cfg(feature = "ambient_per_spad")]
    pub ambient_per_spad: [[u32; DIM]; DIM],
    #[cfg(feature = "nb_spads_enabled")]
    pub spads_enabled: [[u32; DIM]; DIM],
    #[cfg(feature = "signal_per_spad")]
    pub signal_per_spad: [[u32; DIM]; DIM],
    #[cfg(feature = "range_sigma_mm")]
    pub range_sigma_mm: [[u16; DIM]; DIM],
    #[cfg(feature = "distance_mm")]
    pub distance_mm: [[u16; DIM]; DIM],
    #[cfg(feature = "reflectance_percent")]
    pub reflectance: [[u8; DIM]; DIM],
    #[cfg(feature = "motion_indicator")]
    pub motion_indicator: [[!; DIM]; DIM],

    pub silicon_temp_degc: i8                   // "internal sensor silicon temperature"
}

impl<const DIM: usize> ResultsData<DIM> {
    const DIM_SQ: usize = DIM*DIM;

    /*
    * Provide an empty buffer-like struct; owned usually by the application and fed via 'feed()'.
    */
    pub(crate) fn empty() -> Self {

        Self {
            #[cfg(feature = "target_status")]
            target_status: [[TargetStatus::Misc(0);DIM];DIM],
            #[cfg(feature = "nb_targets_detected")]
            targets_detected: [[0;DIM];DIM],

            #[cfg(feature = "ambient_per_spad")]
            ambient_per_spad: [[0;DIM];DIM],
            #[cfg(feature = "nb_spads_enabled")]
            spads_enabled: [[0;DIM];DIM],
            #[cfg(feature = "signal_per_spad")]
            signal_per_spad: [[0;DIM];DIM],
            #[cfg(feature = "range_sigma_mm")]
            range_sigma_mm: [[0;DIM];DIM],
            #[cfg(feature = "distance_mm")]
            distance_mm: [[0;DIM];DIM],
            #[cfg(feature = "reflectance_percent")]
            reflectance: [[0;DIM];DIM],

            silicon_temp_degc: 0
        }
    }

    pub(crate) fn feed(&mut self, raw_results: &VL53L5CX_ResultsData) {

        // helpers
        //
        // Note: Rust (1.80.1) doesn't allow accessing the 'DIM_SQ' from here (in any fashion).
        //
        fn into_matrix_map<IN: Copy,OUT, const DIM: usize>(raw: &[IN], out: &mut [[OUT;DIM];DIM], f: impl Fn(IN) -> OUT) {
            let raw = &raw[..DIM*DIM];      // take only the beginning of the C buffer

            // The order here depends on the layout of the 1D ULD C vector.
            for r in 0..DIM {
                for c in 0..DIM {
                    out[r][c] = f(raw[r*DIM+c]);
                }
            }
        }
        fn into_matrix<X: Copy, const DIM: usize>(raw: &[X], out: &mut [[X;DIM];DIM]) {     // no mapping
            into_matrix_map(raw,out,identity)
        }

        #[cfg(feature = "target_status")]
        into_matrix_map(&raw_results.target_status, &mut self.target_status, |v:u8| { TargetStatus::from_uld(v) });
        #[cfg(feature = "nb_targets_detected")]
        into_matrix(&raw_results.nb_target_detected, &mut self.targets_detected);

        #[cfg(feature = "ambient_per_spad")]
        into_matrix(&raw_results.ambient_per_spad, &mut self.ambient_per_spad);
        #[cfg(feature = "nb_spads_enabled")]
        into_matrix(&raw_results.nb_spads_enabled, &mut self.spads_enabled);
        #[cfg(feature = "signal_per_spad")]
        into_matrix(&raw_results.signal_per_spad, &mut self.signal_per_spad);
        #[cfg(feature = "range_sigma_mm")]
        into_matrix(&raw_results.range_sigma_mm, &mut self.range_sigma_mm);
        #[cfg(feature = "distance_mm")]
        into_matrix_map(&raw_results.distance_mm, &mut self.distance_mm, |v:i16| -> u16 {
            assert!(v > 0, "Unexpected 'distance_mm' value: {} <= 0", v);
            v as u16
        });
        #[cfg(feature = "reflectance_percent")]
        into_matrix(&raw_results.reflectance, &mut self.reflectance);

        self.silicon_temp_degc = raw_results.silicon_temp_degc;     // i8: presumably can operate sub-0°C temperature ❄️❄️
    }
}


// Types of data are presented below in the order they should be considered in applications
// (most important/always on, first). Vendor docs use another order.

//---
// Target status
//
// Vendor docs:
//      "Measurements validity." "5 is considered 100% valid; [...] 6 or 9 can be considered
//      [to be valid by a probability of] 50%; All other values [...] below the 50% level."
//
// Observed values:
//      5, 6, 9, 10, 255
//
// Note: Vendor docs (section 5.5.; Table 4) give detailed explanations for values 0..13 and 255.
//      They are regarded as not relevant enough to surface on the level of 'enum's. Applications
//      can access them though, as the inner values.
//
#[cfg(feature = "target_status")]
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum TargetStatus {
    Valid(u8),          // 100% valid: 5
    HalfValid(u8),      // 50% valid: 6,9
    NoTarget,           // 255
    //
    Misc(u8)            // other values: 0..13 excluding above
}

#[cfg(feature = "target_status")]
impl TargetStatus {
    fn from_uld(v: u8) -> Self {
        match v {
            5 => { Self::Valid(v) },
            6 | 9 => { Self::HalfValid(v) },
            255 => { Self::NoTarget },
            v => {
                if v > 13 {
                    warn!("Unexpected 'target_status' value: {=u8}", v);
                }
                Self::Misc(v)
            }
        }
    }
}

//---
// Number of targets detected
//
// Vendor docs:
//      "Number of detected targets in the current zone. This value should be the first one to check to
//      know a measurement validity."
//
// Observed values:
//
#[cfg(feature = "nb_targets_detected")]
fn _no_op5() {}

//---
// Ambient per SPAD
//
// Vendor docs:
//      "ambient signal rate due to noise" (unit: Kcps/SPAD)
//
// Observed values:
//
#[cfg(feature = "ambient_per_spad")]
fn _no_op6() {}

//---
// Number of SPADs enabled
//
// Vendor docs:
//      "Number of SPADs enabled for the current measurement. A far or low reflective target
//      activates more SPADs."
//
// Observed values:
//
#[cfg(feature = "nb_spads_enabled")]
fn _no_op7() {}

//---
// Signal per SPAD  // "signal returned to the sensor in kcps/spads"
//
// Vendor docs:
//      "Quantity of photons measured during the VCSEL." (unit: Kcps/SPAD)
//
// Observed values:
#[cfg(feature = "signal_per_spad")]
fn _no_op8() {}

//---
// Range sigma  // "sigma of the current distance in mm"
//
// Vendor docs:
//      "Sigma estimator for the noise in the reported" (unit: mm)
//
// Observed values:
#[cfg(feature = "range_sigma_mm")]
fn _no_op9() {}

//---
// Distance
//
// Vendor docs:
//      "Target distance" (unit: mm)
//
// Observed values:
#[cfg(feature = "distance_mm")]
fn _no_op10() {}

//---
// Reflectance      // "estimated reflectance in percent"
//
// Vendor docs:
//      "Estimated target reflectance in percent" (unit: percent)
//
// Observed values:
#[cfg(feature = "reflectance_percent")]
fn _no_op11() {}
