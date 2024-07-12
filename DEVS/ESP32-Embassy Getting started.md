# ESP32 + Embassy; Getting started

Wrote a repo about this:

- [ESP32-Embassy sample](https://github.com/lure23/ESP32-Embassy-sample)

**TL;DR**: The official `esp-template` approach does not support Embassy (just yet), and would lock one in with a single target type.

We adjust the approach `esp-hal/examples` has, which allows multiple targets (chip variants) to be used, from the same code base. However, we do this without needing `cargo xtask`.

All this is likely to remain in flux, for a while. **Track changes in that repo.**
