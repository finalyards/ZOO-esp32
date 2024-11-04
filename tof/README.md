# `tof`

A "Time of Flight" sensor measures distance in front of it, by actively sending pulses (of infrared laser) and measuring the time (in picoseconds!!!) it takes to see the reflections.

>To the author, these sensors remain purely magical!

## vl53l5cx

You need two libraries:

- [`vl53l5cx`](vl53l5cx/README.md) - Application level API and examples

	This subproject is what you'd likely use in your own code. It provides a "platform" adaptation for `esp-hal` API's and uses [Embassy](https://embassy.dev) to provide `async` interfaces for waiting for measurements.

	There are some examples, too.

- [`vl53l5cx_uld`](vl53l5cx_uld/README.md) - Lower level C/Rust interface for the above project.

	ULD stands for ["Ultra Light Driver"](https://www.st.com/en/embedded-software/stsw-img023.html) - it's the vendor's terminology for their embedded C driver (not Linux).

Before using the libraries, you need to:

1. Check out the `ULD` side's [`README`](vl53l5cx_uld/README.md), follow the instructions.
2. Move to higher API side, and run some examples.
3. Adapt to your own needs?

If you file discussions or issues on this GitHub, please prefix them with `[tof]`. 

See you there!
