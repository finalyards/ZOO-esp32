/*
* Accessing the devkit BOOT button (attached to GPIO9; active low).
*/
use esp_hal::{
    delay::Delay,
    prelude::*,
    time::now
};

struct BootButton {

}

impl BootButton {

    fn new() -> Self {
        unimplemented!()
        //Self {}
    }

    async fn wait_until(state: bool) {
        unimplemented!();
    }

    pub async fn wait_until_pressed(&self) {
        self.wait_until(false).await;
    }
    pub async fn wait_until_depressed(&self) {
        self.wait_until(true).await;
    }
}
