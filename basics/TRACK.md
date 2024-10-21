# TRACK

## `probe-rs` semihosting availability

- ["FR [doc]: Ability to exit back to host's command prompt on-cue from embedded program"](https://github.com/probe-rs/probe-rs/issues/2801)

	i.e. what's available and what's not needs to be found out via trial; should be at least minimally documented


## `defmt` request for "multiple channels"

- ["Question: Multiple Channels"](https://github.com/knurling-rs/defmt/issues/552)

	It is closed. But it might be the place to ask for this, if we.. want to.
	
	Ideally, the author would like to see `defmt` logging macros and use of more RTT channels be completely separated. In that way, this is perhaps only a documentation issue between (or in the gaps between) multiple actors:
	
	- Knurling/Ferrous Systems (defmt)
	- `probe-rs`
		- CLI
		- `cargo embed`
		- `rtt-target`

	The only problem is that this is *not approachable* for a newcomer Rust embedded developer!  Their time should not be drained to understanding an ecosystem, but getting certain "boxes checked" and focusing on the application's complexity.

	Right?

	Ending this jungle session now, as unsuccessful. 🦖🐊🍃🌺🌿🌿🌿
