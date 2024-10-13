# Semihosting

Semihosting is a wonderful thing! What it does is allow use of *your host resources (time, file system, stdin/out) from within the code running in the MCU*.<sup>`[1]`</sup> This is highly practical for making interactive examples and automated tests.

Note that semihosting is only a *debugging / development feature*. It requires a host to interact with and is thus not something you'd count on, for actual operation.

For us, the semihosting features are made available by the [`semihosting`](https://crates.io/crates/semihosting) crate.

**Some** semihosting features are not available (not stable) for RISC-V. Here's a table (on `semihosting` 0.1.15):

|what|status|
|---|---|
|`io`, `fs`, `process`|works|
|`args`, `time`|experimental; did not work on `stable` Rust|

>There are ways around the lack of stable `experimental` (heh, a misnomer anyways!) support. For example, you can write things in a file and let the MCU read such. Or pipe via `stdin`.

#### References

- Segger Wiki > [Semihosting](https://wiki.segger.com/Semihosting)
- ["Understanding RISC-V semihosting"](https://embeddedinn.com/articles/tutorial/understanding-riscv-semihosting/) (blog, Oct'22) `[1]`
- Crates.io > [`semihosting`](https://docs.rs/semihosting/0.1.15/semihosting/index.html)
