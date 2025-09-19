/*
* 'defmt' logging together with interactive host/MCU comms.
*
* THIS DOES NOT WORK!  Compared to the 'esp-hal' example, the behaviour on the input key presses
*   is different. They are echoed locally but do not reach the MCU side.
*
* Based on:
*   - esp-hal: examples/[...]/embassy_usb_serial_jtag.rs
*
* References:
*   - asking this on Matrix 'esp-rs' channel (Oct-24)
*       -> https://matrix.to/#/#esp-rs:matrix.org/$i3m0AoEB_utQfHG3Wxlb5L5NfaT-c1VKJqrxcd5VDwk
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    time::now,
    timer::timg::TimerGroup,
    usb_serial_jtag::{UsbSerialJtag, UsbSerialJtagRx, UsbSerialJtagTx},
    Async,
};

use static_cell::StaticCell;

const MAX_BUFFER_SIZE: usize = 512;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let channels = rtt_target::rtt_init_default!();    // 1024 bytes for out ("Terminal"); 16 bytes for in
    rtt_target::set_defmt_channel(channels.up.0);

    init_defmt();
    info!("Init!");     // see that 'defmt' output works

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let (rx, tx) = UsbSerialJtag::new_async(peripherals.USB_DEVICE).split();

    static SIGNAL: StaticCell<Signal<NoopRawMutex, heapless::String<MAX_BUFFER_SIZE>>> =
        StaticCell::new();
    let signal = &*SIGNAL.init(Signal::new());

    spawner.spawn(reader(rx, &signal)).unwrap();
    spawner.spawn(writer(tx, &signal)).unwrap();
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging. Also
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

//const D_PROVIDER: Delay = Delay::new();
//fn delay_ms(ms: u32) { D_PROVIDER.delay_millis(ms); }

#[embassy_executor::task]
async fn writer(
    mut tx: UsbSerialJtagTx<'static, Async>,
    signal: &'static Signal<NoopRawMutex, heapless::String<MAX_BUFFER_SIZE>>,
) {
    use core::fmt::Write;
    embedded_io_async::Write::write_all(
        &mut tx,
        b"Hello async USB Serial JTAG. Type something.\r\n",
    )
        .await
        .unwrap();
    loop {
        let message = signal.wait().await;
        signal.reset();
        write!(&mut tx, "-- received '{}' --\r\n", message).unwrap();
        embedded_io_async::Write::flush(&mut tx).await.unwrap();
    }
}

#[embassy_executor::task]
async fn reader(
    mut rx: UsbSerialJtagRx<'static, Async>,
    signal: &'static Signal<NoopRawMutex, heapless::String<MAX_BUFFER_SIZE>>,
) {
    let mut rbuf = [0u8; MAX_BUFFER_SIZE];
    loop {
        let r = embedded_io_async::Read::read(&mut rx, &mut rbuf).await;
        match r {
            Ok(len) => {
                let mut string_buffer: heapless::Vec<_, MAX_BUFFER_SIZE> = heapless::Vec::new();
                string_buffer.extend_from_slice(&rbuf[..len]).unwrap();
                signal.signal(heapless::String::from_utf8(string_buffer).unwrap());
            }
            #[allow(unreachable_patterns)]
            Err(e) => error!("RX Error: {:?}", e),
        }
    }
}


/***
//! ```
//! use rtt_target::{rtt_init_default, rprintln};
//!
//! fn main() -> ! {
//!     let mode = loop {
//!         read = channels.down.0.read(&mut read_buf);
//!         for i in 0..read {
//!             match read_buf[i] as char {
//!                 '0' => break 0,
//!                 '1' => break 1,
//!                 _ => {}
//!             }
//!         }
//!     };
//! }
***/