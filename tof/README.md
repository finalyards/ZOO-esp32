# `tof`

Support for using Time-of-Flight sensors (IR distance measurement), from Rust.

Following models are supported:

- [VL53L8CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l8cx.html)
- [VL53L5CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html)

You can use such sensors for:

- distance measurement (2 &#8229; 300cm, depending on reflectivity)
- movement and presence detection

The sensors provide 4x4 or 8x8 resolution, detecting the targets by local peaks in the reflected IR beams (see figure below).

>![](.images/multiple_targets.png)
>
><small>*Source: UM3109 - rev 11; page 12 (ST.com)*</small>

In this case, two targets are detected, one (strongest) at 70cm and a secondary (weaker) at 10cm.

>[!NOTE]
>
>In the above figure, the distance <u>conveniently happens</u> to be exactly 600mm. Turns out, this is the *minimum distance* between any two targets. This is one of the limitations that you should be aware of - unfortunately only marginally mentioned in one of the specs. Other limitations involve interplay of resolution modes vs. sampling rate vs. power consumption - and the role of reflectance **and ambient light** in signal strength and overall possibility of detecting stuff. The best spec numbers are taken in dark conditions; other parts of the spec do shed some light (literally) to these issues.

<!-- #hidden
We should take the time, some week, to study these features in depth and make e.g. an interactive web page that you can use to see, whether your case would fall within the possibilities of the sensor.
-->

## Folder structure

You'll need three libraries:

### [`vl_api`](vl_api/README.md) - Application level API

This level provides:

- an API that you can use in your own projects
- a "platform" adaptation for ESP32; i.e. depends on `esp-hal`
- support for handling multiple sensors
- Embassy `async` interfaces for ease of programming

This is the level you are expected to build your own applications on top.


### [`vl_uld`](vl_uld/README.md) - Lower level: Rust/C interface

ULD stands for ["Ultra Light Driver"](https://www.st.com/en/embedded-software/stsw-img023.html) - it's the vendor's terminology for their embedded C drivers (not Linux).

This library provides the Rust/C interfacing, and some other goodies for providing a higher abstraction level than what the C level does:

- matrices instead of linear vectors
- enums instead of magic integer values
- missing values marked, without need to cross-reference matrices (applies to reading multiple targets)

In order to build this library, you need to provide it a copy of the vendor's ULD library.

### `VL53L[58]CX_ULD_API`

This is the vendor's C library:

```
├── LICENSE.txt
├── inc
│   ├── vl53l8cx_api.h
│   ├── vl53l8cx_buffers.h
│   ├── vl53l8cx_plugin_detection_thresholds.h
│   ├── vl53l8cx_plugin_motion_indicator.h
│   └── vl53l8cx_plugin_xtalk.h
└── src
    ├── vl53l8cx_api.c
    ├── vl53l8cx_plugin_detection_thresholds.c
    ├── vl53l8cx_plugin_motion_indicator.c
    └── vl53l8cx_plugin_xtalk.c
```    

It's available via a click-through license from the vendor's site - you'll have the installation instructions below. We'll only need the source code - the files presented above.

>Note: The vendor has two separate libraries, one per sensor model, but we merge them in the `vl_uld` level, using Rust features to differentiate between the needed model, and only compiling in the code your sensors need.

## Requirements

These requirements apply to both the `vl_uld` and `vl_api`.

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
bindgen 0.72.0
-->

### Tools

```
$ sudo apt install make patch dos2unix
```


### The vendor C libary

The vendor's C driver is a separate download, with a click-through license.

1. Fetch a zip from the vendor:

	(product page) > `Tools & Software` > "Ultra lite driver (ULD) API"

	- for [VL53L8CX](https://www.st.com/en/embedded-software/stsw-img040.html)
	- for [VL53L5CX](https://www.st.com/en/embedded-software/stsw-img023.html) 

	`Get software` > `Get latest` > check the license > ...

	>[!NOTE]
	>You can `"Download as a guest"`, after clicking the license. You *will* need to provide an email address for the actual download link, but that can be a temporary one...

2. Unzip to a suitable location

3. `export VL53L8CX_ULD_API={your-path}/VL53L8CX_ULD_API`

	>If you are targeting VL53L5CX, you just change the `8` to `5`, naturally. You can also have both API's available.

	<p />
	
	>We only need access to the above folder (that has `src`, `inc`), not the whole unzipped contents.

## Supported dev kits

The workflow has been tested on these MCUs and sensors:

|||`L8CX`|`L5CX`|
|---|---|---|---|
|`esp32c6`|[ESP32-C6-DevKitM-01](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)|&check;|&check;|
|`esp32c3`|[ESP32-C3-DevKitC-02](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitc-02.html)|*tbd.*|*tbd. check again..*|

>ESP32-C3 support is currently on hold. It needs to be retested, at some point. `#volunteers` 
>
>Such testing involves:
>
>- suggesting new pin layouts for ESP32-C3 (the project layout supports having multiple boards, but previous C3 layouts have been removed).
>- checking compatibility with both `probe-rs` and `espflash`. Later versions of `probe-rs` are incompatible with the `esp-hal`'s use of I2C bus (or the other way round; it's essentially a hardware issue so neither of the above are being blamed here..); anyways, one should check whether things work or don't with `probe-rs` - and provide tools and documentation for developing with `espflash`, if necessary.


## Next

Move to the `vl_api` folder. With the above preparation, you should be able to compile the examples.

`vl_api/WIRING.md` has information on how to wire the ESP32 MCU with either of the supported sensors. Once you have the wiring done, you can run examples.

