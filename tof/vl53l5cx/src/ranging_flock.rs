/*
* Scanning multiple VL53L5CX sensors for the next result.
*/
#![cfg(feature = "flock")]

//#[cfg(feature = "defmt")]
//use defmt::{debug, trace};

use esp_hal::{
    gpio::Input,
};

use vl53l5cx_uld::{
    units::TempC,
    RangingConfig,
    Result,
    ResultsData,
    State_Ranging,
};

use crate::{
    VL,
    z_array_try_map::turn_to_something
};

/*
* State for scanning multiple VL53L5CX boards.
*/
pub struct RangingFlock<const N: usize, const DIM: usize> {
    ulds: [State_Ranging<DIM>;N],
    pinINT: Input<'static>,
    last_scanned: usize,     // 0..N-1 and round
}

impl<const N: usize, const DIM: usize> RangingFlock<N,DIM> {

    pub(crate) fn start(vls: [VL;N], cfg: &RangingConfig<DIM>, pinINT: Input<'static>) -> Result<Self> {

        // Turn the ULD level handles into "ranging" state, and start tracking the 'pinINT'.

        let ulds: [State_Ranging<DIM>;N] = turn_to_something(vls, |x| x.into_uld().start_ranging(cfg))?;

        Ok(Self{
            ulds,
            pinINT,
            last_scanned: 0     // initial value doesn't really matter
        })
    }

    /*
    * Get the next available results.
    */
    pub async fn get_data(&mut self) -> (ResultsData<DIM>,TempC) {

        unimplemented!()
    }

    fn stop(self) -> Result<([VL;N], Input<'static>)> {
        let vls = turn_to_something(self.ulds, |x| {
            let uld = x.stop()?;
            Ok( VL::recreate(uld) )
        })?;

        Ok( (vls, self.pinINT) )
    }
}
