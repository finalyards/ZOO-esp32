# Why not features?

ESP32 Rust projects tend to use two approaches (together), for covering multiple chip targets:

- templates (`cargo generate`)
- features (for each separate MCU)

The author of this repo has considered this route, but decided to do things differently. With this repo, you can **CHANGE THE TARGET MCU at ANY TIME**.

```
$ ./set-target.sh 
1) esp32c3
2) esp32c6
Pick your target: 1

'esp32c3' selected.

Continue? (Y/n) 

Files '.cargo/config.toml' and 'Cargo.toml' now using:

   MCU:    esp32c3
   TARGET: riscv32imc-unknown-none-elf

```

Done.

This approach is akin to web development, where you can decide the *delivery platform* of your pages, back and forth (also in web pages, there used to be templates that would have bound you to a solution, in the beginning of the project).

*Is this a better way?*

Perhaps. 

Depends on your needs, really.

The author likes this (enough to keep the complexity it entails), because:

- we don't burden the **library** target with unnecessary features

   This is probably the main reason. Having the platform details *outside* of the `vl53l5cx` library (due to original C design from the vendor), the library doesn't know which chip it's running on. Features are used only for ..well.. *features*. What kind of data you want from the sensor. Not chip configuration.

- ..which means..

   We *could* use features, if there was a mechanism where *examples* have features - that don't fall to the library in the same repo. They don't. <!-- tbd. what abour 'resolver=2'? -->
   
This approach keeps e.g. the `Cargo.toml` pretty clean.

*Where is the complexity?*

In `set-target.sh` - and needing a couple of places in the project adhere to standards (`.cargo/config.toml`, `Cargo.toml` and `examples/pins.rs`). The adherence is pretty invisible to humans; and if we break it, `set-target.sh` will likely tell. Eventually.

## What about Xtensa?

Xtensa targets differ from RISC V in a few extra bits; the author chose to leave them in a branch of their own (see `adafruit-feather-v2`), which gets `main` changes pulled in, if one remembers to.

That branch *doesn't have RISC V components*, and only targets a *single* Xtensa chip. So.. you can use the library with Xtensa, but it's like 2nd tier supported. This is also because the author does not own Xtensa boards, so actual hw validation would lack behind, anyways.

## Opinions?

Please share them in Discussions or as Issues.

I hope you like the repo!

