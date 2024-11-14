/*
* Ranging for a single VL53L5CX sensor.
*/
#![cfg(feature = "single")]

#[cfg(feature = "defmt")]
use defmt::{trace};

use esp_hal::{
    gpio::Input,
    time::{Instant, now}
};

use vl53l5cx_uld::{
    RangingConfig,
    Result,
    ResultsData,
    State_Ranging,
    units::TempC,
};

use crate::{
    VL,
};

#[derive(Clone, Debug)]
pub struct SoloResults<const DIM: usize>{
    pub res: ResultsData<DIM>,
    pub temp_degc: TempC,
    pub time_stamp: Instant,
}

/*
* Ranging for a single board.
*/
#[cfg(feature = "single")]
pub struct Ranging<const DIM: usize> {    // DIM: 4|8
    uld: State_Ranging<DIM>,
    pinINT: Input<'static>
}

#[cfg(feature = "single")]
impl<const DIM: usize> Ranging<DIM> {
    pub(crate) fn start(vl: VL, cfg: &RangingConfig<DIM>, pinINT: Input<'static>) -> Result<Ranging<DIM>> {
        let uld = vl.into_uld().start_ranging(cfg)?;
        Ok(Self{ uld, pinINT })
    }

    pub async fn get_data(&mut self) -> Result<SoloResults<DIM>> {
        let t0 = now();

        // Two kinds of spec can be implemented here:
        //  - wait for the NEXT INT edge, then provide results
        //  - provide results; if none, wait for the INT edge
        //
        // 1st:
        //      Always provides fresh results.
        //      Can miss a result, if it's gotten ready faster than the app moved from 'start()'
        //      to here (unlikely).
        //
        // 2nd:
        //      Would provide more results (theoretically), but some may be stale
        //
        // Since we time the results, 1st feels more.. better choice. For now, at least.
        //
        self.pinINT.wait_for_falling_edge() .await;
        let ts = now();     // nearest time after the (presumed) scan

        trace!("Received falling edge of INT, after {}", now() - t0);

        match self.uld.is_ready()? {
            true => (),
            false => panic!("INT edge seen but sensor has no data"),
        }

        let (res, temp_degc) = self.uld.get_data()?;
        Ok( SoloResults{ res, temp_degc, time_stamp: ts } )
    }

    pub fn stop(self) -> Result<VL> {
        let uld = self.uld.stop()?;
        Ok(VL::recreate(uld))
    }
}
