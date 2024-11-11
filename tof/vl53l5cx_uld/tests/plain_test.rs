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

    /***
    // A test which takes the state returned by the init function (optional)
    #[test]
    fn takes_state(_state: Delay) {
        assert!(true)
    }

    //|// Example for a test which is conditionally enabled
    //|#[test]
    //|#[cfg(feature = "log")]
    //|fn log() {
    //|    log::info!("Hello, log!"); // Prints via esp-println to rtt
    //|    assert!(true)
    //|}

    // Another example for a conditionally enabled test
    #[test]
    #[cfg(feature = "defmt")]
    fn defmt() {
        use defmt_rtt as _;
        defmt::info!("Hello, defmt!"); // Prints via defmt-rtt to rtt
        assert!(true)
    }

    // Tests can be ignored with the #[ignore] attribute
    #[test]
    #[ignore]
    fn it_works_ignored() {
        assert!(false)
    }

    // A test that fails with a panic
    #[test]
    fn it_fails1() {
        assert!(false)
    }

    // A test that fails with a returned Err(&str)
    #[test]
    fn it_fails2() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    // Tests can be annotated with #[should_panic] if they are expected to panic
    #[test]
    #[should_panic]
    fn it_passes() {
        assert!(false)
    }

    // This test should panic, but doesn't => it fails
    #[test]
    #[should_panic]
    fn it_fails3() {}

    // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    #[test]
    #[timeout(10)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
    ***/
}
