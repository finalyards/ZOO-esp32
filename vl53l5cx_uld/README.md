# `v53l5cx_uld`

Turns the `VL53L5CX_ULD_API` source code into something that can be touched with Rust.

## Pre-reading

- ["Using C Libraries in Rust"](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a) (blog, Aug '19)

   A bit old, but relevant (C API's don't age!).
   
## The job

![](.images/bindgen-jumps.png)

>*tbd. Update the image.*


## Requirements

See hardware and software requirements -> [`../README`](../README.md).

## Preparation

The workflow has been tested on these MCUs:

|||
|---|---|
|`esp32c3` (default)|[ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html) with JTAG/USB wiring added|
|`esp32c6`|[ESP32-C6-DevKitM-01](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)|

If you are using ESP32-C3, the repo is ready for use. To use ESP32-C6, run this once: `./set-target.sh` .

>Note: It's usual that embedded Rust repos use the `feature` system (in `Cargo.toml`) for selecting hardware. This repo doesn't. Instead, it keeps the hardware choice as a "separate axis" from the features of the library, and examples. Follow the guidance; if you don't know the context, it doesn't matter the slightest.

### Wiring

See [WIRING](./WIRING.md) for how you are expected to wire a single SATEL board to your MCU.

If you prefer another kind of pin layout, have a look at `pins.toml`.

### Using multiple boards

See [Working with multiple boards](./Working%20with%20multiple%20boards.md) if you intend to have multiple sensors. There are some hoops involved in that setup.


## Compiling 

```
$ cargo build --release --lib
```

This compiles the library, and is a good place to start. 

>One thing to note about the library is that it's fully hardware agnostic; this is something we inherit from the approach of the vendor ULD C API. *Your code* (represented by `examples/` in this repo) brings in, for example, how to drive the I2C bus.

<span />

>The command uses `Makefile` internally. If there are problems with the build, you may want to run the Makefile separately (see below).
>
>```
>$ make src/pins.in src/uld_raw.rs tmp/libvendor_uld.a
>```

Likely you are more interested in the runnable samples, though. Let's have a look!

## Running samples

Running a sample expects that you have a device accessible via `probe-rs`:

```
$ probe-rs list
The following debug probes were found:
[0]: ESP JTAG -- 303a:1001:54:32:04:41:7D:60 (EspJtag)
```

>Note! For now (until 0.24.1 is out), you are recommended to run a version of `probe-rs`, from the git `main` branch.

```
$ cargo run --release --features=targets_per_zone_2,ambient_per_spad,nb_spads_enabled,signal_per_spad,range_sigma_mm,distance_mm,reflectance_percent,defmt --example m2
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

You can steer the `m2` example's behaviour by the set of features you define. Equally, the `targets_per_zone_2` defines the "depth" of possibly separate targets, per zone, that the sensor will report.

>Playing with these will be more fun once we get graphical tools to show the data, instead of matrices.

Note how this change of behaviour of the code is steered by the Cargo `feature` system. It goes so deep that the *underlying ULD C API* gets compiled differently, omitting the features you don't state you are needing.

By leaving the `targets_per_zone_2` feature out, you will get only one data per zone (closest or strongest, depending on the source code; which seems to be favouring `CLOSEST`).


## Graphical tooling

*tbd.*

## Troubleshooting

### General

Make sure you've installed `probe-rs` from GitHub.

### No log output (ESP32-C3 only)

```
$ probe-rs run --log-format [...] target/riscv32imc-unknown-none-elf/release/examples/m2
      Erasing ✔ [00:00:03] [################] 192.00 KiB/192.00 KiB @ 57.05 KiB/s (eta 0s )
  Programming ✔ [00:00:18] [################] 22.54 KiB/22.54 KiB @ 1.25 KiB/s (eta 0s )    Finished in 18.072815s



```

The C3 board *sometimes* needs a physical reset (or even reattaching to the USB), to recover. This should not be frequent, but if you have problems, try:

- keep both `RESET` and `BOOT` buttons pressed; release `RESET`; then release `BOOT`. This sets the device in "download mode".

Other things to try:

- just press the `RESET` button
- detach the USB cable; re-attach 

## References

- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)


