# Using `build_snippets`

The code in `build_snippets` is used by the `build.rs` files of both `vl_uld` and `vl_api` projects.

Why this arrangement?

Mainly, because it works. We have `.cargo/config.toml` setting the Cargo `build.target` - and this mechanism **can not be overridden** in a Cargo project below this path!! <!-- cargo 1.89 -->

**Problem**

Every Cargo project within this repo (since it overrides the `build.target` - which in itself is a good thing and simplifies the patterns)..

..must target the embedded ESP32 chips - NOT the host.

>This is just unfortunate, and there are [discussions](https://internals.rust-lang.org/t/problems-of-cargo-config-files-and-possible-solutions) about whether the tree-traveling `.cargo/config.toml` way is indeed the way Cargo should be going.

**Solutions**

- You *can* make a host-targeting subproject, but it needs to be built with explicit target: 

	```
	$ cargo build --target $(rustc -vV | grep host | 	cut -d' ' -f2)
	```

- ..or you can place an explicit target in the local `.cargo/config.toml`:
	
	```
	[build]
	target = "x86_64-unknown-linux-gnu"
	```

	Now, you broke builds on, say, Apple Silicon hosts.

- Make a *separate* GitHub repo, for the host tools.

	Sure. Will work. Elegant. If this remains a problem, this may be the way to go.


**Work-around**

The author had been using `build_snippets/pins.in` already before - but had two similar ones, one in `vl_uld`, one in `vl_api`.

Just merge those together, and place it in the parent folder.

This works, because *a snippet does not care, where it's used*. When used within `build.rs`, it *automatically gets built under the host target*. Case closed.

<!-- #hidden
Minor inconveniencies:

- cannot really use `feature`s: the file is above the projects using it, and IDE has no way of knowing what features are set/unset. It's best to go featureless.
-->