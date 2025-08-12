# TRACK

## `type_alias_impl_trait` in nightly

Once this makes it to stable, `embassy-executor` should no longer need the `task-arena-size...` features. <sub>[ref](https://docs.embassy.dev/embassy-executor/git/cortex-m/index.html#task-arena)</sub>

- rust-lang > [RFC 2515](https://github.com/rust-lang/rust/issues/63063) (created Jul'19; open in Jan'25)


## `array_try_map`

- ["Tracking issue for array::try_map"](https://github.com/rust-lang/rust/issues/79711)

	At least `tof/vl53l5cx` will benefit from this: turning `[A;N]` into `Result<[B;N]>`.
	

## Rust Rover IDE: Symbol resolving 

- [Symbols not resolved if dependency is from GitHub, and features used](https://youtrack.jetbrains.com/issue/RUST-16444/Symbols-not-resolved-if-dependency-is-from-GitHub-and-features-used) (RustRover YouTrack; Nov'24)

	We have at least one place (`comms/ble` use of `trouble-host`) where this bug causes trouble -heh ;)- in the IDE.
