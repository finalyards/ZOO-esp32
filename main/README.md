# `main`

This folder is for getting things started.


## Running..

```
$ CHIP=esp32c3 make ellie
```

- Should compile
- Uses Embassy (`.await`)
- Does not use hardware sensors, at all.

```
$ CHIP=esp32c3 make ellie-run
probe-rs run --chip esp32c3 --log-format '{L} {s}' target/riscv32imc-unknown-none-elf/release/ellie
      Erasing ✔ [00:00:02] [################################################################################################] 192.00 KiB/192.00 KiB @ 89.29 KiB/s (eta 0s )
  Programming ✔ [00:00:10] [###################################################################################################] 27.06 KiB/27.06 KiB @ 2.47 KiB/s (eta 0s )    Finished in 10.9757595s
INFO  Embassy initialized!
INFO  Bing!
DEBUG Bong!
...
```


<!-- tbd.
## Build library

## Test..
-->


## Logging

The author struggled quite a bit, initially, with ESP32 logging.

There is no real problem. [`defmt`](https://defmt.ferrous-systems.com) is a tremendous ecosystem for embedded, and one we can use, unoptionally.

The problem was that the `probe-rs` host-side software needed to be "fixed" (more like hacked..) for better [C2/C3 compatibility](https://github.com/probe-rs/probe-rs/pull/2748).


>**TL;DR**

<span />

>To implement `Format` for user-defined types (structs or enums) use the `#[derive(Format)]` attribute. No need for std `Display` and `Debug`.

<span />

>There are formatting parameters for some primitives: e.g. `{:u8}`, `{:bool}`. Prefer these parameters for primitives as they compress the data better.

## Debugging

**tbd.**

