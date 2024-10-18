/*
* Testing whether we can mix 'defmt' and interactive terminal IO (including "target read").
*
* Chat on `esp-rs` matrix [says this can be done
*   ](https://matrix.to/#/#esp-rs:matrix.org/$pqQPbzoV6qfccsEEnetHKy-nR1wK5PoZze-q5Bocp3k).
*
*   tbd. TRY AGAIN ONCE > 0.21.0 is OUT!!  Turned difficult to use git 'main' (since it's not only 'esp-hal')
*/
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug, error, warn};
use defmt_rtt as _;

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};

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
    init_defmt();

    info!("Init!");
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let (rx, tx) = UsbSerialJtag::new_async(peripherals.USB_DEVICE).split();

    static SIGNAL: StaticCell<Signal<NoopRawMutex, heapless::String<MAX_BUFFER_SIZE>>> =
        StaticCell::new();
    let signal = &*SIGNAL.init(Signal::new());

    spawner.spawn(reader(rx, &signal)).unwrap();
    spawner.spawn(writer(tx, &signal)).unwrap();

    debug!("Main exiting.");
}

/*
* Tell 'defmt' how to support '{t}' (timestamp) in logging.
*
* Note: 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
*       read operation". Out 'esp_hal::time::now' isn't, but sure seems to work.
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

#[embassy_executor::task]
async fn writer(
    mut tx: UsbSerialJtagTx<'static, Async>,
    signal: &'static Signal<NoopRawMutex, heapless::String<MAX_BUFFER_SIZE>>,
) {
    use core::fmt::Write;
    debug!("Writer started.");

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
    debug!("Reader started.");

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
