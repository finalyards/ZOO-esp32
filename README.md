# VL53L5CX with Rust and Embassy

## Background

The ST [VL53L5CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html) is a tiny (6.4 x 3.0 x 1.5 mm) surface mounted, matrix capable laser distance meter. It is interfaced with I²C but the native C library really is the way it needs to be steered with - I²C content details are not documented.

This library aims to steer the sensor (multiple of them!) using `async` Rust via [Embassy](http://embassy.dev/).

## Requirements

- 1 or more VL53L5CX sensors

	>The [VL53L5CX-SATEL](https://www.digikey.fi/fi/products/detail/stmicroelectronics/VL53L5CX-SATEL/14552430) development board is likely the one you will need.

- ESP32-C3 (or similar) MCU
- breadboard and wires to connect the two

![](.images/layout.png)

*Image 1. Development setup*

<!-- editor's note: 
Original is stored in `.excalidraw/`
-->

<!-- Developed on
macOS 14.5
Multipass 1.14.0-rc1
ESP32-C3-Devkit-C02 (revision xxx)
//coming VL53L5CX-SATEL (x2)
-->

## Preparation

### USB setup

We airgap the electronics from the main development computer using USB/IP. If you follow the suggested VM setup, this means you don't have to install anything except for an IDE and [Multipass](https://multipass.run) on your host computer.

>The author uses macOS as a host, but both Windows and Linux should work the same.

### Wiring

*tbd. Once I get the samples*

### Software

Set up the development environment using [`mp`](https://github.com/akauppi/mp) repo, especially `rust+emb` folder within it.

>You *can* also develop on a single machine, but troubles with USB and what not might not be worth it. If you do things differently, you are on your own. :)

#### The vendor C libary

The [VL53L5CX_ULD library](https://www.st.com/en/embedded-software/stsw-img023.html) is a separate download

1. Fetch it from ST (link above)
2. Place the folder `VL53L5CX_ULD_driver_2.0.0/VL53L5CX_ULD_API` to `VL53L5CX_ULD_API`
3. Install these (for `bindgen` to work) on the VM:

	```
	$ sudo apt install llvm-dev libclang-dev clang
	```

### Rest...

We use `bindgen` from the command line.

```
$ cargo install bindgen-cli
```

Some parts want `nightly`. GOING to work to get this away, but for now: `#sorry!`

```
$ rustup toolchain install nightly
```

```
$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

..and for Rust code generation:

```
$ rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rustfmt
```

## Compilation (no hardware needed)

```
$ make build
```

## Running

Attach the hardware via USB/IP.

>TL;DR: `sudo usbip attach -r {IP} -b {bus-id}` so that it shows up at `lsusb`.
><details><summary>Like this</summary>
>
>```
>$ lsusb
>[...]
>Bus 001 Device 004: ID 303a:1001 Espressif USB JTAG/serial debug unit
>[...]
>```
></details>

```
$ make run!
...
```


<!--
## Tests
etc..
-->



## References

### VL53L5CX

- [Breakout Boards for VL53L5CX](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) (ST.com)
- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)
