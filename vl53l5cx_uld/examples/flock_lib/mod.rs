/*
* tbd. THIS WILL LIKELY BECOME A CRATE OF ITS OWN, so the 'vl53l5cx_uld' is left for catering
*       to single-board interfacing only.
*
* Flock is when you tie multiple sensors together, to gain a larger matrix (and/or orient it in
* space).
*/
#[allow(unused_imports)]
#[cfg(feature = "defmt")]
use defmt::{debug, error};

use esp_hal::{
    gpio::Output
};

mod flock_platform;
use flock_platform::{
    Dispenser
};
mod flock_ranging;
use flock_ranging::{
    FlockRanging
};

extern crate vl53l5cx_uld as uld;
use uld::{
    Platform,
    PowerMode,
    RangingConfig,
    VL53L5CX_InAction,
    VL53L5CX
};

type Result<X> = core::result::Result<X,FlockError>;

#[derive(core::fmt::Debug)]
struct FlockError{ sensor_id: u8, e: u8 }

/*
* Helper that wraps both 'uld::VL54L5CX' (initially) and 'uld::VL54L5CX_InAction' (later). We
* just pair those with the gate-keeper 'LPn' signals for each sensor.
*/
struct GatedPairs<T, const N: usize> {
    pairs: [(T, Output<'static>);N]
}

impl<T,X, const N: usize> GatedPairs<T,N> {
    /*
    * Run given function on all the 'uld' instances; raise each board's 'LPn' gate before (lower after),
    * to select them.
    *
    * If there is an error, can either cancel immediately or gather all the possible results (in either
    * case, gathered results will be lost).
    */
    fn with_each<F>(&mut self, f: F) -> Result<X>
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

pub type Flock<P: Platform + 'static, const N: usize> = GatedPairs<VL53L5CX<P>,N>;

pub type FlockInAction<const N: usize> = GatedPairs<VL53L5CX_InAction,N>;

impl<P: Platform + 'static, const N: usize> Flock<P,N> {
    /*
    * Provided a 'Platform' (for accessing I2C) and a number of chip-select pins (for picking the
    * particular sensor):
    *
    *   - ping all the sensors for proofing their existence (early fail)
    *
    * This (as other 'Flock' methods) mirrors the behaviour of 'uld::VL53L5CX::new_maybe', but
    * extends them to all the sensors.
    */
    #[allow(non_snake_case)]
    pub fn new_maybe(/*move*/ mut p: P, LPns: [Output<'static>;N]) -> Result<Self> {
        //R let mut g = Gate::new(LPns).init();

        let mut pez = Dispenser::new(p);
        let dispense = || pez.dispense();

        // Create (and ping) each of the boards.
        //
        // Note: It's important we don't proceed to initialization (firmware upload) before having
        //      pinged all boards for existence (i.e. to check their basic wiring). This approach
        //      shows (wiring?) mistakes early on - and it matches 'VL53L5CX::new_maybe()' behaviour.
        //

        // Rust note: If there was '.try_map()' (similar to '.try_for_each' but collecting the success values),
        //      this would be doable in one loop.
        //
        let tmp = LPns.iter().enumerate().map( move |(i,LPn)| {
            let pp = dispense();
            VL53L5CX::new_maybe(pp)
                .map( |vl| (vl,LPn) )
        });

        if tmp.iter().any(|x| x.is_err()) {
            let (i,Err(e)) = tmp.enumerate().find(|(i,x)| x.is_err());
            Err(FlockError{sensor_id: i as u8,e})
        } else {
            let pairs = tmp.map(|x| x.unwrap());
            Ok( Self{ pairs } )
        }
    }

    pub fn init(mut self) -> Result<Flock_InAction<N>> {
        let mut buf: [Option<(VL53L5CX_InAction,Output<'static>)>;N] = [None;N];

        let tmp = self.with_each(|vl| {
            vl.init()
        })?;

        for (i,((_,LPn),x)) in self.pairs.iter().zip(tmp).enumerate() {
            buf[i] = Some((x,LPn))
        }

        buf.map( Option::unwrap )
    }
}

#[allow(non_camel_case_types)]
pub struct Flock_InAction<const N: usize> {
    pairs: [(VL53L5CX_InAction,Output<'static>);N],
}

impl<const N: usize> Flock_InAction<N>{
    /**
    * @brief Start 'FlockRanging', using all the sensors given.
    */
    pub fn start_ranging<const DIM: usize>(&mut self, cfg: &RangingConfig<DIM>) -> Result<FlockRanging<DIM,N>> {
        self.with_each(|vl| {
            vl.start_ranging(cfg)
        })
    }

    //---
    // Maintenance; special use
    //
    pub fn get_power_mode(&mut self) -> Result<PowerMode> {
        // It's just easier to read them all; though reading just the first would use less bus time.
        let res= self.g.with_each(|vl| {
            vl.get_power_mode()
        });
        res.map(|modes| {
            assert!(modes[1..].all(modes[0]));
            modes[0]
        })
    }
    pub fn set_power_mode(&mut self, v: PowerMode) -> Result<()> {
        self.with_each(|vl| {
            vl.set_power_mode(v)
        })
    }
}


/*
* Turn 'N' ULD level results into a single Flock 'Result'.
*/
fn join_results<'a, X, const N: usize>(rs: [uld::Result<X>;N]) -> Result<[X;N]> {

    // Rust note: By checking first, whether there is an error or not, we allow 'rs' to be moved
    //      individually, within the branches (simplifies coding).
    //
    if rs.iter().any(|r| r.is_err()) {
        let (i,e) = rs.enumerate().find(|(_,r)| r.is_err());
        FlockError{ sensor_id: i, e }
    } else {
        // all values are good
        let xs: [X;N] = rs.map( Result::unwrap );
        Ok(xs)
    }
}

/*** disabled; KEEP in case we want to report all failed accesses, not just first??
/*
* Turn 'N' ULD level errors into a 'FlockError', if _any_ of them is a fail.
*/
fn join_results<'a, X, const N: usize>(rs: [uld::Result<X>;N]) -> Result<[X;N],N> {

    // Rust note: By checking first, whether there is an error or not, we allow 'rs' to be moved
    //      individually, within the branches (simplifies coding).
    //
    if rs.iter().any(|r| r.is_err()) {
        let rcs = rs.map(|r| match r { Err(rc) => rc, Ok(_) => 0 });
        Err(FlockError::new(rcs))
    } else {
        // Collect the known-good values, and return.
        let xs = rs.map(|r| r.unwrap() );
        Ok(xs)
    }
}

#[derive(core::fmt::Debug)]
pub struct FlockError<const N: usize> {     // tbd. report which sensor(s) misbehaved
    rcs: [u8;N],
}

impl<const N: usize> FlockError<N> {
    fn new(rcs: [u8;N]) -> Self {
        Self{rcs}
    }
    fn failed_sensors<I: Iterator>(&self) -> I {
        self.rcs.iter().enumerate()
            .filter(|i,rc| rc != 0)
            .map(|i,_| i)
    }
}

impl<const N: usize> core::fmt::Display for FlockError<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error on sensor(s): {}", self.failed_sensors())
    }
}
***/
