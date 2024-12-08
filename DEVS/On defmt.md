# On `defmt`

[`defmt`](https://defmt.ferrous-systems.com) is used throughout for logging purposes.

Though you can read and use the examples without deeper knowledge about it, it may be a good idea to read the [`defmt book`](...) and this [post on (upcoming) 1.0](https://github.com/knurling-rs/defmt/discussions/888).

The use of `defmt` means saying "no" to `logger` crate and its APIs. This is one of the watershed decisions the author has done for you - `defmt` has its benefits in embedded surroundings, but comes at the price of some added complexity, and requirements on the host-side tooling (e.g. `probe-rs`).

The author hopes you appreciate the decision!
