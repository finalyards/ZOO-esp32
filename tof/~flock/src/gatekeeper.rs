
use esp_hal::{
    gpio::Output
};

use vl53l5cx_uld as uld;

/*
* Helper that wraps either 'uld::VL54L5CX' (initially), 'uld::VL54L5CX_InAction', or
* 'uld::ranging::Ranging'. Provides access to them only with a certain 'LPn' (chip select) pin
* raised high, to select one of multiple boards.
*/
pub(crate) struct GateKeeper<V, const N: usize> {
    // tbd. T to V; separate 'vs: [V;N]' ja 'LPns: [Output<'static>;N]' (helpompi alla)
    //pairs: [(T, Output<'static>);N]
}

impl<T,const N: usize> GateKeeper<T,N> {

    // The only method we need is '.map' that converts the 'Flock' between states '<Uninitialized>',
    // 'Idle', and 'Scanning' (perhaps even 'PowerOff' could be a separate state?).
    //
    // Ownership of the 'LPn' pins travels in these state transitions.
    //
    // Note: In the case of an error, you won't be able to recover, since the 'self' has been
    //      consumed by this method.
    //
    pub(crate) fn map<F,X>(mut self, f: F) -> Result<GateKeeper<X,N>>
        where F: Fn(T) -> uld::Result<X>
    {
        let xs = self.pairs.iter_mut().map(|(v,LPn)| {
            let x;
            LPn.set_high();
            {
                x = f(v);
            }
            LPn.set_low();
            x
        });
        join_results(xs)
    }

    /*** R LATER
    /*
    * A version that maps with a '&mut'
    */
    pub(crate) fn with_each_ref<F,X>(&mut self, f: F) -> Result<X>
        where F: Fn(&mut T) -> uld::Result<X>
    {
        let xs = self.pairs.iter_mut().map(|(&mut v,LPn)| {
            let x;
            LPn.set_high();
            {
                x = f(v);
            }
            LPn.set_low();
            x
        });
        join_results(xs)
    }

    /*
    * As above, but mix in the contents of another array.
    */
    pub(crate) fn with_each_zip<F,X,Y>(&mut self, ys: &[Y;N], f: F) -> crate::Result<X>
        where F: Fn((Y,T)) -> uld::Result<X>
    {
        let xs = self.pairs.iter_mut().zip(ys).map(|((v,LPn),y)| {
            let x;
            LPn.set_high();
            {
                x = f((y,v));
            }
            LPn.set_low();
            x
        });
        join_results(xs)
    }
    ***/
}

/*
* Turn 'N' ULD level results into a single flock result.
*
* i.e. if any of the results is an error, any such (actually, the first) is reported onwards.
*       Successful values from other boards are lost.
*/
fn join_results<X, const N: usize>(rs: [uld::Result<X>;N]) -> crate::Result<[X;N]> {

    // Rust note: By checking first, whether there is an error or not, we allow 'rs' to be moved
    //      individually within the branches (simplifies coding).
    //
    if rs.iter().any(|r| r.is_err()) {
        let (i,e) = rs.iter().enumerate().find(|(_,r)| r.is_err()).unwrap();
        Err(crate::Error{ sensor_id: i, e })
    } else {
        let xs: [X;N] = rs.map(|r| r.unwrap());
        Ok(xs)
    }
}
