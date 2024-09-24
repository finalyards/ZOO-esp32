/*
*/
use core::cell::RefCell;

use esp_hal::{
    time::now
};

use vl53l5cx_uld::{
    ranging::RangingConfig,
    TempC,
    VL53L5CX_InAction
};

use crate::GateKeeper;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct TimeStamp(u64);

impl TimeStamp {
    fn now() -> Self {
        let t = now().duration_since_epoch().to_millis();
        Self(t)
    }
}

pub struct FlockRanging<'a, const DIM: usize, const N: usize> {
    rings: [uld::Ranging<'a, DIM>;N],
    g: &'a RefCell<GateKeeper<VL53L5CX_InAction>>
}

impl<'a, const DIM: usize, const N: usize> FlockRanging<'_, DIM,N> {
    fn new(cfg: &RangingConfig<DIM>, g: &'a RefCell<GateKeeper<VL53L5CX_InAction>>) -> Self {

        let rings: [_;N] = g.get_mut().with_each(|vl| {
            vl.start_ranging(cfg)
        });
        Self{ rings, g }
    }

    /*
    * Return data from all the sensors that currently have any. If none, the caller should wait.
    *
    * The data are time-stamped as well as we can, allowing the application to interpret them
    * relative to some absolute wall clock time.
    *
    * Note: The measurements mean something that has happened *before* the time stamp (depending
    *       on the integration time configuration etc.), i.e. interpret these as _spans_ in time
    *       domain, not instantaneous incidents.
    *
    * If there is a ULD side error (e.g. with transmission), that error is returned and any data
    * already fetched will be lost. (Should be rare; okay)
    */
    pub fn get_data(&self) -> uld::Result<&[Option<(&uld::ResultsData<DIM>, TempC, u64)>;N]> {

        // tbd. see how much time difference there is, asking '.is_ready()' from the devices.
        //      Likely getting just one time stamp for all of them is fine?  (see time stamp before
        //      the asking, during the loop, after the asking).

        // Cycle through the sensors and see, which have data available. Doing this separate
        // from fetching the data aims to have a more unique timestamp for the data that was
        // fetched (I2C transfer of data itself does not cause major skew). NOTE: THIS HAS NOT BEEN
        // PROVEN, JUST WISHFUL THINKING.
        //
        // Closest time stamps _after_ measurement of each result.
        //
        let tss: [Option<u64>;N] = self.g.get_mut().with_each(|vl| {
            if vl.is_ready() {
                Some( now().duration_since_epoch().to_micros() )
            } else {
                None
            }
        })?;

        let tmp: [Option<_>;N] = self.g.get_mut().with_each_zip(tss, |(vl,ts_maybe)| {
            // For those sensors we know to have data, fetch it. 'uld::Error' return values are
            // collected and converted by 'with_each_zip'.
            //
            match ts_maybe {
                Some(ts) => {
                    vls.get_data()
                        .map(|t| (t.0,t.1,ts))  // (data, silicon temperature, time stamp)
                },
                None => Ok(None)
            }
        });
        tmp
    }
}
