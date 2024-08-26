# Track

## Ability to automatically set features only for `--example` builds

>[UPDATE] This *would* be nice, but the whole system seems to work differently.

Surprised this isn't a feature. <sub>*pun semi-intended*</sub>

- ["There is currently no way to specify features automatically when using `cargo doc`"](https://github.com/rust-lang/cargo/issues/8905) (cargo GitHub Issue, opened 2020)
- ["Allow dependencies that only apply to specific cargo targets (bin, example, etc.)"](https://github.com/rust-lang/cargo/issues/1982) (cargo GitHub issue; opened 2015)
- ["Activating features for tests/benchmarks"](https://github.com/rust-lang/cargo/issues/2911) (cargo GitHub issue, opened 2016)

All of those are essentially asking for the same thing, but from different angles (`doc`, `test` etc.).

>2c:
>In short, the best suggestion the author saw is (because it's backwards compatible):
>
>- add to `Cargo.toml`: `{doc|test|example|...}-defaults = [...]`
>- not changing `--no-defaults` (??) functionality; it would only apply to core `defaults`
>- `example-defaults` (etc.) would be used *instead of* `defaults`, with the default (`example-defaults = ["default"]`)

<!-- not gonna happen; 'build.rs' doesn't get run like that
## A way to detect `example`, `bin` or `lib` builds, in `build.rs`

- ["Environment variable to detect target build type"](https://github.com/rust-lang/cargo/issues/11714) (opened Feb'23)

If this was possible, we could set `example`-specific features within `build.rs`.
-->

## config.toml support [env] with [env.<cfg>]

- [`config.toml` support `[env]` with `[env.<cfg>]`](https://github.com/rust-lang/cargo/issues/10273)

Initially, also wanted to use env. strings within values, but there may be other ways to reach the same (mostly: `CHIP`, `TARGET`).


## `bin` (or `example`) file names need to be valid identifiers

- ["Defmt's proc macros issue an error if the filename is not a correct identifier"](https://github.com/knurling-rs/defmt/issues/853)

	There doesn't seem to be a real reason for this.
	
	>Note: it's not a Rust/Cargo restriction, but one of the `defmt` library.

	Work-around: 
	
	- Prefixing `**/examples/[0-9]*.rs` with `_`
	

## Cargo: larger use of SQLite

- ["Cargo cache cleaning" > "Plan for the future"](https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html#plan-for-the-future) (blog, Dec'23)

	>*"[...] When cargo downloads registry index data, it stores it in a custom-designed binary file format to improve lookup performance. However, this index cache uses many small files, which may not perform well on some filesystems."*

Running `cargo build` currently (Aug'24) takes some 30+ seconds up-front, where the CPU load of `cargo` is only around 10%. This is weird. (Would it be, say, 100%, it should only take 3s). 
	
If this is due to Multipass mounts, a move to SQLite *may* have dramatic effects!
	
- [ ] Find (?) a tracking issue for this? (did not...)
- [ ] When / how to try this on `nightly`?
- [ ] ... `stable`?

	