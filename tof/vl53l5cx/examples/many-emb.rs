/*
* Reading two (or more) boards, using Embassy for multitasking.
*/
#![no_std]
#![no_main]

#![allow(for_loops_over_fallibles)]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};

use {defmt_rtt as _, esp_backtrace as _};

use core::cell::RefCell;

use embassy_executor::Spawner;

use esp_hal::{
    delay::Delay,
    gpio::{Io, Input, Output},
    i2c::I2c,
    peripherals::I2C0,
    prelude::*,
    time::{now, Instant, Duration},
    timer::timg::TimerGroup,
    Blocking
};

use static_cell::StaticCell;

extern crate vl53l5cx;
use vl53l5cx::{
    units::*,
    DEFAULT_I2C_ADDR,
    Mode::*,
    RangingConfig,
    TargetOrder::*,
    VL,
    VLsExt as _
};

mod common;
use common::{
    init_defmt,
    init_heap
};

const DEFAULT_I2C_ADDR_8BIT: u8 = DEFAULT_I2C_ADDR.as_8bit();   // 0x52
const BOARDS: usize = 2;     // number of boards

include!("./pins_gen.in");  // pins!

type I2cType<'d> = I2c<'d, I2C0,Blocking>;
static I2C_SC: StaticCell<RefCell<I2cType>> = StaticCell::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    init_defmt();
    init_heap();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    #[allow(non_snake_case)]
    let (SDA, SCL, PWR_EN, LPns, INT): (_,_,_,[Output;BOARDS],_) = pins!(io);

    let i2c_bus = I2c::new(
        peripherals.I2C0,
        SDA,
        SCL,
        400.kHz()
    );

    let tmp = RefCell::new(i2c_bus);
    let i2c_shared: &'static RefCell<I2c<I2C0,Blocking>> = I2C_SC.init(tmp);

    // Reset VL53L5CX's by pulling down their power for a moment
    if let Some(mut pin) = PWR_EN {
        pin.set_low();
        blocking_delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        pin.set_high();
        info!("Targets powered off and on again.");
    }

    let vls: [VL;BOARDS] = VL::new_flock(LPns, i2c_shared,
|i| I2cAddr::from_8bit(DEFAULT_I2C_ADDR_8BIT + (i as u8)*2)
        ).unwrap();

    /***R
    let vls: [VL;BOARDS] = array_try_map_mut_enumerated(LPns, #[allow(non_snake_case)] |(i,LPn)| {
        LPn.set_high();     // enable this chip and leave it on

        let i2c_addr = I2cAddr::from_8bit(DEFAULT_I2C_ADDR_8BIT + (i as u8)*2);
        let vl = VL::new_and_setup(i2c_shared, i2c_addr)?;

        debug!("Init of board {} succeeded", i);
        Ok(vl)
    }).unwrap();
    ***/
    info!("Init succeeded");

    spawner.spawn(ranging(vls, INT)).unwrap();
}


// Initially, have two tasks:
//  1. runs the TOF sensor
//  2. sees whether the 'INT' pin gets high->low edges, and logs them

#[embassy_executor::task]
#[allow(non_snake_case)]
async fn ranging(/*move*/ vls: [VL;BOARDS], pinINT: Input<'static>) {

    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(), HzU8(10)))
        .with_target_order(CLOSEST);

    let mut ring = vls.start_ranging(&c, pinINT).unwrap();

    let t0 = now();
    let mut _t = Timings::new();

    for _round in 0..10 {
        _t.t0();

        let (i, res, temp_degc, time_stamp) = ring.get_data() .await
            .unwrap();

        _t.results();

        // tbd. Consider making output a separate task (feed via a channel)
        {
            info!("Data #{}: ({}, {})", i, temp_degc, (time_stamp-t0).to_millis());

            info!(".target_status:    {}", res.target_status);
            info!(".targets_detected: {}", res.targets_detected);

            #[cfg(feature = "ambient_per_spad")]
            info!(".ambient_per_spad: {}", res.ambient_per_spad);
            #[cfg(feature = "nb_spads_enabled")]
            info!(".spads_enabled:    {}", res.spads_enabled);
            #[cfg(feature = "signal_per_spad")]
            info!(".signal_per_spad:  {}", res.signal_per_spad);
            #[cfg(feature = "range_sigma_mm")]
            info!(".range_sigma_mm:   {}", res.range_sigma_mm);
            #[cfg(feature = "distance_mm")]
            info!(".distance_mm:      {}", res.distance_mm);
            #[cfg(feature = "reflectance_percent")]
            info!(".reflectance:      {}", res.reflectance);
        }
        _t.results_passed();
        _t.report();
    }
}

struct Timings {
    t0: Instant,
    t1: Instant,    // results read
    t2: Instant,    // results passed
}

impl Timings {
    fn new() -> Self {
        let dummy = Instant::from_ticks(0);
        Self{ t0: dummy, t1: dummy, t2: dummy }
    }

    fn t0(&mut self) {
        self.t0 = now();
    }
    fn results(&mut self) {
        self.t1 = now();
    }
    fn results_passed(&mut self) {
        self.t2 = now();
    }

    fn report(&mut self) {
        let dt_total = self.t2 - self.t0;
        let dt1 = self.t1 - self.t0;
        let dt2 = self.t2 - self.t1;

        fn ms(dur: /*&*/Duration) -> f32 {
            dur.to_micros() as f32 / 1000.0
        }

        debug!("Timing [ms] (total {=f32}): wait+read {}, passing {}", ms(dt_total), ms(dt1), ms(dt2));
    }
}

// DO NOT use within the async portion!!!
const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}

/***R
type UldResult<T> = Result<T,vl53l5cx_uld::Error>;
fn array_try_map_mut_enumerated<A,B, const N: usize>(mut aa: [A;N], f: impl FnMut((usize,&mut A)) -> UldResult<B>) -> UldResult<[B;N]> {
    use arrayvec::ArrayVec;
    let bs_av = aa.iter_mut().enumerate().map(f)
        .collect::<UldResult<ArrayVec<B,N>>>();

    bs_av.map(|x| x.into_inner().ok().unwrap())
}
***/