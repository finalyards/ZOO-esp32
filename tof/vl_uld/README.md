# `vl_uld`

The `uld` part for the VL53 time-of-flight sensors takes care of:

- C/Rust adaptation
- translation of results from 1D (vectors) to 2D (matrices)
- enums in place of "magic" integer values
- merging `.distance_mm`, `.target_status` and the target information (`.nb_target_detected`) together into an enum that is easier to deal with than three separate data sources
- enforce data validation, craft invariants

You should not need to use this level directly, in an application. Use the [`vl_api`](../vl_api/README.md) instead.

Please note that the library itself, like the underlying vendor C library, is *fully MCU agnostic*, i.e. you can use it as a building block for working with VL53 sensors, in Rust, on any MCU family. The examples and the `../vl_api` bring in ESP32 specific details.

## Overview

![](.images/build-map.png)

This build is quite complex. You can just follow the instructions below, but in case there are problems, the above map may be of help.

The `vl_api` only needs the "output lib". Once it is compiled, the vendor C library sources (`VL53L[58]CX_ULD_API`) do not need to be kept around.


## Compiling

```
$ cargo build --release --features=distance_mm,defmt,vl53l8cx
```

## Running examples

>[!NOTE]
>One would need to run examples mostly for development, or bug finding - e.g. if your electronics has issues it may be better to test things here than on the `vl_api` level.

### Required

- one SATEL board, either [SATEL-VL53L8](https://www.st.com/en/evaluation-tools/satel-vl53l8.html) or [VL53L5CX-SATEL](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html).
- ..wired according to [`../vl_api/WIRING`](../vl_api/WIRING.md).

	>*You don't need to wire the `LPn0` unless you plan on using multiple sensors. In this subfolder, we only use one.*

- `probe-rs` or `espflash` connection to your devkit

	>See [`../../README.md`](../../README.md) > `Requirements (hardware)`

<!--
Versions used in development:
	
```
$ probe-rs --version
probe-rs 0.29.1 (git commit: v0.29.0-26-g1cf182e)
```

```
$ espflash --version
espflash 4.0.1
```
-->

### Steps

Test the code with:

```
$ VARIANT=8 make -f Makefile.dev m3
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
[...]
Firmware exited successfully
```

>[!NOTE]
>If you have an ESP32-C3 board, you will need to use `espflash`. Try `VARIANT=8 make -f Makefile.dev m3-with-espflash`, instead.

	
## References

### VL53L5CX

- [Breakout Boards for VL53L5CX](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) (ST.com)
- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)

	- ["Ultra lite driver (ULD) [...] with wide field of view"](https://www.st.com/resource/en/data_brief/stsw-img023.pdf) (PDF, May'21; 3pp)
	- ["A guide to using the VL53L5CX multizone [...]"](https://www.st.com/resource/en/user_manual/um2884-a-guide-to-using-the-vl53l5cx-multizone-timeofflight-ranging-sensor-with-a-wide-field-of-view-ultra-lite-driver-uld-stmicroelectronics.pdf) (PDF, revised Feb'24; 18pp)

- [VL53L5CX Product overview](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) (ST.com DS13754, Rev 13; Sep 2024)

### SATEL

- [How to setup and run the VL53L5CX-SATEL using an STM32 Nucleo64 board](https://www.st.com/resource/en/application_note/an5717-how-to-setup-and-run-the-vl53l5cxsatelusing-an-stm32-nucleo64-board-stmicroelectronics.pdf) (ST.com AN5717, Rev 4; Dec 2024)

- [PCB4109A, version 12, variant 00B](https://community.st.com/ysqtg83639/attachments/ysqtg83639/imaging-sensors-forum/1559/1/PCB4109A-00B-SCH012.pdf) (ST.com; Apr 2021; PDF 2pp.)

<!-- earlier URL (now 404):
https://www.st.com/resource/en/schematic_pack/pcb4109a-00b-sch012.pdf
-->
