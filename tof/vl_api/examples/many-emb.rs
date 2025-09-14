/*
* Reading two (or more) boards, using Embassy for multitasking.
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

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::{DynReceiver, DynSender, Watch}
};

use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, Input, Output, InputConfig, OutputConfig, Level, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    time::{Instant, Duration, Rate},
    timer::timg::TimerGroup,
    Blocking
};

use static_cell::StaticCell;

extern crate vl_api;
use vl_api::{
    units::*,
    DEFAULT_I2C_ADDR,
    FlockResults,
    I2cAddr,
    Mode::*,
    RangingConfig,
    TargetOrder::*,
    VL53,
    VLsExt as _,
};

include!("../tmp/pins_snippet.in");  // pins!

const RESO: usize = 4;
type FRes = FlockResults<RESO>;

static I2C_SC: StaticCell<RefCell<I2c<'static, Blocking>>> = StaticCell::new();

//const BOARDS_N: usize = boards!();

#[allow(non_upper_case_globals)]
const I2C_SPEED: Rate = Rate::from_khz(400);        // max 1000

#[allow(non_snake_case)]
struct Pins<'a, const BOARDS: usize>{
    SDA: AnyPin<'a>,
    SCL: AnyPin<'a>,
    PWR_EN: AnyPin<'a>,
    INT: AnyPin<'a>,
    SYNC: Option<AnyPin<'a>>,
    LPn: [AnyPin<'a>;BOARDS],
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    //R? let o_low_cfg = OutputConfig::default().with_level(Level::Low);
    //R let i_no_pull = InputConfig::default().with_pull(Pull::None);

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, INT, SYNC, LPn } = pins!(peripherals);

    //R const _BOARDS: usize = LPns.length;

    #[allow(non_snake_case)]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low, OutputConfig::default());

    #[allow(non_snake_case)]
    let INT = Input::new(INT, InputConfig::default());  // no pull

    #[allow(non_snake_case)]
    let LPn = LPn.map(|pin| { Output::new(pin, Level::Low, OutputConfig::default()) });

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let i2c_shared: &'static RefCell<I2c<Blocking>> = {
        let i2c_bus = I2c::new(peripherals.I2C0, I2cConfig::default().with_frequency(I2C_SPEED))
            .unwrap()
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
        info!("Targets powered off and on again.");
    }

    let vls = VL53::new_flock(LPn, i2c_shared,
                |i| I2cAddr::from_7bit(DEFAULT_I2C_ADDR.as_7bit() + i)
    ).unwrap();

    info!("Init succeeded");

    // Create a way to separate ranging from passing on the results.
    //
    // Using 'embassy-sync::watch' |1|, because:
    //  - we don't ever want the producer to wait; i.e. prefer to lose some (older) measurement     // vs. 'channel::Channel'
    //
    // |1| https://docs.embassy.dev/embassy-sync/git/default/watch/index.html
    //
    // 'embassy-sync' note:
    //      'Dyn{Receiver|Sender}' are simpler type definitions for 'Receiver|Sender'. We embrace
    //      the simplicity of the types! :)
    //
    static WATCH: Watch<CriticalSectionRawMutex, FRes, 2 /*max receivers*/> = Watch::new();

    let snd = WATCH.dyn_sender();
    let rcv0 = WATCH.dyn_receiver().unwrap();

    spawner.spawn(ranging(vls, INT, snd)).unwrap();

    spawner.spawn(defmt_print_results(rcv0)).unwrap();
}

// Passing 'vls' to the task is a bit tricky..
//  - '&[VL]' would need the argument (because of task) to be ''static'.
//  - '[VL;_]' is not allowed
//  - '[VL;SIZE]' works, but we don't have (a way to find out) the 'SIZE'
//  - 'SIZE' cannot be made a const generic (because of task)
//  - ..?
//
// The easiest (well, the way that works!) seems to be to expose the number of 'LPns' from the
// 'pins' system. DEAL WITH THIS AS AN INTERIM SOLUTION; one to just work on 'LPns' is preferred!
//
#[embassy_executor::task]
#[allow(non_snake_case)]
async fn ranging(/*move*/ vls: [VL53;BOARDS], pin_INT: Input<'static>, snd: DynSender<'static, FRes>) {

    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(), HzU8(10)))
        .with_target_order(CLOSEST);

    let mut ring = vls.start_ranging(&c, pin_INT).unwrap();

    let mut had_results_from = [false;BOARDS];

    for _round in 0..10 {
        let mut _t = Timings::new();

        let t: FlockResults<4> = ring.get_data() .await
            .unwrap();
        _t.results();

        if !had_results_from[t.board_index] {
            had_results_from[t.board_index] = true;
            info!("Skipping first results (normally not valid)");
            continue;
        }

        snd.send(t);

        _t.results_passed().report();
    }

    // tbd. What happens when an Embassy task runs to its end?
    // is the way to close a task / the whole process. Would be clean.
}

#[embassy_executor::task]
async fn defmt_print_results(mut rcv: DynReceiver<'static, FRes>) {
    let mut t0: Option<Instant> = None;

    loop {
        let FlockResults{board_index, res, temp_degc, time_stamp} = rcv.changed().await;

        let dt: Duration = time_stamp - *(t0.get_or_insert(time_stamp));
        let sign = if dt.is_zero() {""} else {"+"};

        info!("Data #{}: ({}, {}{}ms)", board_index, temp_degc, sign, dt.to_millis());

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
}

//---
struct Timings {
    t0: Instant,    // start
    t1: Instant,    // results read
    t2: Instant,    // results passed
}

impl Timings {
    fn new() -> Self {
        Self{ t0: now(), t1: Self::DUMMY, t2: Self::DUMMY }
    }

    fn results(&mut self) { self.t1 = now(); }

    fn results_passed(mut self) -> Self {
        self.t2 = now(); self
    }

    fn report(/*move*/ self) {
        let dt_total = self.t2 - self.t0;
        let dt1 = self.t1 - self.t0;
        let dt2 = self.t2 - self.t1;

        fn ms(dur: /*&*/Duration) -> f32 {
            dur.to_micros() as f32 / 1000.0
        }

        debug!("Timing [ms] (total {=f32}): wait+read {}, passing {}", ms(dt_total), ms(dt1), ms(dt2));
    }

    const DUMMY: Instant = Instant::from_ticks(0);
}

// Note: Could also use 'embassy_time::Delay::delay_ms' (but it's a bit elaborate)

const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}

// Something brings in need for global allocator. Fake it!!!
//
use alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        0 as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unreachable!();     // since we never allocate
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;
