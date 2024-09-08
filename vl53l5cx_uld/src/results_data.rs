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
#[allow(unused_imports)]
use defmt::{warn,debug,trace,assert};

use crate::uld_raw::{
    VL53L5CX_ResultsData
};

#[cfg(feature = "targets_per_zone_1")]
const TARGETS: usize = 1;
#[cfg(feature = "targets_per_zone_2")]
const TARGETS: usize = 2;
#[cfg(feature = "targets_per_zone_3")]
const TARGETS: usize = 3;
#[cfg(feature = "targets_per_zone_4")]
const TARGETS: usize = 4;

pub struct ResultsData<const DIM: usize> {      // DIM: 4,8
    // Metadata: DIMxDIM matrix, regardless of 'TARGETS'
    //
    #[cfg(feature = "ambient_per_spad")]
    pub ambient_per_spad: [[u32; DIM]; DIM],
    #[cfg(feature = "nb_spads_enabled")]
    pub spads_enabled: [[u32; DIM]; DIM],
    #[cfg(feature = "nb_targets_detected")]
    pub targets_detected: [[u8; DIM]; DIM],     // 1..{X in 'targets_per_zone_X' feature}

    // Actual results: DIMxDIMxTARGETS
    #[cfg(feature = "target_status")]
    pub target_status: [[[TargetStatus; DIM]; DIM]; TARGETS],

    #[cfg(feature = "distance_mm")]
    pub distance_mm: [[[u16; DIM]; DIM]; TARGETS],
    #[cfg(feature = "range_sigma_mm")]
    pub range_sigma_mm: [[[u16; DIM]; DIM]; TARGETS],

    #[cfg(feature = "reflectance_percent")]
    pub reflectance: [[[u8; DIM]; DIM]; TARGETS],
    #[cfg(feature = "signal_per_spad")]
    pub signal_per_spad: [[[u32; DIM]; DIM]; TARGETS],

    // Scalar metadata
    pub silicon_temp_degc: i8                   // "internal sensor silicon temperature"
}

impl<const DIM: usize> ResultsData<DIM> {
    /*
    * Provide an empty buffer-like struct; owned usually by the application and fed via 'feed()'.
    */
    pub(crate) fn empty() -> Self {

        Self {
            #[cfg(feature = "ambient_per_spad")]
            ambient_per_spad: [[0;DIM];DIM],
            #[cfg(feature = "nb_spads_enabled")]
            spads_enabled: [[0;DIM];DIM],
            #[cfg(feature = "nb_targets_detected")]
            targets_detected: [[0;DIM];DIM],

            #[cfg(feature = "target_status")]
            target_status: [[[TargetStatus::Other(0);DIM];DIM];TARGETS],

            #[cfg(feature = "distance_mm")]
            distance_mm: [[[0;DIM];DIM];TARGETS],
            #[cfg(feature = "range_sigma_mm")]
            range_sigma_mm: [[[0;DIM];DIM];TARGETS],

            #[cfg(feature = "signal_per_spad")]
            signal_per_spad: [[[0;DIM];DIM];TARGETS],
            #[cfg(feature = "reflectance_percent")]
            reflectance: [[[0;DIM];DIM];TARGETS],

            silicon_temp_degc: 0
        }
    }

    pub(crate) fn feed(&mut self, raw_results: &VL53L5CX_ResultsData) {

        // helpers
        //
        // The ULD C API matrix layout is (for a fictional 2x2x2 matrix = only the corner zones,
        // looking _out_ through the sensor of a SATEL mini-board so that the PCB text is
        // horizontal and right-way-up)  <-- heh, complex :)
        //
        //  ^-- i.e. what the sensor "sees" (not how we look at the sensor)
        //
        // Real world:
        //      [A B]   // A₁..D₁ = first targets; A₂..D₂ = 2nd targets; i.e. same target zone
        //      [C D]
        //
        // ULD C API vector:
        //      [A₁ A₂ B₁ B₂ C₁ C₂ D₁ D₂]   // every "zone" is first covered; then next zone
        //
        fn into_matrix_map_o<IN: Copy,OUT, const DIM: usize>(raw: &[IN], offset: usize, out: &mut [[OUT;DIM];DIM], f: impl Fn(IN) -> OUT) {
            let raw = &raw[..DIM*DIM*TARGETS];      // take only the beginning of the C buffer

            for r in 0..DIM {
                for c in 0..DIM {
                    out[r][c] = f(raw[(r*DIM+c)*TARGETS + offset]);
                }
            }
        }
        #[inline]
        fn into_matrix_o<X: Copy, const DIM: usize>(raw: &[X], offset: usize, out: &mut [[X;DIM];DIM]) {     // no mapping
            into_matrix_map_o(raw,offset,out,identity)
        }
        // Zone metadata: 'TARGETS' (and 'offset', by extension) are not involved.
        fn into_matrix<X: Copy, const DIM: usize>(raw: &[X], out: &mut [[X;DIM];DIM]) {
            let raw = &raw[..DIM*DIM];      // take only the beginning of the C buffer

            for r in 0..DIM {
                for c in 0..DIM {
                    out[r][c] = raw[r*DIM+c];
                }
            }
        }

        {   // show incoming
            trace!("RAW ambient_per_spad:   {}", &raw_results.ambient_per_spad);
            trace!("RAW nb_target_detected: {}", &raw_results.nb_target_detected);
            trace!("RAW nb_spads_enabled:   {}", &raw_results.nb_spads_enabled);
            trace!("RAW signal_per_spad:    {}", &raw_results.signal_per_spad);
            trace!("RAW range_sigma_mm:     {}", &raw_results.range_sigma_mm);
            trace!("RAW distance_mm:        {}", &raw_results.distance_mm);
            trace!("RAW reflectance:        {}", &raw_results.reflectance);
            trace!("RAW target_status:      {}", &raw_results.target_status);
        }

        // Metadata: DIMxDIM (just once)
        //
        #[cfg(feature = "ambient_per_spad")]
        into_matrix(&raw_results.ambient_per_spad, &mut self.ambient_per_spad);
        #[cfg(feature = "nb_spads_enabled")]
        into_matrix(&raw_results.nb_spads_enabled, &mut self.spads_enabled);
        #[cfg(feature = "nb_targets_detected")]
        into_matrix(&raw_results.nb_target_detected, &mut self.targets_detected);

        // Results: DIMxDIMxTARGETS
        //
        for i in 0..TARGETS {
            #[cfg(feature = "target_status")]
            into_matrix_map_o(&raw_results.target_status, i, &mut self.target_status[i], TargetStatus::from_uld);

            // We tolerate '.distance_mm' == 0 for non-existing data (where '.target_status' is 0); no need to check.
            //
            #[cfg(feature = "distance_mm")]
            into_matrix_map_o(&raw_results.distance_mm, i, &mut self.distance_mm[i],
            |v: i16| -> u16 {
                assert!(v >= 0, "Unexpected 'distance_mm' value: {} < 0", v); v as u16
            });
            #[cfg(feature = "range_sigma_mm")]
            into_matrix_o(&raw_results.range_sigma_mm, i, &mut self.range_sigma_mm[i]);

            #[cfg(feature = "reflectance_percent")]
            into_matrix_o(&raw_results.reflectance, i, &mut self.reflectance[i]);
            #[cfg(feature = "signal_per_spad")]
            into_matrix_o(&raw_results.signal_per_spad, i, &mut self.signal_per_spad[i]);
        }

        // Scalar metadata
        //
        self.silicon_temp_degc = raw_results.silicon_temp_degc;     // i8: presumably can operate sub-0°C temperature ❄️❄️
    }
}

//---
// Target status
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
pub enum TargetStatus {
    Valid(u8),          // 100% valid: 5
    HalfValid(u8),      // 50% valid: 6,9
    Invalid,            // 255
    //
    Other(u8),          // other values: 0..13 excluding above; RARE
                        //               14..254 (inclusive); should not occur
}

#[cfg(feature = "target_status")]
impl TargetStatus {
    fn from_uld(v: u8) -> Self {
        match v {
            5 => { Self::Valid(v) },
            6 | 9 => { Self::HalfValid(v) },
            255 => { Self::Invalid },
            v => {
                if v > 13 {
                    warn!("Unexpected 'target_status' value: {=u8}", v);
                }
                Self::Other(v)
            }
        }
    }
}
