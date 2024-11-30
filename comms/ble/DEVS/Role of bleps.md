# Role of `bleps`

As of Nov'24, the Rust `no-std` ecosystem does not provide a BLE stack.

The [`esp-wifi`](https://github.com/esp-rs/esp-hal/tree/main/esp-wifi) crate provides HCI level BLE access, but that is not sufficient for writing applications.

`esp-hal` examples use [`bjoernQ/bleps`](https://github.com/bjoernQ/bleps), which presents itself as:

><h2>bleps - A toy-level BLE peripheral stack</h2>

What does this mean.. to you?

The upper parts of the stack are here more wobbly than with `esp-hal` and `Embassy`, normally. 

To counteract this, the author intends to wrap the `bleps` use within the `src` of this crate itself.

