/*
* Ranging: actually getting measurements from the sensor.
*/

use core::mem::MaybeUninit;
use defmt::{assert, panic};

#[allow(unused_imports)]
use crate::uld_raw::{
    VL53L5CX_Configuration,
    VL53L5CX_ResultsData,
        //
    vl53l5cx_start_ranging,
    vl53l5cx_check_data_ready,
    vl53l5cx_get_ranging_data,
    vl53l5cx_set_resolution,
    vl53l5cx_set_ranging_frequency_hz,
    vl53l5cx_set_ranging_mode,
    vl53l5cx_set_integration_time_ms,
    vl53l5cx_set_sharpener_percent,
    vl53l5cx_set_target_order,
    vl53l5cx_stop_ranging,
    ST_OK,
    RangingMode as RangingMode_R,
    Resolution
};
pub use crate::uld_raw::{   // pass-throughs
    TargetOrder,
};
use crate::{
    Result,
    results_data::ResultsData,
    units::{Ms, Hz}
};

/* Documentation on 'ResultsData' (aka 'VL53L5CX_ResultsData'):
*
pub struct VL53L5CX_ResultsData {
    pub silicon_temp_degc: i8,                  // temperature within the sensor [°C]
    pub nb_target_detected: [u8; 64usize],      // values in range [1..'targets_per_zone_{1..4}'] (inclusive); max steered by feature
    pub distance_mm: [i16; 64usize],            // on 'feature='distance_mm'; tbd. (sample?)
    pub target_status: [u8; 64usize],           // values: 5,6,10,255   // doc: 5,9 are proper; rest imply unreliable results in that zone
}
    The buffers are to be read as:
    - 4x4 reso: tbd.
    - 8x8 reso: tbd.
*/
// tbd. We likely end up copying values, so that the Rust side can get 'ResultsData_4x4' or 'ResultsData_8x8' classes.

// Adding to the C API by joining integration time with the ranging mode - since integration time
// only applies to one of the modes.
//
#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    CONTINUOUS,
    AUTONOMOUS(Ms,Hz)    // (integration time, ranging frequency)
}
use Mode::{CONTINUOUS, AUTONOMOUS};

impl Mode {
    fn as_raw(&self) -> RangingMode_R {
        match self {
            CONTINUOUS => RangingMode_R::CONTINUOUS,
            AUTONOMOUS(_,_) => RangingMode_R::AUTONOMOUS
        }
    }
}

/*
* We provide a setup for each separate 'Ranging' session. This encloses the resolution as a type,
* and also helps ensure that the C ULD API functions get called in a specific order (some vendor
* docs recommend certain orders.. anyways, it makes things more predictable). Other demands,
* according to vendor docs are:
*
*   - "Integration time must be [...] lower than the ranging period, for a selected resolution."
*       = Integration happens within each ranging period. In fact, there should be a 1ms margin
*       left.
*   - "[...] select your [ranging] resolution before [setting the frequency]"
*       - range is [1..60] (4x4) or [1..15 (8x8); ranges inclusive
*   - Integration time and frequency only apply to AUTONOMOUS ranging mode
*   - Integration time range is (for all resolutions): [2ms..1000ms]; inclusive
*   - Sharpener range is [0..99]; inclusive; (0 = disabled)
*/
pub struct RangingConfig<const DIM: u8 = 4> {
    mode: Mode,      // also carries ranging frequency and integration time for 'AUTONOMOUS'
    sharpener_prc: Option<u8>,      // 'None' | 'Some(1..=99)'
    target_order: TargetOrder,
}

impl<const DIM: u8> RangingConfig<DIM> {
    /* We allow construction to make potentially incompatible combinations, but check them within
    * '.apply()'. This is a compromise between simplicity and robustness. Note that some obvious
    * type-system robustness has been done, e.g. bundling ranging frequency and integration times
    * with the ranging mode (since those only apply to one mode).
    */
    pub fn with_sharpener_prc(/*move*/ self, v: Option<u8>) -> Self {
        Self { sharpener_prc: v, ..self }
    }

    pub fn with_target_order(/*move*/ self, order: TargetOrder) -> Self {
        Self { target_order: order, ..self }
    }

    pub fn with_mode(/*move*/ self, mode: Mode) -> Self {
        Self { mode, ..self }
    }

    fn validate(&self) {
        match self.mode {
            AUTONOMOUS(Ms(integration_time_ms), Hz(freq)) => {
                assert!((2..=1000).contains(&integration_time_ms), "Integration time out of range");

                // "The sum of all integration times + 1 ms overhead must be lower than the measurement
                // period. Otherwise, the ranging period is automatically increased." (src: UM2884 - Rev 5 p.9)
                //
                // "4x4 is composed of one integration time"
                // "8x8 is composed of four integration times" (same src)
                //
                let n = match DIM { 4 => 1, 8 => 4, _ => unreachable!() };

                // Note: The test itself is calculated so that inaccuracies don't occur (multiplication instead of division).
                //      The error message parameter is let go more loosely; not a problem.
                //
                assert!((integration_time_ms+1)*n*(freq as u16) < 1000,
                    "Integration time exceeds the available window ({}ms)", (1000_u16/(n * freq as u16))-1
                );

                let freq_range = 1..=match DIM { 4 => 15, 8 => 60, _ => unreachable!() };
                assert!(freq_range.contains(&freq), "Frequency out of range");
            },
            _ => {}
        }

        match self.sharpener_prc {
            Some(v) => { assert!((1..=99).contains(&v), "Sharpener-% out of range") },
            None => {}
        }

        // "Integration time must be [...] lower than the ranging period, for a selected resolution." (source: C ULD sources)
        //  tbd. Uncypher what that means, check it as well.
    }

    fn apply(&self, vl: &mut VL53L5CX_Configuration) -> Result<()> {
        self.validate();    // may panic

        // Set the resolution first. UM2884 (Rev 5) says:
        //  "['..._set_resolution()'] must be used before updating the ranging frequency"

        // ULD C API uses the vector lengths: 16 (4x4), 64 (8x8); not available as enums or #define's
        match unsafe { vl53l5cx_set_resolution(vl, DIM * DIM) } {
            ST_OK => Ok(()),
            e => Err(e)
        }?;

        if let AUTONOMOUS(Ms(ms), Hz(freq)) = self.mode {
            match unsafe { vl53l5cx_set_integration_time_ms(vl, ms as u32) } {
                ST_OK => Ok(()),
                e => Err(e)
            }?;
            match unsafe { vl53l5cx_set_ranging_frequency_hz(vl, freq) } {
                ST_OK => Ok(()),
                e => Err(e)
            }?;
        }

        match unsafe { vl53l5cx_set_ranging_mode(vl, self.mode.as_raw() as _) } {
            ST_OK => Ok(()),
            e => Err(e)
        }?;

        match unsafe { vl53l5cx_set_sharpener_percent(vl, self.sharpener_prc.unwrap_or(0)) } {
            ST_OK => Ok(()),
            e => Err(e)
        }?;

        match unsafe { vl53l5cx_set_target_order(vl, self.target_order as _) } {
            ST_OK => Ok(()),
            e => Err(e)
        }?;

        Ok(())
    }
}

impl<const DIM: u8> Default for RangingConfig<DIM> {
    // defaults are those mentioned in the vendor docs.
    fn default() -> Self {
        Self {
            sharpener_prc: None,
            target_order: TargetOrder::STRONGEST,
            mode: AUTONOMOUS(Ms(5),Hz(1))
        }
    }
}

pub struct Ranging<'a, const DIM: u8> {
    vl: &'a mut VL53L5CX_Configuration,
    buf: VL53L5CX_ResultsData       // results of the latest '.get_data()' call; overwritten for each scan
}

impl<'b: 'c,'c,const DIM: u8> Ranging<'c,DIM> {
    pub(crate) fn new_maybe(vl: &'b mut VL53L5CX_Configuration, cfg: &RangingConfig<DIM>) -> Result<Self> {
        cfg.apply(vl)?;

        // This causes a little bit copying, but is otherwise clean.
        let buf = unsafe {
            MaybeUninit::<VL53L5CX_ResultsData>::zeroed().assume_init()
        };

        match unsafe { vl53l5cx_start_ranging(vl) } {
            ST_OK => {
                Ok(Self{vl,buf})
            },
            e => Err(e)
        }
    }

    pub fn is_ready(&mut self) -> Result<bool> {
        let mut tmp: u8 = 0;
        match unsafe { vl53l5cx_check_data_ready(self.vl, &mut tmp) } {
            ST_OK => Ok(tmp != 0),
            e => Err(e)
        }
    }

    /*
    * Collect results from the last successful scan. You can call this either after
    *   a) checking for valid results using 'poll_ready()', or..
    *   b) having gotten a hardware signal showing a scan is complete.
    *
    //tbd. Describe what happens, if you call here before a scan is ready.
    *
    * The reference returned is to a buffer. It remains valid only until the next time 'get_data'
    * is called (that should be enough for apps).
    */
    pub fn get_data(&mut self) -> Result<ResultsData<DIM>> {

        match unsafe { vl53l5cx_get_ranging_data(self.vl, &mut self.buf) } {
            ST_OK => {
                let tmp: ResultsData<DIM> = self.buf.into();
                Ok(tmp)
            },
            e => Err(e)
        }
    }
}

impl<const DIM: u8> Drop for Ranging<'_,DIM> {
    fn drop(&mut self) {
        match unsafe { vl53l5cx_stop_ranging(self.vl) } {
            ST_OK => (),
            e => panic!("Stop ranging failed; st={}", e)
        }
    }
}
