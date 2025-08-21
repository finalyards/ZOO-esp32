/*
* Convert data received from the ULD C API to more.. robust formats:
*   - 1D vectors -> 2D matrices
*   - integers -> enums or tuple structs
*   - some squeezing of type safety, e.g. negative 'distance_mm's not accepted
*
* Note: It is by design that these conversions happen already at the ULD level.
*
* Note: Many of the individual data are steered by features. These go all the way to the C level:
*       disabling a feature means less driver code, less data to transfer.
*
* References:
*   - vendor's UM2884 > Chapter 5 ("Ranging results"); Rev 5, Feb'24; PDF 18pp.
*       -> https://www.st.com/resource/en/user_manual/um2884-a-guide-to-using-the-vl53l5cx-multizone-timeofflight-ranging-sensor-with-a-wide-field-of-view-ultra-lite-driver-uld-stmicroelectronics.pdf
*/
#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{assert, panic};

use crate::uld_raw::{
    VL_ResultsData,
};
use crate::units::TempC;

// Note: We could also take in 'TARGETS_PER_ZONE' from the ULD C API wrapper.
const TARGETS: usize =
         if cfg!(feature = "targets_per_zone_4") { 4 }
    else if cfg!(feature = "targets_per_zone_3") { 3 }
    else if cfg!(feature = "targets_per_zone_2") { 2 }
    else { 1 };

/*
* Results data, in matrix format.
*
* Note: Scalar metadata ('silicon_temp_degc') that ULD C API treats as a result is being delivered
*       separately. This is mainly a matter of taste: many of the matrix "results" are actually
*       also metadata. Only '.distance_mm' and (likely) '.reflectance_percent' can be seen as
*       actual results. It doesn't really matter.
*/
#[derive(Clone, Debug)]
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
}

impl<const DIM: usize> ResultsData<DIM> {
    /*
    * Provide an empty buffer-like struct; owned usually by the application and fed via 'feed()'.
    */
    #[cfg(not(all()))]
    fn empty() -> Self {

        Self {
            #[cfg(feature = "ambient_per_spad")]
            ambient_per_spad: [[0;DIM];DIM],
            #[cfg(feature = "nb_spads_enabled")]
            spads_enabled: [[0;DIM];DIM],
            #[cfg(feature = "nb_targets_detected")]
            targets_detected: [[0;DIM];DIM],

            #[cfg(feature = "target_status")]
            target_status: [[[TargetStatus::NoTarget;DIM];DIM];TARGETS],

            #[cfg(feature = "distance_mm")]
            distance_mm: [[[0;DIM];DIM];TARGETS],
            #[cfg(feature = "range_sigma_mm")]
            range_sigma_mm: [[[0;DIM];DIM];TARGETS],

            #[cfg(feature = "signal_per_spad")]
            signal_per_spad: [[[0;DIM];DIM];TARGETS],
            #[cfg(feature = "reflectance_percent")]
            reflectance: [[[0;DIM];DIM];TARGETS],
        }
    }

    pub(crate) fn from(raw_results: &VL_ResultsData) -> (Self,TempC) {
        use core::mem::MaybeUninit;

        //validate_raw(raw_results);  // panics if input not according to expectations

        let mut x: Self = {
            let un = MaybeUninit::<Self>::uninit();
            unsafe { un.assume_init() }
        };

        let tempC = x.feed(raw_results);
        (x, tempC)
    }

    fn feed(&mut self, rr: &VL_ResultsData) -> TempC {
        use core::convert::identity;

        // helpers
        //
        // The ULD C API matrix layout is,
        //  - looking _out_ through the sensor so that the SATEL mini-board's PCB text is horizontal
        //    and right-way-up
        //      ^-- i.e. what the sensor "sees" (not how we look at the sensor)
        //  - for a fictional 2x2x2 matrix = only the corner zones
        //
        // Real world:
        //      [A B]   // A₁..D₁ = first targets; A₂..D₂ = 2nd targets; i.e. same target zone
        //      [C D]
        //
        // ULD C API vector:
        //      [A₁ A₂ B₁ B₂ C₁ C₂ D₁ D₂]   // every "zone" is first covered; then next zone
        //
        // Rust note:
        //      'const DIM' generic needs to be repeated for each 'fn'; we cannot use the "outer":
        //          <<
        //              error[E0401]: can't use generic parameters from outer item
        //          <<
        //
        #[allow(dead_code)]
        fn into_matrix_map_o<IN: Copy, OUT, const DIM: usize>(raw: &[IN], offset: usize, out: &mut [[OUT; DIM]; DIM], f: impl Fn(IN) -> OUT) {
            let raw = &raw[..DIM * DIM * TARGETS];      // take only the beginning of the C buffer

            for r in 0..DIM {
                for c in 0..DIM {
                    out[r][c] = f(raw[(r * DIM + c) * TARGETS + offset]);
                }
            }
        }
        #[inline]
        #[allow(dead_code)]
        fn into_matrix_o<X: Copy, const DIM: usize>(raw: &[X], offset: usize, out: &mut [[X; DIM]; DIM]) {     // no mapping
            into_matrix_map_o(raw, offset, out, identity)
        }
        // Zone metadata: 'TARGETS' (and 'offset', by extension) are not involved.
        #[allow(dead_code)]
        fn into_matrix<X: Copy, const DIM: usize>(raw: &[X], out: &mut [[X; DIM]; DIM]) {
            let raw = &raw[..DIM * DIM];      // take only the beginning of the C buffer

            for r in 0..DIM {
                for c in 0..DIM {
                    out[r][c] = raw[r*DIM+c];
                }
            }
        }

        // Metadata: DIMxDIM (just once)
        //
        #[cfg(feature = "ambient_per_spad")]
        into_matrix(&rr.ambient_per_spad, &mut self.ambient_per_spad);
        #[cfg(feature = "nb_spads_enabled")]
        into_matrix(&rr.nb_spads_enabled, &mut self.spads_enabled);
        #[cfg(feature = "nb_targets_detected")]
        into_matrix(&rr.nb_target_detected, &mut self.targets_detected);

        // Results: DIMxDIMxTARGETS
        //
        for i in 0..TARGETS {
            #[cfg(feature = "target_status")]
            into_matrix_map_o(&rr.target_status, i, &mut self.target_status[i], TargetStatus::from_uld);

            // We tolerate '.distance_mm' == 0 for non-existing data (where '.target_status' is 0); no need to check.
            //
            #[cfg(feature = "distance_mm")]
            into_matrix_map_o(&rr.distance_mm, i, &mut self.distance_mm[i],
            |v: i16| -> u16 {
                assert!(v >= 0, "Unexpected 'distance_mm' value: {} < 0", v); v as u16
            });
            #[cfg(feature = "range_sigma_mm")]
            into_matrix_o(&rr.range_sigma_mm, i, &mut self.range_sigma_mm[i]);

            #[cfg(feature = "reflectance_percent")]
            into_matrix_o(&rr.reflectance, i, &mut self.reflectance[i]);
            #[cfg(feature = "signal_per_spad")]
            into_matrix_o(&rr.signal_per_spad, i, &mut self.signal_per_spad[i]);
        }

        TempC(rr.silicon_temp_degc)
    }
}

//---
// Target status
//
// Note: Vendor docs (UM2884 Rev.5; chapter 5.5; Table 4) gives detailed explanations for values
//      0..13 and 255. We intend to provide enums for values that are _actually seen_, so that
//      application code doesn't need to deal with integers. Where multiple choices exist, they
//      are provided  as the inner values.
//
#[derive(Copy, Clone, Debug)]       // 'Clone' needed for 'ResultsData' to be cloneable.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetStatus {
    NotUpdated,         // 0    "Ranging data are not updated" (O)
    Valid,              // 5    "Range valid" = 100% valid
    SemiValid(u8),      // 6    "Wrap around not performed (typically the first range)"
                        // 9    "Range valid with large pulse (may be due to a merged target)"
    NoTarget,           // 255  "No target detected (only if number of targets detected is enabled)"
    Error(u8),          // 1    "Signal rate too slow on SPAD array"
                        // 2    "Target phase"
                        // 3    "Sigma estimator too high"
                        // 4    "Target consistency failed" (O)
                        // 7    "Rate consistency failed"
                        // 8    "Signal rate too low for the current target"
                        // 10   "Range valid, but no target detected at previous range"
                        // 11   "Measurement consistency failed"
                        // 12   "Target blurred by another one, due to sharpener"
                        // 13   "Target detected but inconsistent data. Frequently happens for secondary targets." (O)
                        //
                        //      (O): Observed in wild
}

impl TargetStatus {
    fn from_uld(v: u8) -> Self {
        match v {
            0 => { Self::NotUpdated }
            5 => { Self::Valid },
            6 | 9 => { Self::SemiValid(v) },
            255 => { Self::NoTarget },
            ..=13 => { Self::Error(v) },
            v => panic!("Unexpected value {} for target status", v),
        }
    }
}

/***R
/*
* Validates that the input we get from ULD C API is according to assumptions (i.e. validate our
* ASSUMPTIONS; the data of course are fine!!!).
*/
fn validate_raw<const DIM: usize>(rr: &VL_ResultsData) {

    // helpers
    //
    #[allow(dead_code)]
    fn assert_matrix_o<X: Copy>(raw: &[X], assert_f: fn(X) -> ()) {
        let raw = &raw[..DIM * DIM * TARGETS];      // take only the beginning of the C buffer

        for r in 0..DIM {
            for c in 0..DIM {
                for offset in 0..TARGETS {  // the targets are in consecutive bytes; best to have this inmost
                    let v = raw[(r * DIM + c) * TARGETS + offset];
                    assert_f(v);
                }
            }
        }
    }
    // Zone metadata: 'TARGETS' (and 'offset', by extension) are not involved.
    fn assert_matrix<X: Copy>(raw: &[X], assert_f: fn(X) -> ()) {
        let raw = &raw[..DIM * DIM];      // take only the beginning of the C buffer

        for r in 0..DIM {
            for c in 0..DIM {
                out[r][c] = raw[r*DIM+c];
            }
        }
    }

    // Metadata: DIMxDIM (just once)
    //
    // '.ambient_per_spad'
    //  <<
    //      [INFO ] .ambient_per_spad: [[1, 2, 0, 3], [1, 4, 1, 0], [2, 1, 3, 0], [9, 2, 1, 2]]
    //  <<
    // true

    // '.spads_enabled'
    //  <<
    //      [INFO ] .spads_enabled:    [[1280, 3328, 3584, 4352], [1024, 2816, 3584, 3584], [1280, 2816, 4352, 3328], [1280, 3584, 3584, 2816]]
    //  <<
    // true

    // '.targets_detected'
    //  <<
    //      [INFO ] .targets_detected: [[1, 1, 2, 2], [1, 1, 1, 1], [1, 1, 1, 1], [1, 2, 2, 1]]
    //  <<
    //
    // Q: Can this ever be zero?
    //
    #[cfg(feature = "nb_targets_detected")]
    assert_matrix(&rr.nb_target_detected, |x| => { assert_gt(x, 0, "'.nb_target_detected' == 0"); });

    // Results: DIMxDIMxTARGETS
    //
    for i in 0..TARGETS {
        // '.target_status'
        //  <<
        //      [INFO ] .target_status:    [[[Valid(5), Valid(5), Valid(5), Valid(5)], [Valid(5), Valid(5), Valid(5), Valid(5)], [Valid(5), Valid(5), Valid(5), Valid(5)], [Valid(5), Valid(5), Valid(5), Valid(5)]], [[Other(0), Other(0), Valid(5), Other(13)], [Other(0), Other(0), Other(0), Other(0)], [Other(0), Other(0), Other(0), Other(0)], [Other(0), Other(4), Other(13), Other(4)]]]
        //  <<
        assert_matrix_o(&rr.target_status, |x| { assert(x.within_range(...)) });

        // '.distance_mm'
        // <<
        //      [INFO ] .distance_mm:      [[[13, 13, 12, 5], [12, 23, 23, 12], [13, 17, 19, 10], [10, 13, 6, 0]], [[0, 0, 259, 753], [0, 0, 0, 0], [0, 0, 0, 0], [0, 597, 657, 765]]]
        // <<
        //      normally > 0
        //      can be == 0 if '.target_status' is 0
        //
        #[cfg(feature = "distance_mm")]
        into_matrix_map_o(&rr.distance_mm, i, &mut self.distance_mm[i],
                          |v: i16| -> u16 {
                              assert!(v >= 0, "Unexpected 'distance_mm' value: {} < 0", v); v as u16
                          });

        // '.range_sigma_mm'
        //  <<
        //      [INFO ] .range_sigma_mm:   [[[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]], [[0, 0, 3, 11], [0, 0, 0, 0], [0, 0, 0, 0], [0, 34, 18, 24]]]
        //  <<
        #[cfg(feature = "range_sigma_mm")]
        into_matrix_o(&rr.range_sigma_mm, i, &mut self.range_sigma_mm[i]);

        // '.reflectance'
        //  <<
        //      [INFO ] .reflectance:      [[[1, 0, 0, 0], [1, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]], [[0, 0, 11, 17], [0, 0, 0, 0], [0, 0, 0, 0], [0, 9, 12, 14]]]
        //  <<
        #[cfg(feature = "reflectance_percent")]
        into_matrix_o(&rr.reflectance, i, &mut self.reflectance[i]);

        // '.signal_per_spad'
        //  <<
        //      [INFO ] .signal_per_spad:  [[[5171, 1655, 1377, 1506], [4859, 1815, 1372, 1480], [4910, 1716, 1395, 1717], [4623, 1630, 1359, 2050]], [[0, 0, 119, 21], [0, 0, 0, 0], [0, 0, 0, 0], [0, 17, 20, 17]]]
        //  <<
        #[cfg(feature = "signal_per_spad")]
        into_matrix_o(&rr.signal_per_spad, i, &mut self.signal_per_spad[i]);
    }

    TempC(rr.silicon_temp_degc)
}
***/
