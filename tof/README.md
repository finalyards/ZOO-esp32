# `tof`

A "Time of Flight" sensor measures distance in front of it, by actively sending pulses (of infrared laser) and measuring the time (in picoseconds!!!) it takes to <strike>hear</strike> see the echos.

>To the author, these sensors remain purely magical!

## vl53l5cx

- [`vl53l5cx_uld`](vl53l5cx_uld/README.md) - Lower level C/Rust interface for the [vl53l5cx](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html).

	ULD stands for ["Ultra Light Driver"](https://www.st.com/en/embedded-software/stsw-img023.html) - it's the vendor's terminology for their embedded C driver (not Linux).

	You need to download such C code, and place it within the project. See the `README` for instructions.

- [`vl53l5cx`](vl53l5cx/README.md) - Application level API and examples

	Using the ULD driver (above), this subproject is what you'd use as an application-facing API. It supports using [Embassy](https://embassy.dev) to make your applications multitasking-savvy.
	
	There are some examples, too.


## vl53l7cx

Though the author doesn't have any of these, the vendor site states that VL53L7CX is *"pinouts and driver compatible"* with the previous model.

If you successfully use the library with VL53L7CX, please mention. :)


<!-- #whisper
## vl53l8cx

This chip is a further deviation / development, and offers e.g. SPI in addition to I2C communications. It is not claiming pin or driver compatibility with VL53L5CX.
-->
