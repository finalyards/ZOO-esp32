# `tof`

Support for using Time-of-Flight sensors (IR distance measurement), from Rust.

Following chips are supported:

- [VL53L8CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l8cx.html)
- [VL53L5CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html)

You can use such sensors for:

- distance measurement (2 &#8229; 300cm, depending on reflectivity)
- movement and presence detection
- gesture control

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

## Introductions

You need two libraries:

### [`vl_uld`](vl_uld/README.md) - Lower level C/Rust interface

ULD stands for ["Ultra Light Driver"](https://www.st.com/en/embedded-software/stsw-img023.html) - it's the vendor's terminology for their embedded C driver (not Linux).

This library provides the Rust/C interfacing, and some other goodies for providing a higher abstraction level than what the C level does (e.g. matrices are presented with x,y indices). 
	
You need to build this library first, for your sensor type; see instructions within `vl_uld/README`.

### [`vl_api`](vl_api/README.md) - Application level API and examples

This project provides a "platform" adaptation for `esp-hal` I2C bus to talk with the lower level ULD library. It also uses Embassy to provide `async` interfaces for waiting for measurements.
	
This is the level you are expected to build your own applications on top.


## Getting started

1. Check out the `ULD` project's [`README`](vl_uld/README.md), follow the instructions.
   - You'll need to download the vendor C library sources: [STSW-IMG023](https://www.st.com/en/embedded-software/stsw-img023.html) (for VL53L5CX) or [STSW-IMG040](https://www.st.com/en/embedded-software/stsw-img040.html) (for VL53L8CX)
2. Move to higher API project, build it, and run some examples.
3. Use in your projects.

