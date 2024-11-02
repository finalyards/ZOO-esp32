/*
* state_ranging.rs:
*
*   'RangingConfig':  how the ranging should be done
*   'State_Ranging':  handle to the sensor once ranging is ongoing
*/
#[cfg(feature = "defmt")]
use defmt::{assert, panic, trace};

use crate::uld_raw::{VL53L5CX_Configuration, vl53l5cx_start_ranging, vl53l5cx_check_data_ready, vl53l5cx_get_ranging_data, vl53l5cx_set_resolution, vl53l5cx_set_ranging_frequency_hz, vl53l5cx_set_ranging_mode, vl53l5cx_set_integration_time_ms, vl53l5cx_set_sharpener_percent, vl53l5cx_set_target_order, vl53l5cx_stop_ranging, ST_OK, RangingMode as RangingMode_R, Resolution as Resolution_R, VL53L5CX_ResultsData};

// Enums from 'uld_raw.rs' that are exposed in the public API
pub use crate::uld_raw::{
    TargetOrder
};

use crate::{
    results_data::ResultsData,
    units::{MsU16, HzU8, TempC, ExtU32 as _},
    Error,
    Result,
};

/* Documentation on vendor ULD C API:
pub struct VL53L5CX_ResultsData {
    pub silicon_temp_degc: i8,                  // temperature within the sensor [°C]
    pub nb_target_detected: [u8; 64usize],      // values in range [1..'targets_per_zone_{1..4}'] (inclusive); max steered by feature
    pub distance_mm: [i16; 64usize],            // note that values could be negative (but never are)
    pub target_status: [u8; 64usize],           // values: 5,6,10,255   // doc: 5,9 are proper; rest imply unreliable results in that zone
}
    The buffers are to be read as:
    - 4x4 reso: tbd.
    - 8x8 reso: tbd.
*/

// tbd. #review
/*
* The 'Resolution' trait became an implementation detail, and a plain function, expressing the
* dimensions in app level being by the 'DIM' const generic.
*
* This defines which resolutions the system is able to play with; if there are new ones (for similar
* sensors, in the future), adding the details here will open them up for the larger 'Ranging' and
* other such structs.
*/
fn get_reso_details<const DIM: usize>() -> (Resolution_R /*Raw entry*/, u8 /*integration time*/, HzU8 /*max freq*/) {
    match DIM {
        4 => { (Resolution_R::_4X4, 1, 15.Hz()) },
        8 => { (Resolution_R::_8X8, 4, 60.Hz()) },
        _ => unreachable!()
    }
}

// Adding to the C API by joining integration time with the ranging mode - since integration time
// only applies to one of the modes.
//
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    CONTINUOUS,
    AUTONOMOUS(MsU16,HzU8)    // (integration time, ranging frequency)
}
use Mode::{CONTINUOUS, AUTONOMOUS};
use crate::state_hp_idle::State_HP_Idle;

impl Mode {
    fn as_uld(&self) -> RangingMode_R {
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
#[derive(Clone)]
pub struct RangingConfig<const DIM: usize = 4> {
    mode: Mode,      // also carries ranging frequency and integration time for 'AUTONOMOUS'
    sharpener_prc: Option<u8>,      // 'None' | 'Some(1..=99)'
    target_order: TargetOrder,
}

impl<const DIM: usize> RangingConfig<DIM> {
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
        let (_,R_INTEGRATION_TIMES_N, R_FREQ_RANGE_MAX): (_,u8,HzU8) = get_reso_details::<DIM>();

        match self.mode {
            AUTONOMOUS(MsU16(integration_time_ms), HzU8(freq)) => {
                assert!((2..=1000).contains(&integration_time_ms), "Integration time out of range");

                // "The sum of all integration times + 1 ms overhead must be lower than the measurement
                // period. Otherwise, the ranging period is automatically increased." (src: UM2884 - Rev 5 p.9)
                //
                // "4x4 is composed of one integration time"
                // "8x8 is composed of four integration times" (same src)
                //
                let n = R_INTEGRATION_TIMES_N;  // 1 (4x4); 4 (8x8)

                // Note: The test itself is calculated so that inaccuracies don't occur (multiplication instead of division).
                //      The error message parameter is let go more loosely; not a problem.
                //
                assert!((integration_time_ms as u16+1)*(n as u16)*(freq as u16) < 1000,
                    "Integration time exceeds the available window ({}ms)", (1000_u16/(n as u16 * freq as u16))-1
                );

                let freq_range = 1..(R_FREQ_RANGE_MAX.0 as u8);    // 1..15 (4x4); 1..60 (8x8)
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
        let ULD_RESO: Resolution_R = get_reso_details::<DIM>().0;

        // Set the resolution first. UM2884 (Rev 5) says:
        //  "['..._set_resolution()'] must be used before updating the ranging frequency"

        match unsafe { vl53l5cx_set_resolution(vl, ULD_RESO as u8) } {  // reso value: 16 (4x4); 64 (8x8)
            ST_OK => Ok(()),
            e => Err(Error(e))
        }?;

        if let AUTONOMOUS(MsU16(ms), HzU8(freq)) = self.mode {
            match unsafe { vl53l5cx_set_integration_time_ms(vl, ms as u32) } {
                ST_OK => Ok(()),
                e => Err(Error(e))
            }?;
            match unsafe { vl53l5cx_set_ranging_frequency_hz(vl, freq as u8) } {
                ST_OK => Ok(()),
                e => Err(Error(e))
            }?;
        }

        match unsafe { vl53l5cx_set_ranging_mode(vl, self.mode.as_uld() as _) } {
            ST_OK => Ok(()),
            e => Err(Error(e))
        }?;

        match unsafe { vl53l5cx_set_sharpener_percent(vl, self.sharpener_prc.unwrap_or(0)) } {
            ST_OK => Ok(()),
            e => Err(Error(e))
        }?;

        match unsafe { vl53l5cx_set_target_order(vl, self.target_order as _) } {
            ST_OK => Ok(()),
            e => Err(Error(e))
        }?;

        Ok(())
    }
}

impl<const DIM: usize> Default for RangingConfig<DIM> {
    // defaults are those mentioned in the vendor docs.
    // Note: Resolution default comes from the 'RangingConfig' struct definition (hopefully!).
    //
    fn default() -> Self {
        Self {
            sharpener_prc: None,
            target_order: TargetOrder::STRONGEST,
            mode: AUTONOMOUS(5.ms(),1.Hz()),
        }
    }
}

#[allow(non_camel_case_types)]
pub struct State_Ranging<const DIM: usize> {    // DIM: 4|8
    // Access to 'VL53L5CX_Configuration'.
    // The 'Option' is needed to have both explicit '.stop()' and an implicit 'Drop'.
    outer_state: Option<State_HP_Idle>,
    rbuf: ResultsData<DIM>      // Rust-side results store
}

impl<'b: 'c,'c,const DIM: usize> State_Ranging<DIM> {
    pub(crate) fn transition_from(/*move*/ mut st: State_HP_Idle, cfg: &RangingConfig<DIM>) -> Result<Self> {
        let vl: &mut VL53L5CX_Configuration = st.borrow_uld_mut();
        cfg.apply(vl)?;

        match unsafe { vl53l5cx_start_ranging(vl) } {
            ST_OK => {
                let x = Self{
                    outer_state: Some(st),
                    rbuf: ResultsData::empty()
                };
                Ok(x)
            },
            e => Err(Error(e))
        }
    }

    /*
    * Used by the app-level, together with interrupts (and/or timer) and '.await', to see which
    * board(s) have fresh data.
    */
    pub fn is_ready(&mut self) -> Result<bool> {
        let mut tmp: u8 = 0;
        match unsafe { vl53l5cx_check_data_ready(self.borrow_uld_mut(), &mut tmp) } {
            ST_OK => Ok(tmp != 0),
            e => Err(Error(e))
        }
    }

    // tbd. consider adding 'time_stamp' as in the Flock
    /*
    * Collect results from the last successful scan.
    *
    //tbd. Try and describe what happens, if you call here before a scan is ready.  tbd. Make a test/example
    *
    * Note: The data is valid until the next '.get_data()' call. Rust reference management takes
    *       care that all reads to them are dropped, before a new round is replacing them. #rust
    */
    pub fn get_data(&mut self) -> Result<(&ResultsData<DIM>, TempC)> {
        use core::mem::MaybeUninit;
        use core::ptr::addr_of_mut;

        // The 'i8' field within the struct needs explicit initialization.
        // See -> https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-a-struct-field-by-field
        //
        let mut buf: VL53L5CX_ResultsData = {
            let mut un = MaybeUninit::<VL53L5CX_ResultsData>::uninit();
            let up = un.as_mut_ptr();
            unsafe {
                addr_of_mut!((*up).silicon_temp_degc).write(0);
                un.assume_init()
            }
        };
        //unsafe { MaybeUninit::zeroed().assume_init() }        // the easy way :)

        match unsafe { vl53l5cx_get_ranging_data(self.borrow_uld_mut(), &mut buf) } {
            ST_OK => {
                let temp_c = self.rbuf.feed(&buf);
                Ok((&self.rbuf, temp_c))
            },
            e => Err(Error(e))
        }
    }

    /*
    * Stop the ranging; provides access back to the 'HP Idle' state of the sensor.
    */
    pub fn stop(mut self) -> Result<State_HP_Idle> {
        match Self::_stop(self.outer_state.as_mut().unwrap()) {
            Ok(()) => {
                Ok( self.outer_state.take().unwrap() )  // leave 'None' for the 'Drop' handler
            },
            Err(e) => Err(e)
        }
    }

    /*
    * Lower level "stop", usable by both the explicit '.stop()' and 'Drop' handler.
    *
    * Takes '&mut Self': 'Drop' handler cannot call the normal '.stop()' that consumes the struct.
    */
    fn _stop(outer: &mut State_HP_Idle) -> Result<()> {
        match unsafe { vl53l5cx_stop_ranging(outer.borrow_uld_mut()) } {
            ST_OK => Ok(()),
            e => Err(Error(e))
        }
    }

    fn borrow_uld_mut(&mut self) -> &mut VL53L5CX_Configuration {
        self.outer_state.as_mut().unwrap().borrow_uld_mut()
    }
}

/*
* A Drop handler, so the ranging will seize (on the sensor) if the application simply drops the
* state (instead of turning it back to 'HP Idle').
*/
impl<const DIM: usize> Drop for State_Ranging<DIM> {
    fn drop(&mut self) {
        #[cfg(feature = "defmt")]
        trace!("Drop handler called");

        match self.outer_state {
            None => (),
            Some(ref mut outer) => {
                match Self::_stop(outer) {
                    Ok(_) => {},
                    Err(Error(e)) => { panic!("Stop ranging failed; st={}", e) }
                }
            }
        }
    }
}
