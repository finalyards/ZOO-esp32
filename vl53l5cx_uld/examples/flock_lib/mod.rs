/*
* tbd. THIS WILL LIKELY BECOME A CRATE OF ITS OWN, so the 'vl53l5cx_uld' is left for catering
*       to single-board interfacing only.
*
* Flock is when you tie multiple sensors together, to gain a larger matrix (and/or orient it in
* space).
*/
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
    RangingConfig,
    VL53L5CX_InAction,
    VL53L5CX
};

pub struct Flock<P: Platform + 'static, const N: usize> {
    vls: [VL53L5CX<P>;N],
    g: Gate<N>
}

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
    pub fn new_maybe(/*move*/ mut p: P, LPns: [Output<'static>;N]) -> Result<Self,N> {
        let mut g = Gate::new(LPns).init();

        let mut pez = Dispenser::new(p);

        // Create (and ping) each of the boards.
        //
        // Note: It's important we don't proceed to initialization (firmware upload) before having
        //      pinged all boards for existance (i.e. to check their basic wiring). Init takes ~3s
        //      per board; this shows mistakes early on (and it matches 'VL53L5CX' behaviour).
        //
        let tmp = g.with_each(|| {
            let pp = pez.dispense();
            VL53L5CX::new_maybe(pp)
        });
        let vls= join_results(tmp)?;

        Self { vls, g }
    }

    pub fn init(self) -> Result<Flock_InAction> {
        unimplemented!()
    }
}

#[allow(non_camel_case_types)]
pub struct Flock_InAction<const N: usize> {
    vls: [VL53L5CX_InAction;N],
    g: Gate<N>
}

impl<const N: usize> Flock_InAction<N>{
    /**
    * @brief Start 'FlockRanging', using all the sensors given.
    */
    pub fn start_ranging<const DIM: usize>(&mut self, cfg: &RangingConfig<DIM>) -> Result<FlockRanging<DIM>> {
        unimplemented!();
    }

    //---
    // Maintenance; special use
    //
    pub fn get_power_mode(&mut self) -> Result<PowerMode> {
        // Take the first sensor's power mode.
        unimplemented!();
    }
    pub fn set_power_mode(&mut self, v: PowerMode) -> Result<()> {
        unimplemented!();
    }
}


/*
* Wrap of the pins that are used for enabling just a single VL53L5CX, at a time.
*/
struct Gate<const N: usize>(
    [Output<'static>;N]
);

impl<const N: usize> Gate<N> {
    #[allow(non_snake_case)]
    fn new(LPns: [Output<'static>;N]) -> Self {
        Self(LPns)
    }

    fn init(mut self) -> Self {
        self.0.iter_mut().for_each(|p| p.set_low());  // disable all sensors
        self
    }

    /*
    * Do something on all the sensors.
    */
    fn with_each<F: Fn() -> Result<()>>(&mut self, f: F) {
        self.0.iter_mut().for_each(|p| {
            let x;
            p.set_high();
            {
                x = f();
            }
            p.set_low();
            let _= x;   // tbd. handling the return values?
            ()
        })
    }
}

type Result<X,const N: usize> = core::result::Result<X,FlockError<N>>;

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
