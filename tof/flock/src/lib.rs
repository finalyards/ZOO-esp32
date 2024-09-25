#![no_std]

mod gatekeeper;
mod platform;
mod ranging;

#[allow(unused_imports)]
#[cfg(feature = "defmt")]
use defmt::{debug, error};

use esp_hal::{
    gpio::Output
};

use platform::{
    Dispenser
};
use ranging::{
    FlockRanging
};
pub(crate) use gatekeeper::GateKeeper;

use vl53l5cx_uld::{
    Platform,
    PowerMode,
    RangingConfig,
    VL53L5CX_InAction,
    VL53L5CX
};

type Result<X> = core::result::Result<X,self::Error>;

#[derive(core::fmt::Debug)]
struct Error{ sensor_id: u8, e: u8 }

pub type Flock<P: Platform + 'static, const N: usize> = GateKeeper<VL53L5CX<P>,N>;

pub type FlockInAction<const N: usize> = GateKeeper<VL53L5CX_InAction,N>;

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
        let mut shared_p = RefCell::new(p);

        // We start by having an "empty" GateKeeper, but turn the 'LPns' into its ownership already
        // here. From this on, things proceed by mapping.
        //
        let g: GateKeeper<(),N> = GateKeeper::new([();N], LPns);

        // Create (and ping) each of the boards.
        //
        // Note: It's important we don't proceed to initialization (firmware upload) before having
        //      pinged all boards for existence (i.e. to check their basic wiring). This approach
        //      shows (wiring?) mistakes early on - and it matches 'VL53L5CX::new_maybe()' behaviour.
        //
        g.map(|_| {
            let pp = shared_p.clone();     // get a new ULD-level 'Platform'
            VL53L5CX::new_maybe(pp)
        })
    }

    pub fn init(mut self) -> Result<FlockInAction<N>> {
        self.map(|vl| {
            vl.init()
        })
    }
}

impl<const N: usize> FlockInAction<N>{
    /**
    * @brief Start 'FlockRanging', using all the sensors given.
    *
    * This will start the sensors "doing their thing". The caller can either observe an interrupt
    * line, or busy-wait until some data becomes available. Note that the caller retains their
    * ownership to the 'LPn' signals.
    */
    pub fn start_ranging<const DIM: usize>(&mut self, cfg: &RangingConfig<DIM>) -> Result<ranging::FlockRanging<DIM,N>> {
        FlockRanging::new(cfg, &self)
    }

    //---
    // Maintenance; special use
    //
    pub fn get_power_mode(&mut self) -> Result<PowerMode> {
        // It's just easier to read them all; though reading just the first would use less bus time.
        let res= self.g.with_each(|&mut vl| {
            vl.get_power_mode()
        });
        res.map(|modes| {
            assert!(modes[1..].all(modes[0]));
            modes[0]
        })
    }
    pub fn set_power_mode(&mut self, v: PowerMode) -> Result<()> {
        self.with_each(|&mut vl| {
            vl.set_power_mode(v)
        })
    }
}
