use vl53l5cx_uld::{
    Ranging,
    RangingConfig,
    //Result as UldResult,
    ResultsData,
    TempC
};
use crate::VL;

/*
* An ongoing ranging operation, on one or multiple VL boards.
*/
pub struct RingN<'a, const N: usize, const DIM: usize> {
    rn: [Ranging<'a, DIM>;N]
}

impl<const N: usize, const DIM: usize> RingN<'_, N,DIM> {
    /*
    * Start a ranging (measurement) operation.
    *
    * Use directly for ranging on multiple boards. For just a single one, wrap via 'VL::start_ranging'.
    */
    pub fn start_many(VLs: &[&mut VL;N], cfg: &RangingConfig<DIM>) -> Self {

        let rn: [Ranging<DIM>;N] = VLs.map(|VL| {
            let uld = VL.borrow_uld_mut();
            uld.start_ranging(cfg)
                .unwrap()
        });

        Self { rn }
    }

    /*
    * Wait for an 'INT' to happen (signifies that *some* VL now has results ready); then read and
    * return any fresh results.
    *
    * Notes:
    *   - VL chips keep the 'INT' low for 100us, after which they automatically let it float, again.
    *   - we intend a single wire to be shared by all the 'INT' outputs; this means:
    *       - single 'INT' low may exceed 100us, if another interrupt occurs during the 100us
    *   - Reading results from a chip takes ~4ms (in the magnitude of milliseconds); this means,
    *       by the time we've read the first board, the original INT is long gone, and others may
    *       have come (and gone). This is NOT A PROBLEM - just setting your expectations.
    */
    pub async fn get_data(&mut self) -> (ResultsData<DIM>,TempC) {

        unimplemented!()
    }
}

impl<const DIM: usize> RingN<'_, 1,DIM> {
    pub fn start_one(vl: &mut VL) -> Self {
        unimplemented!()
    }

    pub async fn get_single_data(&mut self) -> (ResultsData<DIM>,TempC) {
        unimplemented!()
    }
}
