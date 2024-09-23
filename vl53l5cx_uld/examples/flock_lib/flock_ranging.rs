/*
*/
use esp_hal::{
    time::now
};

use uld::{
    TempC
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct TimeStamp(u64);

impl TimeStamp {
    fn now() -> Self {
        let t = now().duration_since_epoch().to_millis();
        Self(t)
    }
}

//#[cfg_attr(feature = "defmt", derive(defmt::Format))]
//struct SensorIndex(u8);

pub struct FlockRanging<'a, const DIM: usize, const N: usize> {
    rings: [uld::Ranging<'a, DIM>;N]
}

impl<const DIM: usize, const N: usize> FlockRanging<DIM,N> {

    /*
    * Return data from all the sensors. 'None' if a sensor's data is not available.
    *
    * If there is a single 'uld' side error, that error is returned and any data already
    * fetched will be lost. (Should be rare)
    */
    pub fn get_data(&self) -> uld::Result<[Option<(&uld::ResultsData<DIM>, TempC, u64)>]> {

        // tbd. see how much time difference there is, asking '.is_ready()' from the devices.
        //      Likely getting just one time stamp for all of them is fine?  (see time stamp before
        //      the asking, during the loop, after the asking).

        // Cycle through the sensors and see, which have data available. Doing this separate
        // from fetching the data aims to have a more unique timestamp for the data that was
        // fetched (I2C transfer of data itself does not cause major skew). NOTE: THIS HAS NOT BEEN
        // PROVEN, JUST WISHFUL THINKING.
        //
        let mask: [bool;N] = self.with_each_sensor(|vls| {
            vls.is_ready()
        })?;

        let ts = now().duration_since_epoch().to_micros();

        let tmp: [Option<_>;N] = self.with_sensors_enumerated(|(i,vls)| {
            if mask[i] {
                let (data, temp_c) = vls.get_data()?;
                Some((data, temp_c, ts))
            } else {
                None
            }
        });
        tmp
    }
}
