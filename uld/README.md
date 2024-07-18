# `uld-sys`

Turns the `VL53L5CX_ULD_API` source code into something that can be touched with Rust.

## Pre-reading

- ["Using C Libraries in Rust"](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a) (blog, Aug '19)

   A bit old, but relevant (C API's don't age!).
   
- [`bindgen` book](https://rust-lang.github.io/rust-bindgen/introduction.html)

  >Note: Not that you really need to pre-read the above. `#joking`

## The job

![](.images/bindgen-jumps.png)

><font color=orange>*tbd. CHANGES TO THE IMAGE ARE LIKELY!*</font>


## Compiling 

```
$ cargo build
```

>Note: Somewhat unintuitively, the cradle doesn't need any Chip-select features. No `esp32c3`-like.
>
>This is *either* because
>
>- a) the compilation (`clang`) is done using a single target that covers all ESP32 (RISC-V) chips: `riscv32`
>- b) the author hasn't really cracked it; such features *will* be needed!!


## References

- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)



