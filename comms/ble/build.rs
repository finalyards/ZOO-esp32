use std::{
    env,
    fs,
};

fn main() {
    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    if std::env::var("__CFBundleIdentifier").is_ok() {
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
}
