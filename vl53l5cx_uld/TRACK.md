# Track

## `probe-rs` doesn't support semihosting args, yet

- [ ] *tbd. tracking issue*

Semihosting `args` *is* an experimental feature. While the library does seem to support it, `probe-rs` has this to say:

```
 WARN probe_rs::cmd::run::normal_run_mode: Target wanted to run semihosting operation SYS_GET_CMDLINE, but probe-rs does not support this operation yet. Continuing...
 WARN probe_rs::cmd::run::normal_run_mode: Target wanted to run semihosting operation 0x13 with parameter 0x0,but probe-rs does not support this operation yet. Continuing...
```

In effect, one cannot use args, with `probe-rs`, yet.

Version: `0.24.0 (git commit: 63c59d2f)`

References:

- semihosting docs > [optional features](https://docs.rs/semihosting/latest/semihosting/#optional-features)

