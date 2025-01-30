# Troubleshooting

## Dev board not showing in `defmt` logs

Occasionally, they need resetting to a "download state". Do this by:

- Press the `BOOT` button 
- Press and release `RESET`
- Release `BOOT`

<!-- #hidden; does WSL2 need the reconnect?
- *// if using using USB/IP, reconnect with e.g. `sudo usbip attach -r 192.168.1.29 -b 3-1`*
-->

The logs should now show.

Sometimes, one also needs to detach the USB cable physically.

<!-- #whisper
This has been observed on both ESP32-C3 and ESP32-C6 devkits.
-->

## [ESP32-C3] Problem with certain time critical hardware situations, and the JTAG interface, and `defmt` / other RTT 😊

The development tools we use (`probe-rs` and `defmt`) bring up a tooling issue that affects ESP32-C3 (but not e.g. ESP32-C6).

There's a specific case where ESP32-C3 seems to work against its spec, and while `probe-rs` had a fix for this, it won't be kept around. [Details here](https://github.com/probe-rs/probe-rs/issues/2818#issuecomment-2358791448). That is quite fair.

The problem only occurs when logging via `defmt`/`probe-rs`, and I2C communication is active. In that case it's a bit random, but the 80k firmware upload of the VL53L5CX driver (`vl53l5cx_uld` folder) never succeeds.

Ways to circumvent this:

- use another MCU type for projects, where this is a problem, e.g. ESP32-C6
