/*
* Scanning multiple VL53L5CX sensors for the next result.
*/
#![cfg(feature = "flock")]

#[cfg(feature = "defmt")]
use defmt::{debug,trace};

use esp_hal::{
    gpio::Input,
    time::{now, Instant}
};

use vl_uld::{
    units::TempC,
    RangingConfig,
    Result,
    ResultsData,
    State_Ranging,
};

use arrayvec::ArrayVec;

use crate::{
    VL53
};

#[derive(Clone, Debug)]
pub struct FlockResults<const DIM: usize>{
    pub board_index: usize,
    pub res: ResultsData<DIM>,
    pub temp_degc: TempC,
    pub time_stamp: Instant,
}

/*
* State for scanning multiple VL53L5CX boards.
*
* Note: A generator would be ideal for this (could keep the state within it).
*/
pub struct RangingFlock<const N: usize, const DIM: usize> {
    ulds: [State_Ranging<DIM>;N],
    pinINT: Input<'static>,
    pending: ArrayVec<FlockResults<DIM>,N>    // tbd. pick suitable capacity once we know the behaviour
}

impl<const N: usize, const DIM: usize> RangingFlock<N,DIM> {

    pub(crate) fn start(vls: [VL53;N], cfg: &RangingConfig<DIM>, pinINT: Input<'static>) -> Result<Self> {

        // Turn the ULD level handles into "ranging" state, and start tracking the 'pinINT'.

        let ulds: [State_Ranging<DIM>;N] = array_try_map(vls, |x| x.into_uld().start_ranging(cfg))?;

        Ok(Self{
            ulds,
            pinINT,
            pending: ArrayVec::new()
        })
    }

    /*
    * Get the next available results.
    *
    * By design, we provide just one result at a time. This is akin to streaming/generation, and
    *       makes it easier for the recipient, compared to getting 1..N results, at once.
    */
    pub async fn get_data(&mut self) -> Result<FlockResults<DIM>> {

        // Time stamp the results as fast after knowing they exist, as possible.

        // 1. Anything in the 'pending'? Give them first.
        // 2. Check for new results
        // 3. If nothing, wait for the next INT lowering edge
        // 4. Also read new results on rising edge of INT: since we share the INT signal across
        //    boards, new entries may have turned up, in the mean time.
        //
        // Note: All of this logic is experimental. Let's trace what happens in reality, and
        //      adjust!
        //      - some measurements may get lost, but the number should be minimal
        //      - if two measurements from the same board, the older one shall never replace the newer one
        //        (they can be both delivered)
        //      - time stamps should be as close to actual measurement as possible!

        // Trace if we see new data
        //#[cfg(false)]
        {
            for (i,uld) in self.ulds.iter_mut().enumerate() {
                if uld.is_ready()? {
                    trace!("Data available on entry: {}", i);
                }
            }
        }

        loop {
            // Add new results to the 'self.pending'.
            for (i,uld) in self.ulds.iter_mut().enumerate() /*.rev()*/ {
                if uld.is_ready()? {
                    let time_stamp = now();
                    let (res,temp_degc) = uld.get_data()?;
                    let o = FlockResults{ board_index: i, res, temp_degc, time_stamp };

                    debug!("New data from #{}, pending becomes {}", i, self.pending.len()+1);
                    self.pending.push(o);
                } else {
                    debug!("No new data from #{}", i);
                }
            }

            // Return already pending results, one at a time.
            if let Some(tmp) = self.pending.pop() {
                return Ok(tmp);
            }

            // No data; sleep until either edge
            //
            // Falling edge: VM has gotten new data
            // Rising edge: since we use same INT for all sensors, it might make sense to check
            //      this edge as well. If we are fast enough to fall in sleep before the INT-low
            //      ends (100us from the last new result), it's possible there's yet more data we
            //      didn't hear of. Checking both edges ensures we get even those, with sub-ms delay.
            //
            assert!(self.pending.is_empty());
            {
                trace!("Going to sleep (INT {}).", if self.pinINT.is_low() {"still low"} else {"high"});

                let t0 = now();
                self.pinINT.wait_for_any_edge().await;

                debug!("Woke up to INT edge (now {}; slept {}ms)", if self.pinINT.is_low() {"low"} else {"high"}, (now() - t0).to_millis());
            }
        }
    }

    pub fn stop(self) -> Result<([VL53;N], Input<'static>)> {
        let vls = array_try_map(self.ulds, |x| {
            let uld = x.stop()?;
            Ok( VL53::recreate(uld) )
        })?;

        Ok( (vls, self.pinINT) )
    }
}

// 'ArrayVec' allows mapping one '[;N]' to another. Plain Rust (1.82, stable) doesn't.
// 'array_try_map' does this without 'ArrayVec', but is unstable.
//
pub(crate) fn array_try_map<A,B, const N: usize>(aa: [A;N], f: impl FnMut(A) -> Result<B>) -> Result<[B;N]> {
    use arrayvec::ArrayVec;
    let bs_av = aa.into_iter().map(f).collect::<Result<ArrayVec<B,N>>>() ?;

    Ok(bs_av.into_inner().ok().unwrap())
}
