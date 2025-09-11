/*
* Just read some data from a Satel board.
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn, panic};

#[cfg(feature = "run_with_espflash")]
use esp_println as _;
#[cfg(feature = "run_with_probe_rs")]
use defmt_rtt as _;

use esp_backtrace as _;

use esp_hal::{
    delay::Delay,
    gpio::{AnyPin, DriveMode, Input, InputConfig, Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    main,
    time::{Instant, Rate}
};

extern crate vl_uld as uld;
use uld::{
    Result,
    VL53,
    RangingConfig,
    TargetOrder::CLOSEST,
    Mode::AUTONOMOUS,
    units::*,
};

#[cfg(feature = "vl53l8cx")]
use uld::SyncMode;

include!("../tmp/pins_snippet.in");  // pins!, boards!

mod common;
use common::MyPlatform;

#[main]
fn main() -> ! {
    match main2() {
        Err(e) => {
            panic!("Failed with ULD error code: {}", e);
        },

        Ok(()) => {
            info!("End of ULD demo");
            semihosting::process::exit(0);      // back to developer's command line
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]         // hides "field 'SYNC' is never read" -warning
struct Pins<'a> {
    SDA: AnyPin<'a>,
    SCL: AnyPin<'a>,
    PWR_EN: AnyPin<'a>,
    INT: AnyPin<'a>,
    #[cfg(feature = "vl53l8cx")]
    SYNC: Option<AnyPin<'a>>,   // not used, but needed by the 'pins!' macro
    LPn: [AnyPin<'a>; boards!()]
}

#[allow(non_upper_case_globals)]
const I2C_SPEED: Rate = Rate::from_khz(400);        // max 1000

//?? #[cfg(feature="run_with_espflash")]
//?? esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(false)]
fn main3() -> Result<()> {
    use esp_hal::gpio::Pull;

    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Drive PWR_EN up; otherwise no IOVdd to raise the LPn.
    #[allow(non_snake_case)]
    let _PWR_EN = {
        let ret = Output::new(peripherals.GPIO18, Level::High, OutputConfig::default());
        info!("PWR_EN pulled up.");
        ret
    };

    // Measure voltages here - 3v3, 1v8, IOVdd

    let c_drain: OutputConfig = OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain);     // SATEL has external pull-up resistor

    // Loop, changing GPIO21 (LPn) between High/Low
    //let mut PIN = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());  // works
    let mut PIN = Output::new(peripherals.GPIO21, Level::High, c_drain);

    debug!("Initial state should be: UP");
    blocking_delay_ms(5000);

    let mut up: bool = true;
    loop {
        blocking_delay_ms(2600);

        if up {
            PIN.set_low();
            debug!("DOWN");
        } else {
            PIN.set_high();
            debug!("UP");
        }
        up = !up;
    }
}

fn main2() -> Result<()> {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    #[allow(non_snake_case)]
    let Pins{ SDA, SCL, PWR_EN, INT, LPn, .. } = pins!(peripherals);

    #[allow(non_snake_case)]
    let mut PWR_EN = Output::new(PWR_EN, Level::Low, OutputConfig::default());

    #[allow(non_snake_case)]
    let INT = Input::new(INT, InputConfig::default());  // no pull

    // Set 'LPn' pins (if any) to suitable states:
    //  - 0: let it float HIGH (= "logical low"); we want to talk to it
    //  - 1..: LOW disabled them from the I2C bus
    //
    // If there are no LPn pins specified, that's fine: SATEL pulls its 'LPn' up.
    //
    // Note: Keeping the handle to the pins is optional. 'esp-hal' will likely not reset their
    //      config, but keeping the handle makes this safe.
    //
    #[allow(non_snake_case)]
    let LPn: [Output;boards!()] = {
        let c_opendrain: OutputConfig = OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain);  // SATEL has external pull-up resistor

        // Note! Tried with (fancier) '.iter().enumerate().map(|i,pin|)', but that complexity
        //      isn't well suited to 'AnyPin', 'OutputPin'. It causes a reference ('&AnyPin') to
        //      be taken, and dealing with that causes errors, no matter which way you take.
        //      'impl OutputPin' is defined for 'AnyPin', but not for '&AnyPin'.
        //
        let mut i=0;
        LPn.map(|pin| {
            let lev = if i==0 {Level::High} else {Level::Low};
            i += 1;
            Output::new(pin, lev, c_opendrain)
        })
    };
    let _ = LPn;

    let pl = {
        let x = I2c::new(peripherals.I2C0, I2cConfig::default()
            .with_frequency(I2C_SPEED)
        ).unwrap();

        let i2c_bus = x
            .with_sda(SDA)
            .with_scl(SCL);

        MyPlatform::new(i2c_bus)
    };

    // Reset VL53(s) by pulling down their power for a moment
    {
        PWR_EN.set_low();
        blocking_delay_ms(10);      // VL53L5CX: 10ms based on UM2884 Rev. 6, Chapter 4.2
                                        // VL54L8:   (no such reference found)
        PWR_EN.set_high();
        info!("Target powered off and on again.");
    }

    let vl = VL53::new_with_ping(pl)?.init()?;

    info!("Init succeeded");

    // Extra test, to see basic comms work
    #[cfg(false)]
    {
        let mut vl = vl;
        vl.i2c_no_op()
            .expect("to pass");
        info!("I2C no-op (get power mode) succeeded");
    }

    //--- ranging loop
    //
    let freq = Rate::from_hz(10);

    let c = {
        let m = AUTONOMOUS(5.ms(),HzU8(freq.as_hz() as u8),
        #[cfg(feature = "vl53l8cx")]
        SyncMode::NONE
        );

        RangingConfig::<4>::default()
            .with_mode(m)
            .with_target_order(CLOSEST)
    };

    let mut ring = vl.start_ranging(&c)
        .expect("to start ranging");

    for round in 0..3 {
        let t0= Instant::now();

        // wait for 'INT' to fall
        loop {
            if INT.is_low() {
                //debug!("INT after: {}", t0.elapsed());  // tbd. needs 'esp-hal' to have 'defmt' feature enabled
                debug!("INT after: {}ms", t0.elapsed().as_millis());
                break;
            } else if t0.elapsed().as_millis() > 1000 {
                panic!("No INT detected");
            }
            blocking_delay_us(20);   // < 100us
        }

        let (res, temp_degc) = ring.get_data()
            .expect("Failed to get data");

        info!("Data #{} ({})", round, temp_degc);

        #[cfg(feature = "target_status")]
        info!(".target_status:    {}", res.target_status);
        #[cfg(feature = "nb_targets_detected")]
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

    Ok(())
}

const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) { D_PROVIDER.delay_millis(ms); }
fn blocking_delay_us(us: u32) { D_PROVIDER.delay_micros(us); }

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
*
* Note! 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
*       read operation". Our 'Instant::now' isn't, but sure seems to work.
*
* Reference:
*   - defmt book > ... > Hardware timestamp
*       -> https://defmt.ferrous-systems.com/timestamps#hardware-timestamp
*
* Note: If you use Embassy, a better way is to depend on 'embassy-time' and enable its
*       "defmt-timestamp-uptime-*" feature.
*/
#[cfg(feature="run_with_probe_rs")]
defmt::timestamp!("{=u64:us}", {
    let now = Instant::now();
    now.duration_since_epoch().as_micros()
});

#[cfg(feature="run_with_espflash")]
#[unsafe(no_mangle)]
pub extern "Rust" fn _esp_println_timestamp() -> u64 {
    Instant::now()
        .duration_since_epoch()
        .as_millis()
}

