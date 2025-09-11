/*
* Reading a single board, using Embassy.
*/
#![no_std]
#![no_main]

#![allow(for_loops_over_fallibles)]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use esp_backtrace as _;
//R? use embassy_time as _;  // show it used in Cargo.toml

use core::cell::RefCell;

use embassy_executor::Spawner;

use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, Input, InputConfig, Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    time::{Instant, Duration},
    timer::timg::TimerGroup,
    Blocking
};

use static_cell::StaticCell;

extern crate vl_api;
use vl_api::{
    DEFAULT_I2C_ADDR,
    Mode::*,
    RangingConfig,
    SoloResults,
    TargetOrder::*,
    ULD_VERSION,
    VL53,
    units::*
};

include!("../tmp/pins_snippet.in");  // pins!, boards!

static I2C_SC: StaticCell<RefCell<I2c<'static, Blocking>>> = StaticCell::new();

const BOARDS_N: usize = boards!();

#[allow(non_snake_case)]
struct Pins<'a>{
    SDA: AnyPin<'a>,
    SCL: AnyPin<'a>,
    PWR_EN: AnyPin<'a>,
    INT: AnyPin<'a>,
    SYNC: Option<AnyPin<'a>>,   // 'pins!()' needs the field to exist
    LPn: [AnyPin<'a>;BOARDS_N]
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, INT, LPn, .. } = pins!(peripherals);   // not needing 'SYNC'

    #[allow(non_snake_case)]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low, OutputConfig::default());

    #[allow(non_snake_case)]
    let INT = Input::new(INT, InputConfig::default());     // no 'Pull'

    // 'LPn's are pulled up within the SATELs (10k on front side of L8 level translator; 4.7k on L5CX),
    // but it turns out best to just drive them straight (SATEL only uses them as input).
    //
    #[allow(non_snake_case)]
    let mut LPns = LPn.map(|pin| {
        Output::new(pin, Level::Low, OutputConfig::default())   // disabled
    });
    let _ = LPns;
    LPns[0].set_high();     // enable the first board

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let i2c_cfg = I2cConfig::default()
        //R .with_frequency(50.kHz())   // WAS _THIS_ THAT MADE IT???? !===!?!?!?!?!?!??!?! tbd.tbd.-
        ;

    let i2c_bus = I2c::new(peripherals.I2C0, i2c_cfg)
        .unwrap()
        .with_sda(SDA)
        .with_scl(SCL);

    let tmp = RefCell::new(i2c_bus);
    let i2c_shared: &'static RefCell<I2c<Blocking>> = I2C_SC.init(tmp);

    // Reset VL53L5CX by pulling down their power for a moment
    {
        PWR_EN.set_low();
        blocking_delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        PWR_EN.set_high();
        info!("Target powered off and on again.");
    }

    // Enable one of the wired boards. Others remain low.
    LPns[0].set_high();

    let vl = VL53::new_and_setup(&i2c_shared, &DEFAULT_I2C_ADDR)
        .unwrap();

    info!("Init succeeded, ULD version {}", ULD_VERSION);

    spawner.spawn(ranging(vl, INT)).unwrap();
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
        if _round==0 { info!("Skipping first results (normally not valid)");
            continue;
        }

        _t.results();

        // tbd. Consider making output a separate task (feed via a channel)
        {
            info!("Data ({}, {})", temp_degc, (time_stamp-t0).as_millis());

            info!(".target_status:    {}", res.target_status);
            #[cfg(any(feature = "targets_per_zone_2", feature = "targets_per_zone_3", feature = "targets_per_zone_4"))]
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

// DO NOT use within the async portion!!!
const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}

/*
* Would rather not have this, but /something/ (only in 'single-emb', not 'many-emb') requires an allocator.
* Otherwise:
*   <<
*       error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait
*   <<
*
* tbd. #help What is causing this?  Some dependency needing 'anyhow', perhaps???
*
*   "INTERESTINGLY"... this doesn't need to be called. But it needs to exist, as a function. :R
*/
fn _init_heap() {
    use core::mem::MaybeUninit;

    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        #[allow(static_mut_refs)]
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}
