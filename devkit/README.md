# `devkit`

Hardware available in supported devkits - but not an MCU feature.

- multi-colored LED 🚨🟢🔵

<!--
- BOOT button
-->

## Examples

### `rgb`

```
$ cargo run --release --features=rgb-led,defmt --example rgb
...
```

Cycles the RGB LED.


<!--
### `button`

```
$ cargo run --release --example button
```

<!_-- #whisper
Reading a button could be used for interactive prompting (since `semihosting` doesn't provide that on `probe-rs`).
--_>

-->