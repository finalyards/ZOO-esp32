/*
* Reading a single board, using Embassy.
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
    gpio::{Io, Input},
    i2c::I2c,
    peripherals::I2C0,
    prelude::*,
    time::{now, Instant, Duration},
    timer::timg::TimerGroup,
    Blocking
};
use esp_hal::gpio::Output;
use static_cell::StaticCell;

extern crate vl53l5cx;
use vl53l5cx::{
    DEFAULT_I2C_ADDR,
    Mode::*,
    RangingConfig,
    TargetOrder::*,
    ULD_VERSION,
    VL,
    units::*
};

mod common;
use common::{
    init_defmt,
    init_heap
};

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

    // Reset VL53L5CX by pulling down their power for a moment
    if let Some(mut pin) = PWR_EN {
        pin.set_low();
        blocking_delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        pin.set_high();
        info!("Target powered off and on again.");
    }

    // Enable one of the wired boards. Ensures that the others (if any) won't jump on the I2C bus.
    //
    // Note: This also works (gets ignored) if 'LPns' is empty (and the one LPn line has been pulled up).
    //
    for (i,pin) in LPns.iter_mut().enumerate() {
        if i==0 {
            pin.set_high();
        } else {
            pin.set_low();
        }
    }

    let vl = VL::new_and_setup(&i2c_shared, DEFAULT_I2C_ADDR)
        .unwrap();

    info!("Init succeeded, ULD version {}", ULD_VERSION);

    spawner.spawn(ranging(vl, INT)).unwrap();
}


#[embassy_executor::task]
#[allow(non_snake_case)]
async fn ranging(/*move*/ vl: VL, pinINT: Input<'static>) {

    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),HzU8(10)))  // 10.Hz() - but don't want to use 'fugit::Rate' in ULD project
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c, pinINT).unwrap();

    let t0 = now();
    let mut _t = Timings::new();

    for _round in 0..10 {
        _t.t0();

        let (res, temp_degc, time_stamp) = ring.get_data() .await
            .unwrap();

        _t.results();

        // tbd. Consider making output a separate task (feed via a channel)
        {
            info!("Data ({}, {})", temp_degc, (time_stamp-t0).to_millis());

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
