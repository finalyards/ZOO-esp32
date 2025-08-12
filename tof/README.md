# `tof`

A "Time of Flight" sensor measures distance in front of it, by actively sending pulses (of infrared laser) and measuring the time (in picoseconds!!!) it takes to see the reflections.

>To the author, these sensors remain purely magical!

## VL53L5CX

You need two libraries:

- [`vl53l5cx`](vl53l5cx/README.md) - Application level API and examples

	This project provides a "platform" adaptation for `esp-hal` I2C bus to talk with the lower level "ULD" library (see below). It also uses Embassy to provide `async` interfaces for waiting for measurements.
	
	This is the level you are expected to build your own applications on top.

- [`vl53l5cx_uld`](vl53l5cx_uld/README.md) - Lower level C/Rust interface

	ULD stands for ["Ultra Light Driver"](https://www.st.com/en/embedded-software/stsw-img023.html) - it's the vendor's terminology for their embedded C driver (not Linux).

Before using the libraries, you need to:

1. Check out the `ULD` project's [`README`](vl53l5cx_uld/README.md), follow the instructions.
2. Move to higher API project, and run some examples.
3. Adapt to your own needs.

See you there!
