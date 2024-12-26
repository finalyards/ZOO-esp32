# TRACK

<!-- solved
## Sniffing the (ELF) output path in `cargo`

- [ ] ["Need a reliable way to get the target dir from the build script"](https://github.com/rust-lang/cargo/issues/9661)

	The determination to *not make it easy* to know where `target` really is seems clear, from the above issue. It's not going to be "fixed".
	
	IF we had a way to sniff <sub>(not doing a `ls` runner)</sub> the path where the ELF will be built, we could:
	
	- alongside the `runner`, use our `Makefile.2` to "just run" (no building) the output

	The current "no" approach is **restrictive** and **it's relatively easy to make a Rust tool that sniffs the ELF output path**. (call it `hound`?)

	Just.. not doing it.. yet.
-->

## `array_try_map`

- ["Tracking issue for array::try_map"](https://github.com/rust-lang/rust/issues/79711)

	At least `tof/vl53l5cx` will benefit from this: turning `[A;N]` into `Result<[B;N]>`.
	

## Rust Rover IDE: Symbol resolving 

- [Symbols not resolved if dependency is from GitHub, and features used](https://youtrack.jetbrains.com/issue/RUST-16444/Symbols-not-resolved-if-dependency-is-from-GitHub-and-features-used) (RustRover YouTrack; Nov'24)

	We have at least one place (`comms/ble` use of `trouble-host`) where this bug causes trouble -heh ;)- in the IDE.
