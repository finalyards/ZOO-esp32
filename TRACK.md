# TRACK

## `probe-rs` support for `esp32c3`

- [""Fix" ESP32-C2 and ESP32-C3"](https://github.com/probe-rs/probe-rs/pull/2748)

- ["Flashing esp32-s3 with probe-rs causes flash algorithm verification fail"](https://github.com/probe-rs/probe-rs/issues/2338) (probe-rs GitHub)

<!-- hidden; we can overcome this
## Reasons keeping us on `nightly`

### `type_alias_impl_trait`

- [Tracking issue for RFC 2515, "Permit impl Trait in type aliases"](https://github.com/rust-lang/rust/issues/63063)

- [ ] Once that's in, we could try again with `stable`.  ((Likely some places need `nightly`, would be good to list them.))

    ```
    $ git grep nightly
    Cargo.toml:static_cell      = { version = "2.1.0",  features = ["nightly"] }  # needs nightly?
    rust-toolchain.toml:channel = "nightly"
    src/bin/test.rs:#![feature(type_alias_impl_trait)]    // needs 'nightly'
    src/bin/test.rs:    make_static     // nightly
    src/bin/test.rs:    let timers = make_static!(timers);  // nightly
    uld-sys/Makefile:#      - '--formatter=prettyplease' (not needing nightly?) didn't work
    uld-sys/Makefile:#      - using 'rustfmt.toml' > normalize_doc_attributes requires nightly
    uld-sys/Makefile:       rustup run nightly \
    uld-sys/Makefile:       rustup run nightly \
    uld-sys/rustfmt.toml:    # needs nightly
    ```
-->

<!-- hidden; overcame by placing `main` in its own folder
## Avoid using parent's `.cargo/config`

- [How-to: ignore cargo config file in parent folder](https://users.rust-lang.org/t/how-to-ignore-cargo-config-file-in-parent-folder/55232) (Feb'21)

- [Add flag to ignore all parent directory configs](https://github.com/rust-lang/cargo/issues/7621) (GitHub Issue; 2019)
-->

