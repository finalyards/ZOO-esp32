include!("build_snippets/pins.in");  // process_pins()

const PINS_OUT_FN: &str = "pins_snippet.in";

// Note!
//  - Do NOT declare 'rerun-if-changed' files, since handling the TOML is not all we do:
//      exposing 'OUT_DIR' needs to be run for every build!
//
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    const IDE: bool = option_env!("__CFBundleIdentifier").is_some();

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
    //
    // Note: disabled for IDE, to allow for VM development.
    if !IDE {
        use std::fs;
        const TMP: &str = ".OUT_DIR";

        fs::write(TMP, &out_dir)
            .expect(format!("Unable to write {TMP}").as_str());
    }

    //---
    // Turn 'pins.toml' -> '{output dir}/pins_snippet.in’
    {
        use std::{
            fs,
            path::{PathBuf}
        };

        let toml = include_str!("./pins.toml");    // "argument must be a string literal" (i.e. 'PINS_TOML' won't work)
        let snippet: String = process_pins(toml, &mcu)
            .unwrap();

        let fn_ = PathBuf::from(&out_dir).join(PINS_OUT_FN);

        fs::write(fn_, snippet)
            .expect(format!("Unable to write {{ fn_.display() }}").as_str());
    }

    // ok
}
