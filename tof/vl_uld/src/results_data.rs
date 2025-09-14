/*
* Convert data received from the ULD C API to more easy-to-use formats:
*   - 1D vectors -> 2D matrices
*   - integers -> enums or tuple structs
*   - '.nb_targets_detected', '.distance_mm' and '.target_status' merged into one, "measurement"
*       enum, making use of the data easier - without losing any of its details!!!
*
* It is by design that these conversions happen already at the ULD level.
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
use defmt::{assert, debug, panic, warn};

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
    //R #[cfg(feature = "nb_targets_detected")]
    //R pub targets_detected: [[u8; DIM]; DIM],     // 1..{X in 'targets_per_zone_X' feature}

    // Actual results: DIMxDIMxTARGETS
    //#[cfg(feature = "target_status")]
    //pub target_status: [[[TargetStatus; DIM]; DIM]; TARGETS],

    //#[cfg(feature = "distance_mm")]
    //pub distance_mm: [[[u16; DIM]; DIM]; TARGETS],
    pub meas: [[[Meas; DIM]; DIM]; TARGETS],
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
    #[cfg(false)]
    fn empty() -> Self {

        Self {
            #[cfg(feature = "ambient_per_spad")]
            ambient_per_spad: [[0;DIM];DIM],
            #[cfg(feature = "nb_spads_enabled")]
            spads_enabled: [[0;DIM];DIM],
            //R #[cfg(feature = "nb_targets_detected")] // these could be gone from the API
            //R targets_detected: [[0;DIM];DIM],

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
            let un = MaybeUninit::<Self>::zeroed(); // tbd. 'uninit', when things work, again
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
        #[allow(dead_code)]
        fn into_matrix_map_o_pos<IN: Copy, OUT, const DIM: usize>(raw: &[IN], offset: usize, out: &mut [[OUT; DIM]; DIM], f: impl Fn(IN, usize) -> OUT) {
            let raw = &raw[..DIM * DIM * TARGETS];      // take only the beginning of the C buffer

            for r in 0..DIM {
                for c in 0..DIM {
                    let x: usize = r * DIM + c;
                    out[r][c] = f(raw[x*TARGETS + offset], x);
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

        //R#[cfg(feature = "nb_targets_detected")]
        //Rinto_matrix(&rr.nb_target_detected, &mut self.targets_detected);

        //R Validity check: expect '.nb_target_detected' to be always >= 1.
        #[cfg(false)]   //R disabled; conversion does it, below
        {
            let raw = &rr.nb_target_detected[..DIM * DIM];  // take only the beginning of the C buffer

            for r in 0..DIM {
                for c in 0..DIM {
                    let v = raw[r*DIM+c];
                    assert!( v>0, "Unexpected: no target detected!");
                }
            }
        }

        // Results: DIMxDIMxTARGETS
        //
        for i in 0..TARGETS {

            // DEBUG: print out data; this advances our understanding of it!
            //          See -> 'DEVS/Data analysis.md'
            //#[cfg(false)]
            {
                use core::mem::MaybeUninit;

                let r_nb = &rr.nb_target_detected[..DIM * DIM];
                let r_dist = &rr.distance_mm[..DIM * DIM * TARGETS];
                let r_ts = &rr.target_status[..DIM * DIM * TARGETS];

                // Output data that's the same for all targets.
                if i==0 {
                    defmt::trace!("\n\tnb_target_detected: {}\n\tRAW distance_mm: {}\n\tRAW target_status: {}", r_nb, r_dist, r_ts);
                }

                // Collect the current target's data (they are scattered; cannot use slicing)
                //
                // Rust doesn't allow 'DIM*DIM' to be used (since template param), but we know
                // the debugging happens with 'DIM'==4.
                //
                // To be safe, make then 64 long; slicing will only expose the necessary part!
                //
                let mut dist_buf: [i16;64] = unsafe { MaybeUninit::zeroed().assume_init() };
                let mut ts_buf: [u8;64] = unsafe { MaybeUninit::zeroed().assume_init() };

                for r in 0..DIM {
                    for c in 0..DIM {
                        let v = r_dist[(r*DIM+c)*TARGETS + i];
                        dist_buf[r*DIM+c] = v;

                        let v = r_ts[(r*DIM+c)*TARGETS + i];
                        ts_buf[r*DIM+c] = v;
                    }
                }

                // Target specific data
                defmt::trace!("\n\tdistance_mm (#{}): {}\n\ttarget_status: {}", i, dist_buf[..DIM*DIM], ts_buf[..DIM*DIM]);
            }

            //R #[cfg(feature = "target_status")]
            //R into_matrix_map_o(&rr.target_status, i, &mut self.target_status[i], TargetStatus::from_uld);

            // REMOVE, since if pointing at the sky, there shouldn't be any measurements even for
            // the first target.
            //
            /***R
            #[cfg(all(not(feature = "_multi"), feature = "distance_mm"))]
            into_matrix_map_o_pos(&rr.distance_mm, i, &mut self.distance_mm[i],
                |v: i16, pos: usize| -> u16 {
                      assert!(TARGETS == 1);
                      let i2: usize = pos*TARGETS + i;    // index for per-target data ('target_status')
                      let i3: usize = pos + i;            // index for meta data

                      let target_status = rr.target_status[i2];
                      let detected = rr.nb_target_detected[i3];

                      assert!(detected == 1 ||false, "No detection in single-target mode");
                      assert!(v > 0, "Unexpected value (single target mode): {} <= 0", v);

                      match target_status {
                          //...|... => {} // seen; ok
                          5|6|9 => {}  // valid and semi-valid
                          x => {
                              warn!("Unseen '.target_status' (single target mode): {}", x);
                          }
                      };

                      v as u16
                  });***/

            // '.meas[]' gets always filled
            into_matrix_map_o_pos(&rr.distance_mm, i, &mut self.meas[i],
            |v: i16, pos: usize| -> Meas {
                // L5CX: keeps the values >= 0.
                // L8: also provides negative values, at times.
                //
                #[cfg(feature = "vl53l5cx")]
                assert!(v >= 0, "Unexpected '.distance_mm' value: {} < 0", v);

                let i2: usize = pos*TARGETS + i;    // index for per-target data ('target_status')
                let i3: usize = pos + i;            // index for meta data

                let target_status = rr.target_status[i2];
                let detected = rr.nb_target_detected[i3];

                let /*mut*/ ret: Meas;

                if (i as u8) < detected {     // "target detected"
                    // Expecting target status:
                    //      5:   good measurement, (valid '.distance_mm' > 0)
                    //      6|9: semi-valid measurement ("50%" trust; '.distance_mm' > 0)
                    //      ...: invalid measurement ("0%" trust; '.distance_mm' > 0)
                    //
                    // Other status (seen):
                    //      ...tbd.
                    //
                    ret = match (target_status,v > 0) {
                        (5,true) => {
                            Meas::Valid(v as u16)
                        },
                        (x@ 6|x @ 9,true) => {
                            Meas::SemiValid(v as u16, x)
                        },
                        (x,_) => {
                            Meas::Invalid(v,x)
                        },
                    };

                    // tbd. move this to a later stage; going through all >1 targets (with direct
                    //      indices to the results).  Turn those into their own type of result.
                    //
                    // Warn, if difference to the previous target is < 600mm. This occurs in data,
                    // but is against the hardware specs.
                    //
                    /***
                    if i>0 {
                        match ret {
                            Meas::Valid(..) | Meas::SemiValid(..) => {  // target_status: 5|6|9, have value in 'v'
                                let v_prior = rr.distance_mm[pos*TARGETS + (i-1)];
                                let

                                let diff = v-v_prior;
                                if diff < 600 {
                                    warn!("Measurements from adjacent targets (same zone) less than 60cm apart: {} < 600\n\tprior: {}\n\tthis: {}", diff, v_prior, v);
                                }
                                // tbd. can set 'ret' here, if we want to invalidate it...
                            },
                            Meas::Invalid(..) | Meas::Error(..) => {}
                        }
                    }***/

                    ret

                } else {    // missing target (but '.target_status' and '.distance_mm' can still show to be surprisingly vibrant!)

                    // 'target_status' seen on such data:
                    //      0: "not updated"
                    //      2: "target phase"
                    //      4: "target consistency failed"
                    //
                    match target_status {
                        5|6|9 => {  // very weird, if 'valid', 'semi-valid' target status occurs when not a "detected target". Isn't it...
                            warn!("Valid or semi-valid measurement ({}, {}), but doesn't fit a \"detected target\": offset {} is beyond {}.",
                                target_status, v, i, detected);

                            Meas::Error(v,target_status, Comment::TargetStatusButNoTarget)
                        }
                        x => {
                            Meas::Invalid(v,x)
                        }
                    }
                }
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
// Note: Adding 'Invalid' was an author's choice (to be argued), based on *observing live data*.
//      We can aim at grouping the "other" fields in such ways, if it brings any application level
//      benefits / understanding to the data. The fact is that such error numbers reveal too much
//      of the inner workings of the sensor, and may be best to be left at that - as debugging
//      tools for the vendor's engineers.
//
#[cfg(false)]
#[derive(Copy, Clone, Debug)]       // 'Clone' needed for 'ResultsData' to be cloneable.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetStatus {
    Valid,              // 5    ✅"Range valid" = 100% valid
    SemiValid(u8),      // 6    "Wrap around not performed (typically the first range)"
                        // 9    ✅"Range valid with large pulse (may be due to a merged target)"
    Invalid(u8),        // 2    ✅"Target phase"
                        // 4    ✅"Target consistency failed"
    //NotUpdated,         // 0    ✅"Ranging data are not updated"
    //NoTarget,           // 255  "No target detected (only if number of targets detected is enabled)"
    Other(u8),          // 1    "Signal rate too slow on SPAD array"
                        // 3    "Sigma estimator too high"
                        // 7    "Rate consistency failed"
                        // 8    "Signal rate too low for the current target"
                        // 10   "Range valid, but no target detected at previous range"
                        // 11   "Measurement consistency failed"
                        // 12   "Target blurred by another one, due to sharpener"
                        // 13   ✅"Target detected but inconsistent data. Frequently happens for secondary targets."
                        //
                        //      ✅: Observed in wild
}

#[cfg(false)]
impl TargetStatus {
    fn from_uld(v: u8) -> Self {
        match v {
            5 => { Self::Valid },
            6 | 9 => { Self::SemiValid(v) },
            2 | 4 => { Self::Invalid(v) },
            //0 => { Self::NotUpdated }
            //255 => { Self::NoTarget },
            ..=13|255 => { Self::Other(v) },
            x => panic!("Unexpected target status: {}", x),
        }
    }
}

#[derive(Copy, Clone, Debug)]       // 'Clone' needed for 'ResultsData' to be cloneable.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Meas {
    Valid(u16),                 // 100% confidence (has target; target status = 5)
    SemiValid(u16, u8),         // 50% confidence (has target; target status = 6|9)
    Invalid(i16, u8),           // 0% confidence (all the rest):
                                //  - distance may be 0 or negative (do not use!)
                                //  - target status = anything but the above (not 5|6|9)
    Error(i16, u8, Comment)     // Things the application can either ignore, or report as data errors!
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Comment {
    TargetStatusButNoTarget,     // target "not detected"; target_status = 5|6|9; v not checked
    TargetsTooClose             // two "detected targets", but values < 600mm apart
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
    //R #[cfg(feature = "nb_targets_detected")]
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
