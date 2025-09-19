# Semihosting with `probe-rs`

[Semihosting](https://embeddedinn.com/articles/tutorial/understanding-riscv-semihosting/) is a wonderful thing! It allows use of *your host resources (file system, terminal I/O, time) from within the code running on the MCU*. 

This could be highly practical for making interactive examples and automated tests.

However, as of Oct'24, `probe-rs`(https://github.com/probe-rs/probe-rs) has extremely limited semihosting support, and it is not clear whether the tool sees this as a priority, going forward.

Since there are no documentatation on the `probe-rs` docs, on what can be expected to work, here goes!!

|tried|[`semihosting`](https://docs.rs/semihosting) feature|did it work with `probe-rs`?|
|---|---|---|
|**Works**|
|&nbsp;&nbsp;`println!`|-|works (but we prefer `defmt` for debug logs)|
|&nbsp;&nbsp;`process::exit`|-|works|
|**Not supported** (`probe-rs`)|
|&nbsp;&nbsp;`io::stdin`|`stdio`|nope|
|&nbsp;&nbsp;reading a file|`fs`|nope|
|&nbsp;&nbsp;writing a file|`fs`|nope|
|**Experimental** (`semihosting`)|
|&nbsp;&nbsp;`experimental::env::args`|`args`|nope|
|&nbsp;`experimental::time::SystemTime`|`time`|nope|

<!-- #hidden; above shows that file input would not work..
There are ways around the lack of implementations. For example, you can write things in a file and let the MCU read such (for args, time, ...).
-->

Let's keep an eye on this; above is from Oct'24, as stated.

#### References

- Segger Wiki > [Semihosting](https://wiki.segger.com/Semihosting)
- ["Understanding RISC-V semihosting"](https://embeddedinn.com/articles/tutorial/understanding-riscv-semihosting/) (blog, Oct'22) `[1]`
- Crates.io > [`semihosting`](https://crates.io/crates/semihosting)
