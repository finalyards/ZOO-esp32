# devkit

Hardware available in official (or otherwise common) devkits - but not an MCU feature.

- reading binary buttons (with rebounce management)
- writing RGB led


## Examples

### `button`

```
$ cargo run --release --example button
```

<!-- #whisper
Reading a button could be used for interactive prompting (since `semihosting` doesn't provide that on `probe-rs`).
-->

### `rgb`

...

