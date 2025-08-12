# `v53l5cx_uld`

The `uld` part for the VL53L5CX time-of-flight sensor takes care of

- C/Rust adaptation
- translation of results from 1D vectors to 2D matrices
- enums in place of "magic" integer values

YOU SHOULD NOT USE THIS LEVEL IN AN APPLICATION. Use the [`vl53l5cx`](../vl53l5cx/README.md) API instead (which depends on us). Before that, though, read on, install the build requirements so that the higher API can also be built.

>Note: We don't automatically pull in the vendor ULD C library, because it requires a "click through" license. @ST.com if you are reading this, please consider providing a publicly accessible URL to remove this somewhat unnecessary manual step developers need to go through.


## Pre-reading

- ["Using C Libraries in Rust"](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a) (blog, Aug '19)

   A bit old, but relevant (C API's don't age!).
   
## The build

![](.images/build-map.png)

This build is relatively complex. You can just follow the instructions below, but in case there are problems, the above map may be of help.

## Requirements

### `clang`

```
$ sudo apt install libclang-dev clang
```

### `bindgen`

```
$ cargo install bindgen-cli
```

<!-- author's note:
`bindgen` is available also via `apt`, but the version seems to lag behind (perhaps is special for the Linux kernel use; don't know). At the time, `cargo install` is 0.71.1 while `apt show bindgen` gives:
>Version: 0.66.1-4
-->

>Note: Bindgen docs recommend using it as a library, but we prefer to use it as a command line tool.

<!-- Developed with:
$ clang --version
Ubuntu clang version 18.1.3 (1ubuntu1)
[...]

$ bindgen --version
bindgen 0.71.1
-->

### The vendor C libary

The `VL53L5CX_ULD_API` (ULD C driver) is a separate download.

1. [Fetch it](https://www.st.com/en/embedded-software/stsw-img023.html) from the vendor (`Get software` > `Get latest` > check the license > ...)

	>Note: You can `"Download as a guest"`, after clicking the license.

2. Unzip it to a suitable location
3. `export VL53L5CX_ULD_API={your-path}/VL53L5CX_ULD_API`


### Supported dev kits

The workflow has been tested on these MCUs:

|||
|---|---|
|`esp32c6`|[ESP32-C6-DevKitM-01](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)|
|`esp32c3`|[ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html)|

<!-- #hidden
|`esp32c3`|[ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) with JTAG/USB wiring added<p />*❗️ESP32-C3 has problems with long I2C transfers, in combination with the `probe-rs` tool. Sadly, we cannot really recommend using it. See  [`../../TROUBLES.md`](../../TROUBLES.md) for details.*|
-->

### Flasher: `espflash` or `probe-rs`

We use [defmt](https://docs.rs/defmt/latest/defmt/) for logging and there are two different flashing/monitoring ecosystems that are compatible with it:

- [`espflash`](https://github.com/esp-rs/espflash) from Espressif
- [`probe-rs`](https://github.com/probe-rs/probe-rs) which is multi-target (ARM, RISC-V)

Both of these can flash software onto your device and monitor its running. They work using very different internal approaches, and which one to choose is mostly a matter of choice.

||`espflash`|`probe-rs`|
|---|---|---|
|USB port|any: UART or JTAG|**JTAG only**|
|line format customization|no|yes, with `--log-format`|
|background / author(s)|Espressif|multi-vendor|
|use when...|needing to support ESP32-C3|you have USB/JTAG connector available|

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

>[! NOTE]
>Since `espflash` 4.0, both tools can use the same output formatting. We utilize this. If, however, you have `espflash` 3.3 (and are not willing to update), change:
>
>- `examples/m3.rs`: comment out the line `init_defmt();`
>- `Makefile.dev`: remove `--output-format $(DEFMT_HOST_FMT)` from the targets having to do with `espflash`.
>
>Currently (Apr'25), installing `espflash` 4.0 means you will need to clone the sources and build it from there. <!--tbd. track https://github.com/esp-rs/espflash/releases -->
</details>

### SATEL board

One [SATEL board](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) is needed. 

For wiring, see [`pins.toml`](./pins.toml):

```
[boards.esp32c3]
SDA = 4
SCL = 5
PWR_EN = 6
INT=7

[boards.esp32c6]
SDA = 18
SCL = 19
PWR_EN = 21
INT = 22
```


## Running examples

Test the code with:

```
$ make -f Makefile.dev m3
[...]
0.870700 [INFO ] Target powered off and on again.
0.874266 [DEBUG] Ping succeeded: 0xf0,0x02
3.639815 [INFO ] Init succeeded
4.008711 [DEBUG] INT after: 24.442ms
4.024860 [INFO ] Data #0 (32°C)
4.024911 [INFO ] .target_status:    [[[SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)], [SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)], [SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)], [SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)]], [[SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)], [SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)], [SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)], [SemiValid(6), SemiValid(6), SemiValid(6), SemiValid(6)]]]
4.025215 [INFO ] .targets_detected: [[2, 2, 2, 2], [2, 2, 2, 2], [2, 2, 2, 2], [2, 2, 2, 2]]
4.025322 [INFO ] .ambient_per_spad: [[1, 1, 1, 2], [1, 2, 1, 0], [1, 1, 1, 1], [0, 0, 1, 1]]
4.025446 [INFO ] .spads_enabled:    [[16128, 15872, 15104, 15872], [15104, 15104, 15872, 12800], [15616, 14848, 15616, 11264], [15360, 15360, 15872, 10240]]
4.025566 [INFO ] .signal_per_spad:  [[[137, 144, 222, 345], [154, 92, 168, 325], [120, 105, 204, 415], [112, 165, 262, 572]], [[122, 34, 26, 16], [148, 20, 12, 10], [83, 6, 16, 11], [28, 22, 26, 12]]]
4.025800 [INFO ] .range_sigma_mm:   [[[3, 2, 1, 1], [4, 3, 2, 1], [4, 3, 1, 1], [2, 2, 1, 1]], [[3, 5, 7, 9], [2, 12, 17, 12], [6, 28, 8, 12], [8, 8, 6, 13]]]
4.025994 [INFO ] .distance_mm:      [[[38, 0, 1, 0], [142, 11, 0, 0], [73, 7, 0, 0], [0, 0, 0, 0]], [[300, 202, 907, 933], [253, 1043, 808, 646], [220, 642, 708, 724], [393, 606, 642, 653]]]
4.026182 [INFO ] .reflectance:      [[[0, 0, 0, 0], [4, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]], [[15, 2, 30, 19], [13, 31, 11, 6], [5, 3, 12, 8], [6, 11, 15, 8]]]
4.069097 [DEBUG] INT after: 42.756ms

```

If you have an ESP32-C3 board, this will fail. Use `make -f Makefile.dev m3-with-espflash`, instead.


## Troubleshooting

See [`TROUBLES.md`](./TROUBLES.md).


## Power budget

You can connect multiple SATEL boards to the same MCU, via the I2C bus.

Let's see, how many can be powered without an external power source.

**Facts**

||mA|V|mW|
|---|---|---|---|
|**Drains**|
|- ESP32-C6-DevKitM1|38mA (radio off) .. 189mA (BLE sending at 9.0 dBm)<sup>`|1|`</sup>|3.3|124 .. 624|
|- SATEL board|130mA (AVDD + IOVDD; 50 + 80)|3.3|429|
|- external pull-ups|2k2: 1.5mA x 2 = 3mA|3.3|10|
|**Source**|
|- Power available from a USB 2.0 port (Raspberry Pi 3B)|500mA|5|2500|

For four SATEL boards:

- no radio: 124 + 4*429 + 10 = 1850 mW < 2500 mW
- BLE @ 9.0 dBm: "no radio" + 500 = 2350 mW < 2500 mW

This seems to indicate we should be able to use four SATEL boards, and broadcast on BLE, while staying within the USB 2.0 port's power budget.

>Note: The above calculation is based on peak values. ST.com datasheet itself marks "continuous mode" actual consumption as 313mW, and by using "autonomous mode" (as we do in the code), the value drops below 200 (depends on the scanning frequency). This means powering even up to eight boards could be a thing, from a single USB 2.0 port. It would be best to measure the actual power use.

<small>
`|1|`: [ESP32-C6-MINI-1 & MINI-1U Datasheet v1.2](https://www.espressif.com/sites/default/files/documentation/esp32-c6-mini-1_mini-1u_datasheet_en.pdf) > 5.4 "Current consumption characteristics" <br />
`|2|`: [VL53L5CX Datasheet: DS13754 - Rev 13](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) > 6.4 "Current consumption"
</small>


	
## References

### VL53L5CX

- [Breakout Boards for VL53L5CX](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) (ST.com)
- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)

	- ["Ultra lite driver (ULD) [...] with wide field of view"](https://www.st.com/resource/en/data_brief/stsw-img023.pdf) (PDF, May'21; 3pp)
	- ["A guide to using the VL53L5CX multizone [...]"](https://www.st.com/resource/en/user_manual/um2884-a-guide-to-using-the-vl53l5cx-multizone-timeofflight-ranging-sensor-with-a-wide-field-of-view-ultra-lite-driver-uld-stmicroelectronics.pdf) (PDF, revised Feb'24; 18pp)

- [VL53L5CX Product overview](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) (ST.com DS13754, Rev 12; April 2024)

### SATEL

- [How to setup and run the VL53L5CX-SATEL using an STM32 Nucleo64 board]() (ST.com AN5717, Rev 2; Dec 2021)
- [PCB4109A, version 12, variant 00B](https://www.st.com/resource/en/schematic_pack/pcb4109a-00b-sch012.pdf) (ST.com; 2021; PDF 2pp.)

	>*Interestingly, marked `CONFIDENTIAL` but behind a public web link.*
