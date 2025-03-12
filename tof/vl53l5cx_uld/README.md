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

<!-- tbd. update the image-->

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
|`esp32c3`|[ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html)|
|`esp32c6`|[ESP32-C6-DevKitM-01](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html) ❗️ Currently broken!!!|

<!-- TBD. FIX IT
>NOTE! The author is struggling with I2C access on the `esp-hal` library / and/or the sensors. See [summary in a MRE repo](https://github.com/lure23/vl53l5-c2rust.fork?tab=readme-ov-file#summary). TL;DR: C3 with `esp-println` works; C6 or `defmt-rtt` logging doesn't. THE AUTHOR WOULD REALLY, REALLY LIKE TO HAVE ALL THE COMBINATIONS WORK IN A STABLE MANNER.
-->

<!-- #hidden
|`esp32c3`|[ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) with JTAG/USB wiring added<p />*❗️ESP32-C3 has problems with long I2C transfers, in combination with the `probe-rs` tool. Sadly, we cannot really recommend using it. See  [`../../TROUBLES.md`](../../TROUBLES.md) for details.*|
-->

### Supported flashers

You can run the example/tests either with [`espflash`](https://github.com/esp-rs/espflash) or [`probe-rs`](https://github.com/probe-rs/probe-rs). 

Both of these tools can flash a software on your device and monitor its running, but they work using very different internal approaches.

||example `make` target|USB port(s)|use when...|
|---|---|---|---|
|`espflash`|`m3-with-espflash`|any: UART or JTAG|
|`probe-rs`|`m3-with-probe-rs`|JTAG only|

*tbd. write when to use one over the other*

>[! NOTE]
>The selection of flasher only affects running examples, not how the `vl53l5cx_uld` can be used as a library.

Once you have a hunch, which flasher you'll use, check that it can reach your devkit:

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
</details>

<details><summary>`probe-rs`</summary>

```
$ probe-rs list
The following debug probes were found:
[0]: ESP JTAG -- 303a:1001:54:32:04:07:15:10 (EspJtag)
```
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
PWR_EN = 22
INT = 23
```


## Running examples

There are some ULD level examples used to help development:

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

## Troubleshooting

See [`TROUBLES.md`](./TROUBLES.md).

	
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
