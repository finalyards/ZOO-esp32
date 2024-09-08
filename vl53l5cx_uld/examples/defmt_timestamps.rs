/*
* Support '{t}' in 'defmt' logging
*
* Reference:
*   - defmt book > ... > Hardware timestamp
*       -> https://defmt.ferrous-systems.com/timestamps#hardware-timestamp
*/
use esp_hal::time::current_time;
use defmt::{debug, panic};
use esp_hal::clock::Clocks;
use esp_hal::delay::Delay;
use esp_hal::timer;

// Must be called only once
//
// Note: 'defmt' sample insists the command to be: "(interrupt-safe) single instruction volatile
//      read operation". Not sure, whether 'current_time()' qualifies.
//
// Alternatives:
//  - SystemTimer::now() / 16
//      The 'current_time()' calls this underneath, but also provides a fall-back for "esp32" MCU.
//
pub fn init() {
    defmt::timestamp!("{=u64:us}", {
        current_time().duration_since_epoch().to_micros()
    });
}

// Test to try out different implementations
#[cfg(not(all()))]
pub fn test(clocks: &Clocks) -> ! {
    let d_provider = Delay::new(&clocks);
    let delay_ms = |ms| d_provider.delay_millis(ms);

    // using SystemTimer
    let mut t_last= timer::systimer::SystemTimer::now();
    for _ in 0..10 {
        delay_ms(10);
        let t = timer::systimer::SystemTimer::now();
        debug!("Timestamp test (SystemTimer) {=u64}", t - t_last);    // "current count of Unit 0 in the system timer"
        t_last = t;
    }

    /*** using TimerGroup 0
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let timer0 = timg0.timer0;

    let mut t_last = timer0.now();
    for _ in 0..10 {
        delay_ms(10);
        let t = timer0.now();
        let tmp = t.checked_duration_since(t_last).unwrap();
        debug!("Timestamp test (TimerGroup0) {}", tmp);
        t_last = t;
    }***/

    // 'current_time()' uses 'systimer::SystemTimer::now' under the hood (for other than ESP32)
    // Value is in having also an 'esp32' compatible implementation.
    //
    let mut t_last = current_time();
    for _ in 0..10 {
        delay_ms(10);
        let t1 = current_time();
        debug!("Timestamp test {}", t1.checked_duration_since(t_last).unwrap());
        t_last = t1;
    }

    panic!();
}
