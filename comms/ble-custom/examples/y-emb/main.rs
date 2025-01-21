#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{info, debug};
use defmt_rtt as _;

//use esp_alloc as _;
use esp_backtrace as _;

//use bt_hci::controller::ExternalController;

use embassy_executor::Spawner;
/***
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    signal::Signal
};

use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup
};
use esp_wifi::ble::controller::BleConnector;
***/
//mod btn_sync;
//use btn_sync::{ButtonState, listen};

//static BTN_SIGNAL: Signal<CriticalSectionRawMutex, ButtonState> = Signal::new();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    debug!("aaa");
    init_defmt();

    /***
    let peripherals = esp_hal::init({
        let mut x = esp_hal::Config::default();
        x.cpu_clock = CpuClock::max();
        x
    });
    //esp_alloc::heap_allocator!(72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
        .unwrap();

    info!("OK");
    ***/

    loop {
        panic!("");
    }
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
    use esp_hal::time::now;

    defmt::timestamp!("{=u64:us}", {
        now().duration_since_epoch().to_micros()
    });
}

/***
fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}
***/
