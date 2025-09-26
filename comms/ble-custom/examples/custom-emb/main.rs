#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

use embassy_time as _;      // show enabled in 'Cargo.toml'; we want the time stamp for 'defmt' logs

//use esp_alloc;
use esp_backtrace as _;
//use static_cell as _;       // enable in 'Cargo.toml'

use embassy_executor::Spawner;
use embassy_sync::signal::Signal;
use esp_hal::{
    clock::CpuClock,
    efuse::Efuse,
    gpio::{AnyPin, Input, InputConfig, Pull},
    rng::{Trng, TrngSource},
    timer::{
        systimer::SystemTimer,
        timg::TimerGroup
    }
};
use esp_radio::{
    ble::controller::BleConnector,
    Controller,
};
use static_cell::StaticCell;
#[allow(unused_imports)]
use trouble_host::{
    prelude::*,
    Address,
};

mod btn_task;
mod btn_gatt;
mod gatt_server;

include!("../../tmp/pins_snippet.in");  // pins!

use crate::{
    btn_task::{BtnSignal, btn_task},
};

pub(crate) static BTN_SIGNAL: BtnSignal = Signal::new();

#[allow(non_snake_case)]
struct Pins<'a>{
    BOOT: AnyPin<'a>
}

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> () /* !*/ {      // '!' is still a nightly type
    let peripherals = esp_hal::init(
        esp_hal::Config::default()
            .with_cpu_clock(CpuClock::max())
    );
    esp_alloc::heap_allocator!(size: 72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    // only RISC V boards supported
    //#[cfg(target_arch = "riscv32")]
    let software_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    esp_preempt::start(
        timg0.timer0,
        software_interrupt.software_interrupt0
    );

    static RADIO: StaticCell<Controller<'static>> = StaticCell::new();
    let radio = RADIO.init(esp_radio::init().unwrap());

    // only RISC V boards supported
    {
        let tmp = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(tmp.alarm0);
    }

    let controller: ExternalController<_, 20 /*SLOTS*/> = {
        let bluetooth = peripherals.BT;
        let tmp = BleConnector::new(radio, bluetooth);
        ExternalController::new(tmp)
    };

    #[allow(non_snake_case)]
    let Pins{ BOOT } = pins!(peripherals);

    #[allow(non_snake_case)]
    let BOOT = Input::new(BOOT, InputConfig::default()
        .with_pull(Pull::Up)
    );

    // Boot button task is being run constantly on the background (even when there's no BLE
    // connection). This is just a matter of taste - use 'AnyServiceTask' for running something
    // just when connected.
    //
    spawner.spawn(btn_task(BOOT, &BTN_SIGNAL))
        .unwrap();

    let _trng_source = TrngSource::new(peripherals.RNG, peripherals.ADC1);
    let mut trng = Trng::try_new().unwrap();    // "succeeds, when '_trng_source' is alive" (but  when the 'Trng' is used) #unsure #tbd.

    // Address is Random, as in -> https://embassy.dev/trouble/#_random_address
    let a: Address = Address::random(Efuse::mac_address());     // 6 bytes MAC
    #[cfg(false)]   // Using a fixed address can be useful for testing.
    let a: Address = Address::random(b"rand0m".into());

    debug!("Our address = {:02x}", a.addr.raw());    // output as: "10:15:07:04:32:54" tbd.

    let (mut ress, stack);   // for lifespan
    let host = {
        use trouble_host::HostResources;

        const CONNECTIONS_MAX: usize = 1;       // max nbr of connections
        const L2CAP_CHANNELS_MAX: usize = 2;    // max nbr of L2CAP channels    // tbd. pls explain...

        ress = HostResources::<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX>::new();

        stack = trouble_host::new(controller, &mut ress)
            .set_random_address(a)
            .set_random_generator_seed(&mut trng);
        stack.build()
    };

    gatt_server::run(host) .await;
}
