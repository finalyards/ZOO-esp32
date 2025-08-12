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
use embassy_time as _;  // show it used in Cargo.toml

use core::cell::RefCell;

use embassy_executor::Spawner;

use esp_hal::{
    delay::Delay,
    gpio::Input,
    i2c::master::{Config as I2cConfig, I2c},
    time::{now, Instant, Duration},
    timer::timg::TimerGroup,
    Blocking
};
use esp_hal::gpio::{AnyPin, InputConfig, Level, Output, OutputConfig, Pull};
use fugit::RateExtU32;
use static_cell::StaticCell;

extern crate vl53l5cx;
use vl53l5cx::{
    DEFAULT_I2C_ADDR,
    Mode::*,
    RangingConfig,
    SoloResults,
    TargetOrder::*,
    ULD_VERSION,
    VL,
    units::*
};

include!("./pins_gen.in");  // pins!

static I2C_SC: StaticCell<RefCell<I2c<'static, Blocking>>> = StaticCell::new();

#[allow(non_snake_case)]
struct Pins<const BOARDS: usize>{
    SDA: AnyPin,
    SCL: AnyPin,
    PWR_EN: AnyPin,
    LPns: [AnyPin;BOARDS],
    INT: AnyPin
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    //init_heap();

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let o_low_cfg = OutputConfig::default().with_level(Level::Low);
    let i_pull_none_cfg = InputConfig::default().with_pull(Pull::None);

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, LPns, INT } = pins!(peripherals);

    #[allow(non_snake_case)]
    let mut PWR_EN = Output::new(PWR_EN, o_low_cfg).unwrap();
    #[allow(non_snake_case)]
    let mut LPns = LPns.map(|n| { Output::new(n, o_low_cfg).unwrap() });
    #[allow(non_snake_case)]
    let INT = Input::new(INT, i_pull_none_cfg).unwrap();

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let i2c_cfg = I2cConfig::default()
        .with_frequency(50.kHz())
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

    let vl = VL::new_and_setup(&i2c_shared, &DEFAULT_I2C_ADDR)
        .unwrap();

    info!("Init succeeded, ULD version {}", ULD_VERSION);

    spawner.spawn(ranging(vl, INT)).unwrap();
}


#[embassy_executor::task]
#[allow(non_snake_case)]
async fn ranging(/*move*/ vl: VL, pinINT: Input<'static>) {
    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(),HzU8(10)))  // 10.Hz() with 'fugit::Rate'
        .with_target_order(CLOSEST);

    let mut ring = vl.start_ranging(&c, pinINT)
        .unwrap();

    let t0 = now();
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
            info!("Data ({}, {})", temp_degc, (time_stamp-t0).to_millis());

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
