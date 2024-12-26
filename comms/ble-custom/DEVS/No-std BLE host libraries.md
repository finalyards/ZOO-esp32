# `no-std` BLE host libraries for Rust


We need a way to create *custom* BLE services on ESP32 hardware, using Embassy (i.e. `no-std` environment).

The current (Nov'24) choices seem to be:

|project|comments|suitability|
|---|---|---|
|[`bjoernQ/bleps`](https://github.com/bjoernQ/bleps)|*[...] something that works for testing,demos and personal projects* <p />Used by `esp-hal` examples; does not claim to be comprehensive|looked; would be enough for an early proof of concept, but pointing people in the wrong direction.|
|[`trouble´](https://github.com/embassy-rs/trouble)|*[...] initial implementation was based on bleps*|seems ideal?|

## `bleps`

Was able to build an early, working prototype on this, but:

- it isn't published to `crates.io` (which causes IDE problems for Rust Rover; long story and not very relevant)
- did not like the `gatt!` macro, too much (it declares variable names in a magic-ish way...)

= Looking on!

## `trouble`

- Published by the `embassy-rs` themselves; that sure looks promising!!!

...tbd...

---

*If you are aware of other libraries fulfilling the above mentioned criteria (`no-std`, BLE host), please let the author know.

