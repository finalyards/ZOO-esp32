### Flasher: `espflash` or `probe-rs`

We use [defmt](https://docs.rs/defmt/latest/defmt/) for logging and there are two different flashing/monitoring ecosystems that are compatible with it:

- [`espflash`](https://github.com/esp-rs/espflash) from Espressif
- [`probe-rs`](https://github.com/probe-rs/probe-rs) which is multi-target (ARM, RISC-V)

Both of these can flash software onto your device and monitor its running. They work using very different internal approaches, and which one to choose is mostly a matter of choice.

||`espflash`|`probe-rs`|
|---|---|---|
|USB port|any: UART or JTAG|**JTAG only**|
|background / author(s)|Espressif|multi-vendor|
|line format customization|yes, since version 4.0|yes, with `--log-format`|
|`semihosting::process::exit` compatible|not (v4-dev)|yes|
|use when...|needing to support ESP32-C3|you have USB/JTAG port (ESP32-C6)|

>[! NOTE]
>The selection of flasher only affects running examples, not how the `vl53l5cx_uld` can be used as a library.

Once you have a hunch, which flasher you'll use, check that it can reach your devkit:

<details><summary>`probe-rs`</summary>

```
$ probe-rs list
The following debug probes were found:
[0]: ESP JTAG -- 303a:1001:54:32:04:07:15:10 (EspJtag)
```
</details>

<details><summary>`espflash`</summary>

```
$ espflash board-info
[2025-03-11T16:22:04Z INFO ] Serial port: '/dev/ttyUSB0'
[2025-03-11T16:22:04Z INFO ] Connecting...
[2025-03-11T16:22:04Z INFO ] Using flash stub
Chip type:         esp32c6 (revision v0.0)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       54:32:04:07:15:10
```

<!-- #hidden
>[! NOTE]
>Since `espflash` 4.0, both tools can use the same output formatting. We utilize this. If, however, you have `espflash` 3.3 (and are not willing to update), change:
>
>- `examples/m3.rs`: comment out the line `init_defmt();`
>- `Makefile.dev`: remove `--output-format $(DEFMT_HOST_FMT)` from the targets having to do with `espflash`.
-->
</details>
