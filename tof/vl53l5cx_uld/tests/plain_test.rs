#![no_std]
#![no_main]

use defmt_rtt as _;

mod utils;

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use esp_hal::{
        delay::Delay,
        //prelude::*
    };
    use crate::utils::init_defmt;

    // Optional: A init function which is called before every test
    #[init]
    fn init() -> Delay {
        init_defmt();   // tbd. ideally, we'd call it only "once".

        //let peripherals = esp_hal::init(esp_hal::Config::default());
        let delay = Delay::new();

        // returned state can be consumed by the test cases
        delay
    }

    // The time stamp should be positive
    #[test]
    fn time_stamp_test() {
        assert!(esp_hal::time::now().duration_since_epoch().to_millis() > 0_u64)
    }
}
