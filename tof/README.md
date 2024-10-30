# `tof`

A "Time of Flight" sensor measures distance in front of it, by actively sending pulses (infrared laser) and *measuring the time* (in picoseconds!!!) it takes to hear the echos.

To the author, these sensors remain purely magical!

- [`vl53l5cx`](vl53l5cx/README.md) - ST.com [vl53l5cx](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html) sensor API and examples

	This subfolder shows how to use the sensor, either as a single, or as a flock of multiple.

- [`vl53l5cx_uld`](vl53l5cx_uld/README.md) - Lower level C/Rust interface for the ST.com [vl53l5cx](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html)

	ULD stands for ["Ultra Light Driver"](https://www.st.com/en/embedded-software/stsw-img023.html) - it's the ST.com terminology for their embedded C driver.

	You need to download such C code, and place it within the project. See the `README` for instructions. In practise, you'd likely program applications against the higher-level (non-ULD) Rust APIs, though.
