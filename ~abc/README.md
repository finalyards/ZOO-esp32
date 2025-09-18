# abc

Testing the very basics:

- Rust and `probe-rs` toolchain
- `defmt` logging
- both with and without Embassy

The purpose of this subfolder is to help (debugging) when there are changes to `esp-hal`. Eventually, we'll likely remove it. 

## Examples

### `abc`

```
$ cargo run --release --example abc
```

Just testing basics: `defmt` logging; semihosting to end the run.

The "hello world" to see that you can flash the MCU and run code on it.

<!--
### `abc-emb`

Same for Embassy.
-->

