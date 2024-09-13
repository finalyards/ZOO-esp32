/*
* Support '{t}' in 'defmt' logging
*
* Reference:
*   - defmt book > ... > Hardware timestamp
*       -> https://defmt.ferrous-systems.com/timestamps#hardware-timestamp
*/
#[cfg(feature="next_api")]
use esp_hal::time::now as now;
#[cfg(not(feature="next_api"))]
use esp_hal::time::current_time as now;

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
        now().duration_since_epoch().to_micros()
    });
}
