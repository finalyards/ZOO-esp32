# `v53l5cx_uld`

The `uld` part for the VL53L5CX time-of-flight sensor takes care of C/Rust adaptation and some translation of the results data formats (e.g. matrices instead of a vector).

YOU SHOULD NOT USE THIS LEVEL IN AN APPLICATION. Use the [`vl53l5cx`](../vl53l5cx.md) API, instead (which depends on this ULD level). Before that, though, read on, provide the vendor C API as instructed below, and make sure this stuff builds.


## Pre-reading

- ["Using C Libraries in Rust"](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a) (blog, Aug '19)

   A bit old, but relevant (C API's don't age!).
   
## The build

![](.images/build-map.png)

This build is relatively complex. You can just follow the instructions below, but in case there are problems, the above map may be of help.

<!-- tbd. `examples/common.rs` might get replaced by `../vl53l5cx/src/platform.rs` - and instead of examples there might be just tests. 
-->

## Requirements

### `clang`

```
$ sudo apt install llvm-dev libclang-dev clang
```

### `bindgen`

```
$ cargo install bindgen-cli
```

<!-- author's note:
`bindgen` is available also via `apt`, but the version seems to lag behind (perhaps is special for the Linux kernel use; don't know). At the time, `cargo install` is 0.70.1 while `apt show bindgen` gives:
>Version: 0.66.1-4
-->

>Note: Bindgen docs recommend using it as a library, but we prefer to use it as a command line tool.

### The vendor C libary

The `VL53L5CX_ULD_API` (ULD C driver) is a separate download.

1. [Fetch it](https://www.st.com/en/embedded-software/stsw-img023.html) from the vendor
2. Unzip it to a suitable location
3. `export VL53L5CX_ULD_API={your-path}/VL53L5CX_ULD_API`

	Note: You can download the driver "as guest".


### Supported dev kits

The workflow has been tested on these MCUs:

|||
|---|---|
|`esp32c6`|[ESP32-C6-DevKitM-01](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)|
|`esp32c3`|[ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) with JTAG/USB wiring added|

ESP32-C3 has problems with long I2C transfers, in combination with the `probe-rs` tool. Sadly, we cannot really recommend using it. See  [`../../TROUBLES.md`](../../TROUBLES.md) for details.

## Wiring

See [`../vl53l5cx/WIRING.md`](../vl53l5cx/WIRING.md).


## Compiling 

```
$ cargo build --release --lib
```

This compiles the library, and is a good place to start. 

>One thing to note about the library is that it's fully hardware agnostic; this is something we inherit from the approach of the vendor ULD C API. *Your code* (represented by `examples/` in this repo) brings in, for example, how to drive the I2C bus. This means only `examples` is MCU specific.

<span />

>If there are problems with the build, you may want to run the Makefile separately:
>
>```
>$ make manual
>```

<!-- disabled; going for tests
Likely you are more interested in the runnable samples, though. Let's have a look!
-->


## Running tests

If you have a devkit available, showing with `probe-rs list`, and at least one SATEL board wired to it (according to the instructions in `../vl53l5cx/README.md`, you're ready to run tests!

The point of the tests is to show that the basics (interaction from an app-side `Platform`, via the vendor ULD C API, through to the actual sensor) work. For application level examples, see the `../vl53l5cx` project.

```
$ probe-rs list
The following debug probes were found:
[0]: ESP JTAG -- 303a:1001:54:32:04:41:7D:60 (EspJtag)
```

```
$ cargo test
...
```


<!-- R #later
## Running samples

Running a sample expects that you have a device accessible via `probe-rs`:

```
$ probe-rs list
The following debug probes were found:
[0]: ESP JTAG -- 303a:1001:54:32:04:41:7D:60 (EspJtag)
```

```
$ cargo run --release --features=targets_per_zone_2,ambient_per_spad,nb_spads_enabled,signal_per_spad,range_sigma_mm,distance_mm,reflectance_percent,defmt --example m3
[...]
      Erasing ✔ [00:00:02] [######################] 256.00 KiB/256.00 KiB @ 98.28 KiB/s (eta 0s )
  Programming ✔ [00:00:47] [######################] 104.38 KiB/104.38 KiB @ 2.21 KiB/s (eta 0s )    
  Finished in 47.16089s
0.848963 [INFO ] Target powered off and on again.
0.852650 [DEBUG] Ping succeeded: 0xf0,0x02
3.621574 [INFO ] Init succeeded, driver version VL53L5CX_2.0.0
4.007674 [INFO ] Data #0 (sensor 36°C)
...
5.386734 [INFO ] Data #9 (sensor 35°C)
5.386766 [INFO ] .target_status:    [[[Valid(5), Valid(5), Valid(5), Valid(5)], [Valid(5), Valid(5), Valid(5), Valid(5)], [Valid(5), Valid(5), Valid(5), Valid(5)], [Valid(5), Valid(5), Valid(5), Valid(5)]], [[Valid(5), Other(4), Other(0), Other(0)], [Other(0), Other(4), Other(4), Other(0)], [Other(0), Other(2), Other(0), Other(4)], [Other(0), Other(0), Other(4), Other(0)]]]
5.386989 [INFO ] .targets_detected: [[2, 1, 1, 1], [1, 1, 2, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
5.387078 [INFO ] .ambient_per_spad: [[1, 0, 0, 1], [1, 2, 1, 0], [1, 2, 1, 1], [1, 1, 1, 1]]
5.387188 [INFO ] .spads_enabled:    [[16128, 15872, 15104, 15872], [15104, 15104, 15872, 14848], [15616, 14848, 15616, 15104], [15360, 15360, 15872, 15360]]
5.387302 [INFO ] .signal_per_spad:  [[[3, 15, 13, 13], [18, 17, 14, 12], [18, 16, 16, 11], [18, 15, 14, 13]], [[11, 11, 0, 0], [0, 2, 8, 0], [0, 2, 0, 2], [0, 0, 2, 0]]]
5.387497 [INFO ] .range_sigma_mm:   [[[28, 8, 8, 10], [7, 6, 7, 10], [7, 6, 8, 12], [6, 7, 8, 12]], [[15, 9, 0, 0], [0, 39, 7, 0], [0, 52, 0, 47], [0, 0, 46, 0]]]
5.387656 [INFO ] .distance_mm:      [[[1217, 1711, 1858, 1946], [1746, 1804, 1837, 1908], [1747, 1768, 1841, 1894], [1736, 1778, 1852, 1911]], [[1542, 1653, 0, 0], [0, 726, 1963, 0], [0, 4042, 0, 524], [0, 0, 2247, 0]]]
5.387846 [INFO ] .reflectance:      [[[8, 65, 66, 74], [79, 80, 70, 66], [80, 72, 78, 57], [79, 70, 67, 70]], [[37, 45, 0, 0], [0, 2, 43, 0], [0, 57, 0, 1], [0, 0, 17, 0]]]
5.387989 [INFO ] End of ULD demo
```

That's a bunch of features!!!

You can steer the `m3` example's behaviour by the set of features you define. Equally, the `targets_per_zone_2` defines the "depth" of possibly separate targets, per zone, that the sensor will report.

Note how this change of behaviour of the code is steered by the Cargo `feature` system. It goes so deep that the *underlying ULD C API* gets compiled differently, omitting the features you don't state you are needing. This reduces both the code size and the data transferred from the sensor over I2C.

By leaving the `targets_per_zone_2` feature out, you will get only one data per zone (closest or strongest, depending on the source code; which seems to be favouring `CLOSEST`).
-->


## Troubleshooting

### [ESP32-C3] I2C `TimeOut`

```
0.956520 [INFO ] Target powered off and on again.
0.960236 [DEBUG] Ping succeeded: 0xf0,0x02
1.522238 [ERROR] panicked at 'I2C write to 0x0bd0 (252 bytes) failed: TimeOut'
1.522361 [ERROR] ====================== PANIC ======================
```

This happens with latest versions of `probe-rs`.

- the problem is [wont-fix](https://github.com/probe-rs/probe-rs/issues/2818#issuecomment-2358791448), unless they get news from Espressif

If you need to work on ESP32-C3, you can install commit `6fee4b6` of `probe-rs`. That should work, but you won't get updates to the tool.

>More details in -> [`../../TROUBLES.md`](../../TROUBLES.md).


<!-- Q: is this relevant with tests?  Perhaps move to ../vl53l5cx/ ?
### [ESP32-C3] No `defmt` output

```
$ probe-rs run --log-format '{t:dimmed} [{L:bold}] {s}' target/riscv32imc-unknown-none-elf/release/examples/multiboard
      Erasing ✔ [00:00:02] [################################] 256.00 KiB/256.00 KiB @ 112.53 KiB/s (eta 0s )
  Programming ✔ [00:00:33] [################################] 105.54 KiB/105.54 KiB @ 3.13 KiB/s (eta 0s )
    Finished in 33.767773s



```

This sometimes happens. Something is confused. This seems to resolve the situation:

- detach the USB cable of the device
- attach back
- `usbip attach -r ...` (if you are using USB/IP)
- try again
-->
	
## References

### VL53L5CX

- [Breakout Boards for VL53L5CX](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) (ST.com)
- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)

	- ["Ultra lite driver (ULD) [...] with wide field of view"](https://www.st.com/resource/en/data_brief/stsw-img023.pdf) (PDF, May'21; 3pp)
	- ["A guide to using the VL53L5CX multizone [...]"](https://www.st.com/resource/en/user_manual/um2884-a-guide-to-using-the-vl53l5cx-multizone-timeofflight-ranging-sensor-with-a-wide-field-of-view-ultra-lite-driver-uld-stmicroelectronics.pdf) (PDF, revised Feb'24; 18pp)

- [VL53L5CX Product overview](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) (ST.com DS13754, Rev 12; April 2024)

### SATEL

- [How to setup and run the VL53L5CX-SATEL using an STM32 Nucleo64 board]() (ST.com AN5717, Rev 2; Dec 2021)
- [PCB Schematic VL53L5CX-SATEL](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html#cad-resources) (ST.com; Rev A, ver 012, 2021)
