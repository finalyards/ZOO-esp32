#![no_std]
#![no_main]

/*
* Test assumptions on how the VL... board (SATEL in particular) would function.
*/
use defmt_rtt as _;

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use esp_hal::delay::Delay;
    use crate::init_defmt;

    // Download the firmware
    #[init]
    fn init() -> Delay {
        init_defmt();   // tbd. ideally, we'd call it only "once".

        //let peripherals = esp_hal::init(esp_hal::Config::default());
        let delay = Delay::new();

        // returned state can be consumed by the test cases
        delay
    }

    // VL modules don’t retain their I2C address over reboot.
    #[test]
    fn addr_persistence_test() {

        assert!(false)
    }
}

fn init_defmt() {
    use esp_hal::time::now;
    defmt::timestamp!("{=u64:us}", { now().duration_since_epoch().to_micros() });
}
