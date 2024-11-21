use std::{
    env,
    fs,
    process::Command,
};

fn main() {
    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    if env::var("__CFBundleIdentifier").is_ok() {
        return;
    }

    // Expose 'OUT_DIR' to an external (Makefile) build system
    {
        const TMP: &str = ".OUT_DIR";

        let out_dir = env::var("OUT_DIR")
            .expect("OUT_DIR to have a value");

        fs::write(TMP, out_dir)
            .expect(format!("Unable to write {TMP}").as_str());
    }

    // Show a warning if building for a non-tested target
    {
        const TESTED_ON: &[&str] = ["esp32c3", "esp32c6"].as_slice();

        // Pick the current MCU.
        //
        // $ grep -oE -m 1 '"esp32(c3|c6)"' Cargo.toml | cut -d '"' -f2
        //  esp32c3
        //
        let mcu: String = {
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

        if !TESTED_ON.contains(&mcu.as_str()) {
            println!("cargo:warning=Not tested on chip: '{}'", &mcu);
        }
    }
}

