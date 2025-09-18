/*
* Reading two (or more) boards, using Embassy for multitasking.
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

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, DynamicReceiver, DynamicSender},
    signal::Signal,
    //Rwatch::{DynReceiver, DynSender, Watch}
};

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
    FlockResults,
    I2cAddr,
    Mode::*,
    RangingConfig,
    TargetOrder::*,
    VL53,
    VLsExt as _,
};

include!("../tmp/pins_snippet.in");  // pins!, boards!

const RESO: usize = 4;
type FRes = FlockResults<RESO>;

static I2C_SC: StaticCell<RefCell<I2c<'static, Blocking>>> = StaticCell::new();

#[allow(non_upper_case_globals)]
const I2C_SPEED: Rate = Rate::from_khz(1000);        // max 1000

#[allow(non_snake_case)]
struct Pins<'a, const BOARDS: usize>{
    SDA: AnyPin<'a>,
    SCL: AnyPin<'a>,
    PWR_EN: AnyPin<'a>,
    INT: AnyPin<'a>,
    #[cfg(feature = "vl53l8cx")]
    #[allow(dead_code)]
    SYNC: Option<AnyPin<'a>>,
    LPn: [AnyPin<'a>;BOARDS],
}

// Need to have it; no way around (see below).
const BOARDS_N: usize = boards!();

static DONE: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    #[allow(non_snake_case)]
    #[cfg(feature="vl53l8cx")]
    let Pins{ SDA, SCL, PWR_EN, INT, SYNC: _, LPn } = pins!(peripherals);

    #[allow(non_snake_case)]
    #[cfg(not(feature="vl53l8cx"))]
    let Pins{ SDA, SCL, PWR_EN, INT, LPn, .. } = pins!(peripherals);

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
                |i| I2cAddr::from_7bit(DEFAULT_I2C_ADDR.as_7bit() + (i as u8))
    ).unwrap();

    info!("Init succeeded");

    // Handling the results is separated from fetching them.
    //
    // 'embassy-sync' note:
    //      'Dynamic{Receiver|Sender}' are simpler type definitions for 'Receiver|Sender'. We embrace
    //      the simplicity of the types! :)
    //
    // 'DynamicReceiver':   "Receive-only access to a Channel without knowing channel size."
    // 'DynamicSendor':     (similar)
    //
    static CHANNEL: Channel<CriticalSectionRawMutex, FRes, 2 /*max receivers*/> = Channel::new();
    let (snd,rcv) = (CHANNEL.dyn_sender(), CHANNEL.dyn_receiver());

    spawner.spawn(ranging(vls, INT, snd)).unwrap();

    spawner.spawn(print_results(rcv)).unwrap();

    DONE.wait() .await;

    // Check that the queue gets fully emptied, before returning to host OS prompt.
    {
        let n = CHANNEL.len();
        debug!("After scans are done, {} results still in the Channel.", n);

        //|if b {
        //|    async_delay_ms(1).await;    // does this help it pass?
        //|    debug!("!!! {}", snd.contains_value());
        //|}
    }

    semihosting::process::exit(0);
}

// Passing 'vls' to the task is a bit tricky..
//  - '&[VL53]' would need the argument (because of task) to be ''static'.
//  - '[VL53;_]' is not allowed
//  - '[VL53;SIZE]' works, but we don't have (a way to find out) the 'SIZE'
//  - 'SIZE' cannot be made a const generic (because of task)
//  - ..?
//
// The easiest (well, the way that works!) seems to be to expose the number of 'LPns' from the
// 'pins' system. DEAL WITH THIS AS AN INTERIM SOLUTION; one to just use 'LPns' is preferred!
//
#[embassy_executor::task]
#[allow(non_snake_case)]
async fn ranging(/*move*/ vls: [VL53;BOARDS_N], pin_INT: Input<'static>, snd: DynamicSender<'static, FRes>) {

    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(5.ms(), HzU8(10)))
        .with_target_order(CLOSEST);

    let mut ring = vls.start_ranging(&c, pin_INT).unwrap();

    let mut seen = [false;BOARDS_N];

    for _round in 0..10 {
        let mut _t = Timings::new();

        let t: FlockResults<4> = ring.get_data() .await
            .unwrap();
        _t.results();

        if !seen[t.board_index] {
            seen[t.board_index] = true;
            info!("Skipping first results for board #{} (normally not valid)", t.board_index);
            continue;
        }

        snd.send(t) .await;

        _t.results_passed().report();
    }

    DONE.signal(());
}

#[embassy_executor::task]
async fn print_results(rcv: DynamicReceiver<'static, FRes>) {
    let mut t0: Option<Instant> = None;

    loop {
        let FlockResults{board_index, res, temp_degc, time_stamp}
            = rcv.receive() .await;

        let dt: Duration = time_stamp - *(t0.get_or_insert(time_stamp));

        info!("\n\t\tData #{}: ({}, at {:ms}s)", board_index, temp_degc, dt.as_millis());

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
}

//---
struct Timings {
    t0: Instant,    // start
    t1: Instant,    // results read
    t2: Instant,    // results passed
}

impl Timings {
    fn new() -> Self {
        let dummy: Instant = Instant::EPOCH;
        Self{ t0: Instant::now(), t1: dummy, t2: dummy }
    }

    fn results(&mut self) { self.t1 = Instant::now(); }

    fn results_passed(mut self) -> Self {
        self.t2 = Instant::now(); self
    }

    fn report(/*move*/ self) {
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

// Note: Could also use 'embassy_time::Delay::delay_ms' (but it's a bit elaborate)

const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
