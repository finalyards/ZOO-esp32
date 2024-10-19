# `defmt` or `probe-rs/rtt-target`?

`probe-rs` CLI does not support stdin in `semihosting`. This means, in order to make interactive examples, we can either:

- connect multiple USB ports for a separate UART (with an opto-isolator) -> hw clutter!
- use a dumb button input -> takes up a GPIO pin + needs extra parts
- use wireless netorking (Bluetooth or WLAN) -> overly complicated for a single prompt!
- exchange `defmt` (logging library) with `probe-rs`'s own `rtt-target`

   This library has `defmt` support, so we can see it as an extention to / replacement of the `defmt-rtt` library.

## `defmt-rtt`

By the authors of `defmt`, Ferrous Systems.

Only provides logging; no target-side input.

```rust
use defmt_rtt as _;
```

## `rtt-target`

By the `probe-rs` people themselves.

>*implements input and output via a debug probe*

<span />
>*By convention channel 0 is reserved for terminal use.*

```rust
use rtt_target::{rtt_init_print, rprintln};

fn main() -> ! {
   rtt_init_print!();
```

>*tbd. Finish this, even if we wouldn't use `rtt-target`*

[Reading sample](https://docs.rs/rtt-target/latest/rtt_target/#reading) in `rtt-target` docs.

## References

- crates.io
   - [`defmt-rtt`](https://crates.io/crates/defmt-rtt)
	- [`rtt-target`](https://crates.io/crates/rtt-target)

- `rtt-target` [docs](https://docs.rs/rtt-target/latest/rtt_target/)

