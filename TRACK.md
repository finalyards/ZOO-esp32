# Track

## Ability to steer features only for `--example`s

Surprised this isn't a feature.

- ["Allow dependencies that only apply to specific cargo targets (bin, example, etc.)"](https://github.com/rust-lang/cargo/issues/1982) (`cargo` GitHub issues; opened 2015)

This would allow us simpler management of features in `Cargo.toml`s. Now they need to juggle with optional dependencies, to keep actual `lib` dependencies light.

## A way to detect `example`, `bin` or `lib` builds, in `build.rs`

- ["Environment variable to detect target build type"](https://github.com/rust-lang/cargo/issues/11714) (opened Feb'23)

## config.toml support [env] with [env.<cfg>]

- [`config.toml` support `[env]` with `[env.<cfg>]`](https://github.com/rust-lang/cargo/issues/10273)

   Initially, also wanted to use env. strings within values, but there may be other ways to reach the same (mostly: `CHIP`, `TARGET`).
