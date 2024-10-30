/*
* Presents one VL53L5CX sensor, with its activation line and unique I2C address.
*
* Provides an async way for waiting for the next measurement.
*/
use core::cell::RefCell;

use esp_hal::{
    gpio::Output,
    i2c::{I2c, Instance},
    Blocking
};

use vl53l5cx_uld::{
    DEFAULT_I2C_ADDR_8BIT,
    //RangingConfig,
    VL53L5CX,
    VL53L5CX_InAction,
};

use crate::{
    I2cAddr,
    //RingN,
    uld_platform::Pl
};

pub struct VL {
    uld: VL53L5CX_InAction,   // initialized ULD level driver, with dedicated I2C address
    //R LPn: Output<'a>
}

impl<'a> VL {
    pub fn new_and_setup<T: Instance + 'static>(mut LPn: Output, i2c_shared: &'static RefCell<I2c<'a, T, Blocking>>, i2c_addr: I2cAddr) -> Self {

        // Note: It seems the VL53L5CX doesn't retain its I2C address. Thus, we start each session
        //      by not only initializing the firmware (in '.init()') but also from the default I2C
        //      address. tbd. CONFIRM!!
        //
        let pl = Pl::new(i2c_shared /*, i2c_addr*/);

        /***R?  -- 'pl' siirto closuren sisällä ei toiminut; toisaalta ei varmaa, tarvitaanko LPn:ää alun jälkeen
        // Enable our chip/board for the duration of initializing it
        //
        let uld = with(&mut LPn, || {
            let mut uld = VL53L5CX::new_maybe(pl).unwrap()
                .init().unwrap();

            let a = i2c_addr.as_7bit() << 1;    // vendor ULD uses 8-bit addresses (LSB == 0)
            if a != DEFAULT_I2C_ADDR_8BIT {
                uld.set_i2c_address_CAREFUL(a);
            }
            uld
        });
        ***/

        // Enable our chip/board - and keep it enabled "forever".
        //
        LPn.set_high();

        let mut uld = VL53L5CX::new_maybe(pl).unwrap()
            .init().unwrap();

        let a = i2c_addr.as_7bit() << 1;    // vendor ULD uses 8-bit addresses (LSB == 0)
        if a != DEFAULT_I2C_ADDR_8BIT {
            uld.set_i2c_address_CAREFUL(a).unwrap();
        }

        Self{
            uld,
            //R LPn     // tbd. do we actually need this, any more?
        }
    }

    /*
    * Start ranging just on a single board.
    */
    #[cfg(not(all()))]  //#later
    pub fn start_ranging<const DIM: usize>(&mut self, cfg: &RangingConfig) -> RingN<1,DIM> {
        RingN::<1,DIM>::start_one(&[self], cfg)
    }

    pub(crate) fn borrow_uld_mut(&mut self) -> &mut VL53L5CX_InAction {
        &mut self.uld
    }
}

fn with<T>(LPn: &mut Output, f: impl Fn() -> T) -> T {
    let ret;
    LPn.set_high();
    {
        ret = f();
    }
    LPn.set_low();
    ret
}
