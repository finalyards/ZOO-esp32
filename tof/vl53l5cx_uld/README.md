# `v53l5cx_uld`

The `uld` part for the VL53L5CX time-of-flight sensor takes care of C/Rust adaptation and translation of the results from 1D vectors to 2D matrices, and enums in favor of integer values.

YOU SHOULD NOT USE THIS LEVEL IN AN APPLICATION. Use the [`vl53l5cx`](../vl53l5cx.md) API instead (which depends on us). Before that, though, read on, install the build requirements so that the higher API can build this, as a dependency.


## Pre-reading

- ["Using C Libraries in Rust"](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a) (blog, Aug '19)

   A bit old, but relevant (C API's don't age!).
   
## The build

![](.images/build-map.png)

This build is relatively complex. You can just follow the instructions below, but in case there are problems, the above map may be of help.

<!-- tbd. `examples/common.rs` might get replaced by `../vl53l5cx/src/platform.rs`.
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

Same as in [`../vl53l5cx/WIRING.md`](../vl53l5cx/WIRING.md).

Only one device is needed.


## Compiling 

```
$ cargo build --release --lib
```

This compiles the library, and is a good place to start. 

>One thing to note about the library is that it's fully hardware agnostic; this is something we inherit from the approach of the vendor ULD C API. *Your code* brings in, for example, how to drive the I2C bus. This means only `tests` is MCU specific.

<span />

>If there are problems with the build, you may want to run the Makefile separately:
>
>```
>$ make manual
>```

<!-- disabled; going for tests
Likely you are more interested in the runnable samples, though. Let's have a look!
-->

<!-- tbd. should we have examples???
## Running examples

Example are explorational code checking some driver / hardware interactions. They are run manually, one at a sime.
-->

## Running tests

The tests are used to establish certain presumptions about how the hardware works. In order to run them, you need:

- a devkit connected:

	```
	$ probe-rs list
	The following debug probes were found:
	[0]: ESP JTAG -- 303a:1001:54:32:04:41:7D:60 (EspJtag)
	```

- at least one SATEL board wired to it (according to the instructions in `../vl53l5cx/README.md`)

Then:

```
$ cargo test
...
test tests::time_stamp_test ... 1.094964 [INFO ] Running test: Test { name: "tests::time_stamp_test", function: 0x420012a0, should_panic: false, ignored: false, timeout: None }  embedded_test src/fmt.rs:36
1.095982 [INFO ] Test exited with () or Ok(..)  embedded_test src/fmt.rs:36
ok
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

### `VL53L5CX_Configuration` size dispute

>This *is* a bug of the build system, but happens rather rarely and therefore the author hasn't been able to fix it. Please try! :)

```
$ make -f Makefile.dev m3
EXAMPLE=m3 \
  FEATURES=targets_per_zone_2,ambient_per_spad,nb_spads_enabled,signal_per_spad,range_sigma_mm,distance_mm,reflectance_percent \
  make -f Makefile.dev --no-print-directory _build _run
DEFMT_LOG=debug cargo build --release --features=targets_per_zone_2,ambient_per_spad,nb_spads_enabled,signal_per_spad,range_sigma_mm,distance_mm,reflectance_percent,defmt --example m3
   [...]
error[E0080]: evaluation of constant value failed
  --> src/uld_raw.rs:38:10
   |
38 |         [::core::mem::size_of::<VL53L5CX_Configuration>() - 3248usize];
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `3240_usize - 3248_usize`, which would overflow

For more information about this error, try `rustc --explain E0080`.
error: could not compile `vl53l5cx_uld` (lib) due to 1 previous error
[...]
```

The reason is something's gotten confused with the `tmp/config.h[.next]` files. Remove them, and retry the build:

```
$ rm tmp/config.h*
$ cargo build		# might be neede
$ make manual
$ make -f Makefile.dev m3
```

That should fix it.

	
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
