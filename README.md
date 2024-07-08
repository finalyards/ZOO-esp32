# VL53L5CX with Rust and Embassy

## Background

The ST [VL53L5CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html) is a tiny (6.4 x 3.0 x 1.5 mm) surface mounted laser distance meter. It is interfaced with I²C but the native C library really is the way it needs to be steered with (I²C data details are not documented). 

This library aims to steer the sensor (multiple of them!) using Rust. 

Embedded Rust, using `async` via [Embassy](http://embassy.dev/).

## Requirements

- 1..more of the VL53L5CX sensors
- ESP32-C3 (or similar) MCU
- breadboard to connect the two

![](.images/layout.png)

*Image 1. Development setup*

<!-- editor's note: 
Original is stored in `.excalidraw/`
-->

### USB setup

We airgap the electronics from the main development computer using USB/IP. If you follow the suggested VM setup, this means you don't have to install anything except for an IDE and Multipass on your host computer.

>The author uses macOS as a host, but both Windows and Linux should work the same.

**Software**

Set up the development environment using [`mp`](https://github.com/akauppi/mp) repo, especially `rust+emb` folder within it.

>You *can* also develop on a single machine, but troubles with USB and what not might not be worth it. If you do things differently, you are on your own. :)

<!-- Developed on
macOS 14.5
Multipass 1.14.0-rc1
ESP32-C3-Devkit-C02 (revision xxx)
-->

## Preparation

The [VL53L5CX_ULD library](https://www.st.com/en/embedded-software/stsw-img023.html) is a separate download

1. Fetch it from ST (link above)
2. Copy the folder `VL53L5CX_ULD_driver_2.0.0/VL53L5CX_ULD_API` to `VL53L5CX_ULD_API`
3. Run `make prep`

## Compilation (no hardware needed)

```
$ cd def
```

```
$ cargo build --release --bin embassy_hello_world --features esp32c3 --target riscv32imc-unknown-none-elf
```

>**tbd.** Make the repo bi-targeted, with easy swap between - say - C3 and C6. 
>
>That will also **shorten** the above command.

**TBD. UNFINISHED. Compilation fails.**

<!--
>**FAILS BY:**
>
>```
>...
>   Compiling embassy-time v0.3.1
>error: linking with `rust-lld` failed: exit status: 1
  |
>  = note: LC_ALL="C" PATH="/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/ubuntu/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" VSLANG="1033" "rust-lld" "-flavor" "gnu" "/tmp/rustc2IjKG9/symbols.o" "/home/ubuntu/VL53L5CX-rust/def/target/riscv32imc-unknown-none-elf/release/deps/embassy_hello_world-a95e6d01ad2cf5eb.esp_hal_embassy-33fab272fd2651dd.esp_hal_embassy.b1af071531646076-cgu.0.rcgu.o.rcgu.o" "--as-needed" "-L" "/home/ubuntu/VL53L5CX-rust/def/target/riscv32imc-unknown-none-elf/release/deps" "-L" "/home/ubuntu/VL53L5CX-rust/def/target/release/deps" "-L" "/home/ubuntu/VL53L5CX-rust/def/target/riscv32imc-unknown-none-elf/release/build/defmt-b503f62ac170b7d8/out" "-L" "/home/ubuntu/VL53L5CX-rust/def/target/riscv32imc-unknown-none-elf/release/build/esp-hal-a700bc27812760e5/out" "-L" "/home/ubuntu/VL53L5CX-rust/def/target/riscv32imc-unknown-none-elf/release/build/esp32c3-fba2958ec6e360c2/out" "-L" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv32imc-unknown-none-elf/lib" "-Bstatic" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv32imc-unknown-none-elf/lib/libcompiler_builtins-a7dce359b6e97842.rlib" "-Bdynamic" "-z" "noexecstack" "-L" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/riscv32imc-unknown-none-elf/lib" "-o" "/home/ubuntu/VL53L5CX-rust/def/target/riscv32imc-unknown-none-elf/release/deps/embassy_hello_world-a95e6d01ad2cf5eb" "--gc-sections" "--strip-debug" "-Tlinkall.x" "-Tlinkall.x"
  = note: rust-lld: error: /home/ubuntu/VL53L5CX-rust/def/target/riscv32imc-unknown-none-elf/release/build/esp-hal-a700bc27812760e5/out/memory.x:18: region 'ICACHE' already defined
          >>>     ICACHE : ORIGIN = 0x4037C000,  LENGTH = 0x4000
          >>>                        
>```
-->

<!--
## Tests
etc..
-->



## References

- [Breakout Boards for VL53L5CX](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) (ST.com)
- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)

