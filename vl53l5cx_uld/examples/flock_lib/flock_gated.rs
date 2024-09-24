

/*
* Helper that wraps both 'uld::VL54L5CX' (initially) and 'uld::VL54L5CX_InAction' (later). We
* just pair those with the gate-keeper 'LPn' signals for each sensor.
*/
struct GatedPairs<T, const N: usize> {
    pairs: [(T, Output<'static>);N]
}

impl<const N: usize> GatedPairs<N> {
    /*
    * Run given function on all the 'uld' instances; raise each board's 'LPn' gate before (lower after),
    * to select them.
    *
    * If there is an error, can either cancel immediately or gather all the possible results (in either
    * case, gathered results will be lost).
    */
    fn with_each<F,X>(&mut self, f: F) -> Result<X>
        where F: Fn(T) -> uld::Result<X>
    {
        let xs = self.pairs.iter_mut().map(|(v,LPn)| {
            let x;
            LPn.set_high();
            {
                x = f();
            }
            LPn.set_low();
            x
        });
        join_results(xs.as_slice())
    }
}
