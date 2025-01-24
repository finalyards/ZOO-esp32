# TRACK

## `array_try_map`

- ["Tracking issue for array::try_map"](https://github.com/rust-lang/rust/issues/79711)

	At least `tof/vl53l5cx` will benefit from this: turning `[A;N]` into `Result<[B;N]>`.
	

## Rust Rover IDE: Symbol resolving 

- [Symbols not resolved if dependency is from GitHub, and features used](https://youtrack.jetbrains.com/issue/RUST-16444/Symbols-not-resolved-if-dependency-is-from-GitHub-and-features-used) (RustRover YouTrack; Nov'24)

	We have at least one place (`comms/ble` use of `trouble-host`) where this bug causes trouble -heh ;)- in the IDE.
