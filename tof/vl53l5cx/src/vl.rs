/*
* Presents one VL53L5CX sensor, with its activation line and unique I2C address.
*/
#[cfg(feature = "defmt")]
use defmt::debug;

use core::cell::RefCell;

use esp_hal::{
    gpio::Input,
    i2c::{I2c, Instance},
    Blocking
};

use vl53l5cx_uld::{
    DEFAULT_I2C_ADDR_8BIT,
    RangingConfig,
    Result,
    State_HP_Idle,
    VL53L5CX
};

use crate::{
    I2cAddr,
    uld_platform::Pl,
};

#[cfg(feature = "single")]
use crate::ranging::Ranging;
#[cfg(feature = "flock")]
use crate::ranging_flock::RangingFlock;

pub struct VL {
    uld: State_HP_Idle,   // initialized ULD level driver, with dedicated I2C address
}

impl<'a> VL {
    pub fn new_and_setup<T: Instance + 'static>(i2c_shared: &'static RefCell<I2c<'a, T, Blocking>>,
        i2c_addr: I2cAddr
    ) -> Result<Self> {

        // Note: It seems the VL53L5CX doesn't retain its I2C address. Thus, we start each session
        //      by not only initializing the firmware (in '.init()') but also from the default I2C
        //      address. tbd. CONFIRM!!
        //
        let pl = Pl::new(i2c_shared);

        let mut uld = VL53L5CX::ping_new(pl)?.init()?;

        let a = i2c_addr.as_7bit() << 1;    // vendor ULD uses 8-bit addresses (LSB == 0)
        if a != DEFAULT_I2C_ADDR_8BIT {
            uld.set_i2c_address(a)?;
        }
        debug!("Board now reachable as: {}", i2c_addr);

        Ok(Self{
            uld,
        })
    }

    /*
    * Start ranging on a single board, with an 'INT' pin wired.
    */
    #[cfg(feature = "single")]
    pub fn start_ranging<const DIM: usize>(self, cfg: &RangingConfig<DIM>, pinINT: Input<'static>) -> Result<Ranging<DIM>> {
        Ranging::start(self, cfg, pinINT)
    }

    /*
    * A consuming method, used when moving to "Ranging" state.
    */
    pub(crate) fn into_uld(self) -> State_HP_Idle {
        self.uld
    }

    pub(crate) fn recreate(uld: State_HP_Idle) -> Self {
        Self { uld }
    }
}

/*
* For multiple boards, we can extend the slice itself; this is really handy!
*
* Note: Ranging for a single board is done differently than for multiple, because there are
*       differences. The single board case doesn't need to suffer from unneeded complexity.
*/
#[cfg(feature = "flock")]
pub trait VLsExt<const N: usize, const DIM: usize> {
    fn start_ranging(self, cfg: &RangingConfig<DIM>, pinINT: Input<'static>) -> Result<RangingFlock<N,DIM>>;
}

#[cfg(feature = "flock")]
impl<const N: usize, const DIM: usize> VLsExt<N,DIM> for [VL;N] {
    fn start_ranging(self, cfg: &RangingConfig<DIM>, pinINT: Input<'static>) -> Result<RangingFlock<N,DIM>> {
        RangingFlock::start(self, cfg, pinINT)
    }
    /***
    <<
        Trait `FromIterator<Result<State_Ranging<{ DIM }>, Error>>` is not implemented for `[State_Ranging<{ DIM }>; N]` [E0277]
    <<
    let tmp = self.into_iter().map(|x|
        x.into_uld().start_ranging(cfg)
    ).collect::<[State_Ranging<DIM>;N]>();
    ...
    ***/
}
