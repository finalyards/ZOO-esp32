/*
* Reading a single board, using Embassy.
*/
#![no_std]
#![no_main]

#![allow(for_loops_over_fallibles)]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;     // needed for the panic handler to actually kick in
use embassy_time as _;      // show it used in Cargo.toml

use esp_alloc as _;

use core::cell::RefCell;

use embassy_executor::Spawner;

use embassy_sync::signal::Signal;

use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, Input, InputConfig, Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    time::{Instant, Duration, Rate},
    timer::timg::TimerGroup,
    Blocking
};

use semihosting;
use static_cell::StaticCell;

extern crate vl_api;
use vl_api::{
    units::*,
    DEFAULT_I2C_ADDR,
    Mode::*,
    RangingConfig,
    SoloResults,
    TargetOrder::*,
    VL53,
};

include!("../tmp/pins_snippet.in");  // pins!

static I2C_SC: StaticCell<RefCell<I2c<'static, Blocking>>> = StaticCell::new();

#[allow(non_upper_case_globals)]
const I2C_SPEED: Rate = Rate::from_khz(400);        // max 1000

#[allow(dead_code)]     // allow 'SYNC' to not be used
#[allow(non_snake_case)]
struct Pins<'a,const BOARDS: usize>{
    SDA: AnyPin<'a>,
    SCL: AnyPin<'a>,
    PWR_EN: AnyPin<'a>,
    INT: AnyPin<'a>,
    #[cfg(feature = "vl53l8cx")]
    SYNC: Option<AnyPin<'a>>,   // 'pins!()' needs the field to exist (even though we don't use it)
    LPn: [AnyPin<'a>;BOARDS]
}

static DONE: Signal<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, ()> = Signal::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, INT, LPn, .. } = pins!(peripherals);

    #[allow(non_snake_case)]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low, OutputConfig::default());

    #[allow(non_snake_case)]
    let INT = Input::new(INT, InputConfig::default());     // no pull

    // 'LPn's are pulled up within the SATELs (10k on front side of L8 level translator; 4.7k on L5CX),
    // but it turns out best to just drive them straight.
    //
    #[allow(non_snake_case)]
    let mut LPn = LPn.map(|pin| {
        Output::new(pin, Level::Low, OutputConfig::default())
    });

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let i2c_shared: &'static RefCell<I2c<Blocking>> = {
        let x = I2c::new(peripherals.I2C0, I2cConfig::default()
            .with_frequency(I2C_SPEED)
        ).unwrap();

        let i2c_bus = x
            .with_sda(SDA)
            .with_scl(SCL);

        let tmp = RefCell::new(i2c_bus);
        I2C_SC.init(tmp)
    };

    // Reset VL53's by pulling down their power for a moment
    {
        PWR_EN.set_low();
        blocking_delay_ms(10);      // L5CX: 10ms based on UM2884 Rev. 6, Chapter 4.2
        PWR_EN.set_high();
        info!("Target powered off and on again.");
    }

    // Enable one of the wired boards. Others remain low.
    LPn[0].set_high();

    let vl = VL53::new_and_setup(&i2c_shared, &DEFAULT_I2C_ADDR)
        .unwrap();

    info!("Init succeeded");

    spawner.spawn(ranging(vl, INT)).unwrap();

    // Need something to wait, to know the task(s) are done.
    DONE.wait() .await;

    // Return to the host command line
    semihosting::process::exit(0);
}


#[embassy_executor::task]
#[allow(non_snake_case)]
async fn ranging(/*move*/ vl: VL53, pinINT: Input<'static>) {
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),HzU8(10)))  // 10.Hz() with 'fugit::Rate'
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c, pinINT)
        .expect("ranging to start");

    let t0 = Instant::now();
    let mut _t = Timings::new();

    for _round in 0..10 {
        _t.t0();

        let SoloResults{res, temp_degc, time_stamp} = ring.get_data() .await
            .unwrap();

        // Note: Skip the first results. They are taken in a hurry (it seems; only taking ~20ms vs.
        //      normal 100ms) and are not that great quality.
        //
        if _round == 0 { continue; }

        _t.results();

        // Note: For separating the processing from the scanning, see 'many-emb.rs'.
        {
            info!("\n\t\tData ({}, at {:ms}s)", temp_degc, (time_stamp-t0).as_millis());

            info!("{}", res.meas);
            #[cfg(feature = "ambient_per_spad")]
            info!(".ambient_per_spad: {}", res.ambient_per_spad);
            #[cfg(feature = "nb_spads_enabled")]
            info!(".spads_enabled:    {}", res.spads_enabled);
            #[cfg(feature = "signal_per_spad")]
            info!(".signal_per_spad:  {}", res.signal_per_spad);
            #[cfg(feature = "range_sigma_mm")]
            info!(".range_sigma_mm:   {}", res.range_sigma_mm);
            #[cfg(feature = "reflectance_percent")]
            info!(".reflectance:      {}", res.reflectance);
        }
        _t.results_passed();
        _t.report();
    }

    DONE.signal(());
}

struct Timings {
    t0: Instant,
    t1: Instant,    // results read
    t2: Instant,    // results passed
}

impl Timings {
    fn new() -> Self {
        let dummy = Instant::EPOCH;
        Self{ t0: dummy, t1: dummy, t2: dummy }
    }

    fn t0(&mut self) {
        self.t0 = Instant::now();
    }
    fn results(&mut self) {
        self.t1 = Instant::now();
    }
    fn results_passed(&mut self) {
        self.t2 = Instant::now();
    }

    fn report(&mut self) {
        let dt_total = self.t2 - self.t0;
        let dt1 = self.t1 - self.t0;
        let dt2 = self.t2 - self.t1;

        fn ms(dur: /*&*/Duration) -> f32 {
            dur.as_micros() as f32 / 1000.0
        }

        debug!("Timing [ms] (total {=f32}): wait+read {}, passing {}", ms(dt_total), ms(dt1), ms(dt2));
    }
}

#[allow(dead_code)]
async fn async_delay_ms(ms: u32) {
    use embassy_time::Timer;

    Timer::after_millis(ms as _).await;
}

// DO NOT use within the async portion!!!
const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
