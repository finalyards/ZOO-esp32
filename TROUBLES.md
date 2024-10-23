# Troubleshooting

## ESP32-C3 dev board not showing `defmt` logs

Occasionally, they need resetting to a "download state". Do this by:

- Press both `RESET` and `BOOT` buttons
- Release `RESET`
- Release `BOOT`
- reconnect using USB/IP: e.g. `sudo usbip attach -r 192.168.1.29 -b 3-1`
- try flashing and running

The logs should now show.

>Note: This is not an issue on the ESP32-C6 board that the author has.

Sometimes, one also needs to detach the USB cable physically.

## [ESP32-C3] Use a specific `probe-rs` version

There's a specific case where ESP32-C3 seems to work against its spec, and while `probe-rs` had a fix for this, it won't be kept around. [Details here](https://github.com/probe-rs/probe-rs/issues/2818#issuecomment-2358791448). That is quite fair.

The problem only occurs when logging via `defmt`/`probe-rs`, and I2C communication is active. In that case it's a bit random, but the 80k firmware upload of the ULD driver (`vl53l5cx_uld` specific) never succeeds.

Ways to circumvent this:

- launch the code using e.g. `espflash` (not included in the `mp` setup), ignoring `defmt` logging
- use a specific revision that has the (now removed) hacks for C3:

   ```
   $ cargo install --git https://github.com/probe-rs/probe-rs --rev 6fee4b6 probe-rs-tools --locked --force
   ```

- use another MCU type
