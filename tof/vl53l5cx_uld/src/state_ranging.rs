/*
* state_ranging.rs:
*
*   'RangingConfig':  how the ranging should be done
*   'State_Ranging':  handle to the sensor once ranging is ongoing
*/
#![allow(for_loops_over_fallibles)]     // tbd. how to make this project-wide?  // just say NO! to 'if let' :)

#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{assert, panic, trace, debug};

use crate::uld_raw::{
    VL53L5CX_Configuration,
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
    RangingMode as RangingMode_R,
    Resolution as Resolution_R,
    ST_OK,
    TargetOrder as TargetOrder_R,
    VL53L5CX_ResultsData
};

use crate::{
    results_data::ResultsData,
    state_hp_idle::State_HP_Idle,
    units::{MsU16, HzU8, PrcU8, TempC, ExtU32 as _},
    Error,
    Result,
};

use {
    Mode::{CONTINUOUS, AUTONOMOUS},
    TargetOrder::{CLOSEST, STRONGEST},
};

/*
* Defines which resolutions the device is able to play with; their mapping to ULD and physical
* limits.
*/
// Rust note: 'const fn' gets evaluated at compile time.
const fn reso_details<const DIM: usize>() -> (Resolution_R /*Raw entry*/, u8 /*integration timeS (N)*/, HzU8 /*max freq*/) {
    match DIM {
        4 => { (Resolution_R::_4X4, 1, HzU8(60)) },
        8 => { (Resolution_R::_8X8, 4, HzU8(15)) },
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

impl Mode {
    fn as_uld(&self) -> RangingMode_R {
        match self {
            CONTINUOUS => RangingMode_R::CONTINUOUS,
            AUTONOMOUS(_,_) => RangingMode_R::AUTONOMOUS
        }
    }
}

// 'TargetMode' is 1:1 with ULD API, but we avoid exposing the enum values.
//
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetOrder {
    CLOSEST,
    STRONGEST
}

impl TargetOrder {
    fn as_uld(&self) -> TargetOrder_R {
        match self {
            CLOSEST => TargetOrder_R::CLOSEST,
            STRONGEST => TargetOrder_R::STRONGEST
        }
    }
}

/*
* A setup for each separate 'Ranging' session. This encloses the resolution as a type, and also
* helps ensure that the C ULD API functions get called in a specific order (some vendor docs
* recommend certain orders.. anyways, it makes things more predictable). Other demands, according
* to vendor docs are:
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
pub struct RangingConfig<const DIM: usize> {    // |*|
    mode: Mode,      // also carries ranging frequency and integration time for 'AUTONOMOUS'
    sharpener: Option<PrcU8>,       // value range: 1..=99
    target_order: TargetOrder,
}
    // |*|: decided to /not/ provide a '= 4' default for the 'DIM'. It *would work*, and slightly
    //      make it easier for the _application layer_, but it also messes with compile errors,
    //      because now you need to remember to always add '<DIM>' to use of 'RangingConfig' -
    //      if you don't, you'll get both defaulting-to-4 and generic, which don't match.
    //
    //      tbd. Consider making an application-level 'RangingConfig' that DOES provide such
    //          a default, otherwise aliasing to us.

impl<const DIM: usize> RangingConfig<DIM> {
    /* We allow construction to make potentially incompatible combinations, but check them within
    * '.apply()'. This is a compromise between simplicity and robustness. Note that some obvious
    * type-system robustness has been done, e.g. bundling ranging frequency and integration times
    * with the ranging mode (since those only apply to one mode).
    */
    pub fn with_sharpener(/*move*/ self, v: PrcU8) -> Self {
        Self { sharpener: Some(v), ..self }
    }

    pub fn with_target_order(/*move*/ self, order: TargetOrder) -> Self {
        Self { target_order: order, ..self }
    }

    pub fn with_mode(/*move*/ self, mode: Mode) -> Self {
        Self { mode, ..self }
    }

    fn validate(&self) {
        let (_,R_INTEGRATION_TIMES_N, R_FREQ_RANGE_MAX): (_,u8,HzU8) = reso_details::<DIM>();

        match self.mode {
            AUTONOMOUS(MsU16(integration_time_ms), HzU8(freq)) => {
                assert!((2..=1000).contains(&integration_time_ms), "Integration time out of range");
                    // Source: comment in (and code of) vendor ULD C sources

                // "The sum of all integration times + 1 ms overhead must be lower than the measurement
                // period. Otherwise, the ranging period is automatically increased." (src: UM2884 - Rev 5 p.9)
                //
                // "4x4 is composed of ONE integration time"
                // "8x8 is composed of FOUR integration times" (same src)
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

        match self.sharpener {
            Some(PrcU8(v)) => { assert!((1..=99).contains(&v), "Sharpener out of range (1..=99)") },
            None => {}
        }
    }

    fn apply(&self, vl: &mut VL53L5CX_Configuration) -> Result<()> {
        self.validate();    // may panic
        let /*const*/ ULD_RESO: Resolution_R = reso_details::<DIM>().0;

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

        let sharpener_prc: u8 = match self.sharpener {
            Some(PrcU8(v)) => v,
            None => 0
        };
        match unsafe { vl53l5cx_set_sharpener_percent(vl, sharpener_prc) } {
            ST_OK => Ok(()),
            e => Err(Error(e))
        }?;

        match unsafe { vl53l5cx_set_target_order(vl, self.target_order.as_uld() as _) } {
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
            sharpener: None,
            target_order: STRONGEST,
            mode: AUTONOMOUS(5.ms(),HzU8(1)/*1.Hz()*/),
        }
    }
}

#[allow(non_camel_case_types)]
pub struct State_Ranging<const DIM: usize> {    // DIM: 4|8
    // Access to 'VL53L5CX_Configuration'.
    // The 'Option' is needed to have both explicit '.stop()' and an implicit 'Drop'.
    outer_state: Option<State_HP_Idle>,
}

impl<const DIM: usize> State_Ranging<DIM> {
    pub(crate) fn transition_from(/*move*/ mut st: State_HP_Idle, cfg: &RangingConfig<DIM>) -> Result<Self> {
        let vl: &mut VL53L5CX_Configuration = st.borrow_uld_mut();
        cfg.apply(vl)?;

        match unsafe { vl53l5cx_start_ranging(vl) } {
            ST_OK => {
                let x = Self{
                    outer_state: Some(st),
                };
                Ok(x)
            },
            e => Err(Error(e))
        }
    }

    /*
    * Used by the app-level, to see which of multiple boards (potentially all sharing an 'INT' line)
    * have data available.
    *
    * ASSUMPTION: When getting such data, the 'is_ready()' is no longer giving 'true'.
    */
    pub fn is_ready(&mut self) -> Result<bool> {
        let mut tmp: u8 = 0;
        match unsafe { vl53l5cx_check_data_ready(self.borrow_uld_mut(), &mut tmp) } {
            ST_OK => Ok(tmp != 0),
            e => Err(Error(e))
        }
    }

    /*
    * Collect results from the last successful scan.
    //tbd. Try and describe what happens, if you call here before a scan is ready.
    */
    pub fn get_data(&mut self) -> Result<(ResultsData<DIM>, TempC)> {
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

        match unsafe { vl53l5cx_get_ranging_data(self.borrow_uld_mut(), &mut buf) } {
            ST_OK => {
                let tuple = ResultsData::<DIM>::from(&buf);
                Ok(tuple)
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

        for mut outer in self.outer_state.as_mut() {
            match Self::_stop(&mut outer) {
                Ok(_) => {},
                Err(Error(e)) => { panic!("Stop ranging failed; st={}", e) }
            }
        }
    }
}
