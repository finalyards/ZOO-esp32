/*
* Formulated by vendor sample 'Example 1'
*/
use embassy_executor::Spawner;
use esp_hal::macros::main;
use vl53l5cx::{VL53L5CX, Result};

use defmt::info;
use defmt_rtt as _;

async fn example1() -> Result {
    let addr: u8 = 0;
    let dev: VL53L5CX = VL53L5CX::new();     // tbd. provide the pin (or lack thereof) where INT is routed
                                            // - provide I2C address (unless default); and other config

    assert!( dev.is_alive(), "Device not detected at address {}", addr);

    let ver = dev.get_info().API_REVISION;
    info!("ULD API revision: {}", ver);

    let h = dev.start_ranging();
    for _count in 0..10 {

        // If INT connected
        let data = h.get_data() .await;

        info!("Data no: {}", unimplemented!());      // 'Dev.streamcount' in C

        // We only print one (strongest/closest, based on config) target, per zone.
        for i in 0..16 {
            let status = data.target_status[i][0];
            let dist = data.distance_mm[i][0];
            info!("Zone: {}, Status: {}, Distance: {}mm", i, status, dist )
        }
    }
    //h.stop();   // can be omitted; dropping suffices
}

#[main]
async fn main(spawner: Spawner) {
    example1().await
}