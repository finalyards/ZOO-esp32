# `v53l5cx_uld`

Turns the `VL53L5CX_ULD_API` source code into something that can be touched with Rust.

>Note: Usually Rust/C bindings are done in two layers: a `-sys` library forming a 1-to-1 bridging to C code, and another library adapting the use for Rust.
>
>Currently, we are keeping everything together (for simplicity); depends on the amount of Rust adaptation.

## Pre-reading

- ["Using C Libraries in Rust"](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a) (blog, Aug '19)

   A bit old, but relevant (C API's don't age!).
   
- [`bindgen` book](https://rust-lang.github.io/rust-bindgen/introduction.html) (optional)

  >Note: Not that you really need to pre-read the above.

## The job

![](.images/bindgen-jumps.png)

><font color=orange>*tbd. CHANGES TO THE IMAGE ARE LIKELY!*</font>


## Requirements

- `bindgen`:

	```
	$ apt install llvm-dev libclang-dev clang
	```
	
	```
	$ cargo install bindgen-cli
	```

- `rustfmt`

   ```
   $ rustup component add rustfmt
   ```

- Gnu `make`

```
$ clang -print-targets | grep riscv32
    riscv32     - 32-bit RISC-V
    riscv64     - 64-bit RISC-V
```


## Compiling 

```
$ cargo build
```

>DEV Hint: The command uses a `Makefile` internally. You can also use it directly; have a look at its contents.

<span />

>Note: Somewhat unintuitively, the cradle doesn't need any Chip-select features. No `esp32c3`-like.
>
>This is *either* because
>
>- a) the compilation (`clang`) is done using a single target that covers all ESP32 (RISC-V) chips: `riscv32`
>- b) the author hasn't really cracked it; such features *will* be needed!!


## References

- [Ultra Lite Driver (ULD) for VL53L5CX multi-zone sensor](https://www.st.com/en/embedded-software/stsw-img023.html) (ST.com)



