/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
use anyhow::*;

// Snippets need to be read in here (cannot do in "statement position")
include!("build_snippets/pins.in");   // process_pins(toml: &str, board_id: &str) -> anyhow::Result<()>

/*
* Note: 'build.rs' is supposedly run only once, for any 'examples', 'lib' etc. build.
*
* References:
*   - Environment variables set
*       -> https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
*/
fn main() -> Result<()> {
    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    #[allow(non_snake_case)]
    let IDE_RUN = std::env::var("__CFBundleIdentifier").is_ok();

    // If IDE runs, terminate early.
    if IDE_RUN { return Ok(()) };

    // DEBUG: Show what we know about the compilation.
    //
    //  <<
    //   CARGO_CFG_TARGET_FEATURE=c,m
    //   CARGO_FEATURE_{..feature..}=1
    //   LD_LIBRARY_PATH=/home/ubuntu/VL53L5CX_rs.cifs/vl53l5cx_uld/target/release/deps:/home/ubuntu/VL53L5CX_rs.cifs/vl53l5cx_uld/target/release:/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib
    //   OUT_DIR=/home/ubuntu/target/riscv32imac-unknown-none-elf/[...]
    //   RUSTUP_TOOLCHAIN=stable-x86_64-unknown-linux-gnu
    //   TARGET=riscv32imc-unknown-none-elf
    //  <<
    //
    #[cfg(not(all()))]
    {
        std::env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        std::process::exit(1);
    }

    // Pick the current MCU. To be used as board id for 'pins.toml'.
    //
    // $ grep -oE -m 1 '"esp32(c3|c6)"' Cargo.toml | cut -d '"' -f2
    //  esp32c3
    //
    let board_id: String = {
        use std::process::Command;
        let output = Command::new("sh")
            .arg("-c")
            .arg("grep -oE -m 1 '\"esp32(c3|c6)\"' Cargo.toml | cut -d '\"' -f2")
            .output()
            .expect("'sh' to run");

        // 'output.stdout' is a 'Vec<u8>' (since, well, could be binary)
        //
        let us: &[u8] = output.stdout.as_slice().trim_ascii();
        let x = String::from_utf8_lossy(us);

        //: println!("cargo:warning=BOARD ID: '{}'", &x);     // BOARD ID: 'esp32c3'
        x.into()
    };

    //---
    // Config sanity checks
    {
        // nada
    }

    // Expose 'OUT_DIR' to an external (Makefile) build system
    {
        use std::{fs, env};
        const _FN: &str = ".OUT_DIR";

        let out_dir = env::var("OUT_DIR")
            .expect("OUT_DIR to have a value");

        fs::write(_FN, out_dir)
            .expect(format!("Unable to write {_FN}").as_str());
    }

    //---
    // Turn 'pins.toml' -> 'examples/pins_gen.in’ (named within the TOML itself)
    {
        let toml = include_str!("./pins.toml");
        process_pins(toml, &board_id)?;
    }

    // Link arguments
    //
    {
        let link_args: Vec<&str> = vec!(
            "-Tlinkall.x",
            "-Tdefmt.x"     // required by 'defmt'
        );

        link_args.iter().for_each(|s| {
            println!("cargo::rustc-link-arg={}", s);
        });
    }

    Ok(())
}
