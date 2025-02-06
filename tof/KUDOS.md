# KUDOS

## [bjoernQ/vl53l5-c2rust](https://github.com/bjoernQ/vl53l5-c2rust)

Bjoern's repo was very helpful in debugging some I2C / `esp-hal` issues.

Simple. Doesn't do `bindgen` but carries the vendor driver's Rustified version with it. Easiest first step to VL* sensors, in Rust.

Run it by:

```
$ cargo run --release --example basic
```

