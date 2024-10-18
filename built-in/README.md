# built-in

Exercising features that don't need any added hardware, in addition to an MCU.

- reading binary buttons (with rebounce management)
- reading andlog signals
- internal temperature sensor
- ...

<!-- Note: 
handling the super-LED that exists on the devkits is NOT seen as a built-in. It's part of the devkit, not the MCU.
-->


## Examples

### `button`

```
$ cargo run --release --example button
```

Reading a button can be used for interactive prompting (since `semihosting` doesn't provide that on `probe-rs`).



---

>*tbd. ADC reading is intended to come here*
>
>*temperature sensor likely less so*
