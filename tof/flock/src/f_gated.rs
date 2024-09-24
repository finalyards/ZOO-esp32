

/*
* Helper that wraps either 'uld::VL54L5CX' (initially) and 'uld::VL54L5CX_InAction' (later),
* providing access to them only with a certain 'LPn' (chip select) pin raised high, to select
* one out of multiple boards with the same I2C address (0x52) on the bus.
*/
struct GateKeeper<T, const N: usize> {
    pairs: [(T, Output<'static>);N]
}

impl<const N: usize> GatedKeeper<N> {
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
                x = f(v);
            }
            LPn.set_low();
            x
        });
        join_results(xs.as_slice())
    }

    /*
    * As above, but mix in the contents of another array.
    */
    fn with_each_zip<F,X,Y>(&mut self, ys: &[Y;N], f: F) -> Result<X>
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
        join_results(xs.as_slice())
    }
}
