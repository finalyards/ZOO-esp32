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
use embassy_time::{Duration as EmbDuration, Timer};

use esp_hal::{
    gpio::{Io, Input},
    i2c::I2c,
    prelude::*,
    time::{now, Instant, Duration},
    timer::timg::TimerGroup,
};

extern crate vl53l5cx_uld as uld;

include!("./pins_gen.in");  // pins!

use uld::{
    VL53L5CX,
    ranging::{
        RangingConfig,
        TargetOrder::CLOSEST,
        Mode::AUTONOMOUS,
    },
    units::*,
    API_REVISION,
    DEFAULT_I2C_ADDR_8BIT,
};

extern crate vl53l5cx_many as vl;
use vl::{
    I2cAddr,
    VL
};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    init_defmt();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    #[allow(non_snake_case)]
    let (SDA, SCL, PWR_EN, INT, mut LPns) = pins!(io);

    let i2c_bus = I2c::new(
        peripherals.I2C0,
        SDA,
        SCL,
        400.kHz()
    );

    let i2c_shared = RefCell::new(i2c_bus);

    // Reset VL53L5CX by pulling down their power for a moment
    if let Some(mut pin) = PWR_EN {
        pin.set_low();
        blocking_delay_ms(2);      // tbd. how long is suitable? This is the time the chip itself is with low power. #measure
        pin.set_high();
        info!("Target powered off and on again.");
    }

    #[allow(non_snake_case)]
    let vls = LPns.enumerate().map(|LPn,i| {
        let i2c_addr = DEFAULT_I2C_ADDR_8BIT + i*2;
        VL::new_and_setup(LPn, i2c_shared, I2cAddr::from(i2c_addr))
    });

    info!("Init succeeded, driver version {}", API_REVISION);

    spawner.spawn(ranging(vls)).unwrap();
    //R spawner.spawn(track_INT(INT)).unwrap();
}


// Initially, have two tasks:
//  1. runs the TOF sensor
//  2. sees whether the 'INT' pin gets high->low edges, and logs them

#[embassy_executor::task]
async fn ranging(/*move*/ mut vls: &[VL]) {

    let c = RangingConfig::<4>::default()
        .with_mode(AUTONOMOUS(Ms(5),Hz(10)))
        .with_target_order(CLOSEST);

    let mut ring = vl::start_ranging(vls, &c);

    let t = Timings::new();

    for round in 0..10 {
        t.t0();

        let results = ring.get_data() .await;
        t.results_read();

        // tbd. Consider making output a separate task (feed via a channel)
        //
        for (res, i, temp_degc) in results {
            info!("Data #{} ({})", round, temp_degc);

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
        t.results_passed();
        t.report();
    }
}

/***R
#[embassy_executor::task]
#[allow(non_snake_case)]
async fn track_INT(mut pin: Input<'static>) {

    loop {
        pin.wait_for_rising_edge().await;
        debug!("INT detected");
    }
}***/

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
*
* Note: 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
*       read operation". Our 'esp_hal::time::now' isn't, but sure seems to work.
*
* Reference:
*   - defmt book > ... > Hardware timestamp
*       -> https://defmt.ferrous-systems.com/timestamps#hardware-timestamp
*/
fn init_defmt() {
    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
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
    fn results_read(&mut self) {
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
