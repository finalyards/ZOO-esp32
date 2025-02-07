/*
* Fake the 'esp-hal-next' 'Instant' API, to keep main code simpler.
*/
#[cfg(feature = "esp-hal-next")]
compile_error!("only for released <= 0.23.1");

use esp_hal::time::{
    now as esp_hal_now,
    //self
};

pub struct Instant(esp_hal::time::Instant);

impl Instant {
    pub fn now() -> Self {
        Self( esp_hal_now() )
    }

    pub fn elapsed(&self) -> Elapsed {
        // could be more straightforward, but..
        let dt: u64 = esp_hal_now().checked_duration_since(self.0).unwrap().to_millis();
        Elapsed::from_millis(dt)
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Elapsed(esp_hal::time::Duration);

impl Elapsed {
    fn from_millis(ms: u64) -> Self {
        Self( esp_hal::time::Duration::millis(ms))
    }
    pub fn as_millis(&self) -> u64 {
        self.0.to_millis()
    }
}
