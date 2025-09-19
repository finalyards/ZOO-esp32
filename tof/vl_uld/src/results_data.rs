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
*
* Note: '.range_sigma_mm' does belong with the results (could be within 'Meas'), but it's deemed
*       to not be needed very often, and including it there would just feel complex. The author
*       does think that the valid/semi-valid/invalid applies to it (as well as '.distance_mm');
*       this could be used as a water-shed rule to decide where data fits best. Observe.
*/
#[derive(Clone, Debug)]
pub struct ResultsData<const DIM: usize> {      // DIM: 4,8
    // Metadata: DIMxDIM matrix, regardless of 'TARGETS'
    //
    #[cfg(feature = "ambient_per_spad")]
    pub ambient_per_spad: [[u32; DIM]; DIM],
    #[cfg(feature = "nb_spads_enabled")]
    pub spads_enabled: [[u32; DIM]; DIM],

    // Actual results: DIMxDIMxTARGETS
    pub meas: [[[Meas; DIM]; DIM]; TARGETS],
    #[cfg(feature = "range_sigma_mm")]
    pub range_sigma_mm: [[[u16; DIM]; DIM]; TARGETS],

    #[cfg(feature = "reflectance_percent")]
    pub reflectance: [[[u8; DIM]; DIM]; TARGETS],
    #[cfg(feature = "signal_per_spad")]
    pub signal_per_spad: [[[u32; DIM]; DIM]; TARGETS],
}

impl<const DIM: usize> ResultsData<DIM> {

    pub(crate) fn from(raw_results: &VL_ResultsData) -> (Self,TempC) {
        use core::mem::MaybeUninit;

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

        // Results: DIMxDIMxTARGETS
        //
        for i in 0..TARGETS {

            // DEBUG: print out data; this advances our understanding of it!
            //          See -> 'DEVS/Data analysis.md'
            #[cfg(false)]
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

            // '.meas[]' merges '.target_status', '.distance_mm' and "target detected" checks
            // into one; intended to make application level data use trivial (we help them select
            // what data is valid).
            //
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

        // Check out multi-target results. They may contain valid (or semi-valid) measurements that,
        // when compared across the targets, are TOO CLOSE TOGETHER, breaching the sensor's spec
        // that there should be a limit of min. 600mm.
        //
        // Turn the later results into errors, so they wouldn't be used by the application level
        // (it still *can*, but it's harder this way!).
        //
        #[cfg(feature="_multi")]
        for i in 1..TARGETS {
            use Meas::{Valid, SemiValid};
            use Comment::TargetsTooClose;

            let &mut mut meas = &mut self.meas[i];
            let &mut meas_prev = &mut self.meas[i-1];

            for r in 0..DIM {
                for c in 0..DIM {
                    match (meas[r][c], meas_prev[r][c]) {
                        (Valid(x) | SemiValid(x, ..), Valid(x_prev) | SemiValid(x_prev, ..)) => {
                            if x-x_prev < 600 /*mm*/ {
                                let st = meas[r][c].status();
                                let st_prev = meas_prev[r][c].status();

                                warn!("Measurements in same zone (following targets) are too close! ({}, {})", (x_prev, st_prev),(x, st));
                                meas[r][c] = Meas::Error(x as i16, st, TargetsTooClose);    // return to lower abstraction level; _not_ a valid result!!
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        TempC(rr.silicon_temp_degc)
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
    Error(i16, u8, Comment)     // Things the application can either ignore, use with awareness in some cases (targets too close) or report as data glitches to the user!
}

impl Meas {
    #[allow(dead_code)]         // only needed for 'feature="_multi"' builds
    fn status(&self) -> u8 {
        match self {
            Self::Valid(_) => 5,
            Self::SemiValid(_, st) => *st,
            Self::Invalid(_, st) => *st,
            Self::Error(_,st,_) => *st
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Comment {
    TargetStatusButNoTarget,    // target "not detected"; target_status = 5|6|9; v not checked
    TargetsTooClose             // two "detected targets", but values < 600mm apart
}


/***R prior code
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
}***/
