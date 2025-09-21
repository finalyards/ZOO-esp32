use anyhow::*;

include!("build_snippets/pins.in");  // process_pins()

const PINS_OUT_FN: &str = "tmp/pins_snippet.in";

fn main() -> Result<()> {
    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    {
        let ide = std::env::var("__CFBundleIdentifier").is_ok();
        if ide { return Ok(()) };
    }

    // Pick the current MCU.
    //
    // $ grep -oE -m 1 '"esp32(c3|c6)"' Cargo.toml | cut -d '"' -f2
    //  esp32c3
    //
    let mcu: String = {
        use std::process::Command;
        let output = Command::new("sh") .arg("-c")
            .arg("grep -oE -m 1 '\"esp32(c3|c6)\"' Cargo.toml | cut -d '\"' -f2")
            .output()
            .expect("'sh' to run");

        // 'output.stdout' is a 'Vec<u8>' (since, well, could be binary)
        //
        let us: &[u8] = output.stdout.as_slice().trim_ascii();
        let x = String::from_utf8_lossy(us);

        x.into()
    };

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
    // Turn 'pins.toml' -> 'tmp/pins_snippet.in’
    {
        use std::fs;
        const PINS_TOML: &str = "./pins.toml";

        let toml = include_str!("./pins.toml");    // "argument must be a string literal"
        let snippet: String = process_pins(toml, &mcu)?;

        let fn_ = PINS_OUT_FN;

        fs::write(fn_, snippet).with_context(
            || format!("Unable to write {fn_}")
        )?;

        // Change in TOML retriggers a build
        println!("cargo::rerun-if-changed={}", PINS_TOML);
    }

    Ok(())
}
