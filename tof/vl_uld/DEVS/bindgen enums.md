# `bindgen` and enums

`bindgen` has a "few" ways to handle C enums:

```
$ bindgen --help
[...]
     --default-enum-style <STYLE>
          The default STYLE of code used to generate enums
      --bitfield-enum <REGEX>
          Mark any enum whose name matches REGEX as a set of bitfield flags
      --newtype-enum <REGEX>
          Mark any enum whose name matches REGEX as a newtype
      --newtype-global-enum <REGEX>
          Mark any enum whose name matches REGEX as a global newtype
      --rustified-enum <REGEX>
          Mark any enum whose name matches REGEX as a Rust enum
      --constified-enum <REGEX>
          Mark any enum whose name matches REGEX as a series of constants
      --constified-enum-module <REGEX>
          Mark any enum whose name matches REGEX as a module of constants
```

What are they, really?

["Flavors of enums with Rust bindgen"](https://mdaverde.com/posts/rust-bindgen-enum/) (blog, May'22)

...and [`bindings.rs`](https://github.com/mdaverde/bindgen-enum-flavors/blob/main/src/bindings.rs).

>People shouldn't be using Rust enums unless they have complete control of the C/C++ code <sub>[source](https://github.com/rust-lang/rust-bindgen/issues/758)</sub>

This seems to say that `rust` (`--rustified-enum`) has been the original method, and it's perfectly fine to use it, in our case.
	

## Other reading

- [bindgen docs on Enums](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#enums)

