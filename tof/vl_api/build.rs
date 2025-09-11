/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
use anyhow::*;

// Snippets need to be read in here (cannot do in "statement position")
include!("../build_snippets/pins.in");   // process_pins()

const PINS_OUT_FN: &str = "tmp/pins_snippet.in";

/*
* Note: 'build.rs' is supposedly run only once, for any 'examples', 'lib' etc. build.
*
* References:
*   - Environment variables set
*       -> https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
*/
fn main() -> Result<()> {
    #[allow(unused_imports)]
    use std::{env, fs, process::Command};

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
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        process::exit(1);
    }

    // Pick the current MCU. To be used as board id for 'pins.toml'.
    //
    // $ grep -oE -m 1 '"esp32(c3|c6)"' Cargo.toml | cut -d '"' -f2
    //  esp32c3
    //
    let board_id: String = {
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
        #[cfg(all(feature = "flock_synced", not(feature = "vl53l8cx")))]
        //println!("cargo:warning=Feature 'flock_synced' can ONLY be used on L8CX sensors, but 'vl53l8cx' is not enabled");
        compile_error!("Feature 'flock_synced' can ONLY be used on L8CX sensors, but 'vl53l8cx' is not enabled");

        // One of these
        #[cfg(not(any(feature = "single", feature = "flock")))]
        compile_error!("Please enable one of: {{single|flock}}");

        #[cfg(all(feature = "single", feature = "flock"))]
        compile_error!("Both features are enabled: {{single, flock}}");
    }

    // Expose 'OUT_DIR' to an external (Makefile) build system
    {
        use std::{fs, env};
        const TMP: &str = ".OUT_DIR";

        let out_dir = env::var("OUT_DIR")
            .expect("OUT_DIR to have a value");

        fs::write(TMP, out_dir)
            .expect(format!("Unable to write {TMP}").as_str());
    }

    //---
    // Turn 'pins.toml' -> 'tmp/pins_snippet.in'
    {
        #[cfg(feature="vl53l8cx")]
        const SENSOR_ID: &str = "vl53l8";   // without "cx"
        #[cfg(feature="vl53l5cx")]
        const SENSOR_ID: &str = "vl53l5cx";

        let toml = include_str!("../pins.toml");
        let snippet: String = process_pins(toml, &board_id, SENSOR_ID)?;

        let fn_ = PINS_OUT_FN;

        fs::write(fn_, snippet).with_context(
            || format!("Unable to write {fn_}")
        )?;

        // Change in TOML retriggers a build
        println!("cargo::rerun-if-changed={}", "../pins.toml");
    }

    // Link arguments
    //
    {
        let link_args: Vec<&str> = vec!(
            "-Tlinkall.x",
            "-Tdefmt.x"     // required by 'defmt'
        );

        for s in link_args {
            println!("cargo::rustc-link-arg={s}");
        }
    }

    Ok(())
}
