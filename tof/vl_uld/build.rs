/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
use anyhow::*;

include!("../build_snippets/pins.in");  // process_pins()

// 'X' is either 5 or 8
const X: u8 =
         if cfg!(feature = "vl53l8cx") { 8 }
    else if cfg!(feature = "vl53l5cx") { 5 }
    else { 0 };     // an error will be produced, later

#[allow(non_snake_case)]
const CONFIG_H_NEXT: &str = "tmp/config58.h.next";

const PINS_OUT_FN: &str = "tmp/pins_snippet.in";

/*
* Note: 'build.rs' is supposedly run only once, for any 'examples', 'lib' etc. build.
*
* References:
*   - Environment variables set
*       -> https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
*/
fn main() -> Result<()> {
    use std::{
        env,
        fs,
        process::Command
    };

    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    #[allow(non_snake_case)]
    let IDE_RUN = env::var("__CFBundleIdentifier").is_ok();

    // If IDE runs, terminate early.
    if IDE_RUN { return Ok(()) };

    // DEBUG: Show what we know about the compilation.
    //
    // <<
    //   CARGO_CFG_TARGET_FEATURE=c,m
    //   CARGO_FEATURE_{..feature..}=1
    //   LD_LIBRARY_PATH=/home/ubuntu/VL53L5CX_rs.cifs/vl53l5cx_uld/target/release/deps:/home/ubuntu/VL53L5CX_rs.cifs/vl53l5cx_uld/target/release:/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib
    //   RUSTUP_TOOLCHAIN=stable-x86_64-unknown-linux-gnu
    //   TARGET=riscv32imc-unknown-none-elf
    // <<
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
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
        // In the nature of Rust features being combinable, several 'targets_per_zone{2..4}' are allowed, the
        // grandest of them making the call. = we don't need to check for sanity.
        //
        //"targets_per_zone_2",
        //"targets_per_zone_3",
        //"targets_per_zone_4",

        //R "range_sigma_mm" relates to "distance_mm"
        //R #[cfg(all(feature = "range_sigma_mm", not(feature = "distance_mm")))]
        //R println!("cargo:warning=Feature 'range_sigma_mm' does not make sense without feature 'distance_mm' (which is not enabled)");

        // One sensor type, per project
        #[cfg(not(any(feature = "vl53l5cx", feature = "vl53l8cx")))]
        panic!("📍 Must enable feature: {{vl53l5cx|vl53l8cx}}");

        #[cfg(all(feature = "vl53l5cx", feature = "vl53l8cx"))]
        panic!("📍 Must enable ONLY one of features: {{vl53l5cx|vl53l8cx}}");
    }

    // EXAMPLE config sanity checks.
    //
    // Note: we need to steer clear of other projects using this code for _their_ examples. Which we can.
    if env::var("EXAMPLE").is_ok() {
        let pwd = env::var("PWD").expect("to have 'PWD'");
            // "/home/ubuntu/.../tof/vl_uld" | "/home/ubuntu/.../tof/vl_api" | ...

        if pwd.ends_with("/vl_uld") {
            #[cfg(not(any(feature = "run_with_espflash", feature = "run_with_probe_rs")))]
            panic!("📍 Must enable feature: run_with_{{espflash|probe_rs}}");

            #[cfg(all(feature = "run_with_espflash", feature = "run_with_probe_rs"))]
            panic!("📍 Must enable ONLY one of features: run_with_{{espflash|probe_rs}}");
        }
    }

    // Expose 'OUT_DIR' to an external (Makefile.dev) build system
    {
        const TMP: &str = ".OUT_DIR";

        let out_dir = env::var("OUT_DIR")
            .expect("OUT_DIR to have a value");

        fs::write(TMP, out_dir)
            .expect(format!("Unable to write {TMP}").as_str());
    }

    //---
    // Turn 'pins.toml' -> 'tmp/pins_snippet.in’
    //
    // Note: It's arguable, should the 'pins.toml' be in '../pins.toml' (it is common to us, and
    //      the 'vl_api/examples'), or just in the 'vl_api/examples'. It's also connected with
    //      'WIRING.md' and 'build_snippets/'. Ideas, opinions? #feedback
    {
        #[cfg(feature="vl53l8cx")]
        const SENSOR_ID: &str = "vl53l8";   // without "cx"
        #[cfg(feature="vl53l5cx")]
        const SENSOR_ID: &str = "vl53l5cx";

        const PINS_TOML: &str = "../pins.toml";

        let toml = include_str!("../pins.toml");    // "argument must be a string literal"
        let snippet: String = process_pins(toml, &board_id, SENSOR_ID)?;

        let fn_ = PINS_OUT_FN;

        fs::write(fn_, snippet).with_context(
            || format!("Unable to write {fn_}")
        )?;

        // Change in TOML retriggers a build
        println!("cargo::rerun-if-changed={}", PINS_TOML);
    }

    //---
    // Create a C config header, reflecting the Rust-side features required.
    //
    // MUST BE BEFORE running the Makefile.
    //
    // Note: Never run this on IDE builds - the features a person selects in the IDE UI don't necessarily match
    //       what the real builds will be about.
    {
        use itertools::Itertools;
        let mut defs: Vec<String> = vec!();

        macro_rules! add {
            ($x:expr) => { defs.push($x.into()); }
        }
        // ^-- Practically the same as:
        // let add = |s: dyn Into<String>| { defs.push(s.into()) };    // does not compile

        // Output-enabling features (in Rust, we have them enabling; in C they are disable flags). Same thing.
        //
        // First group: metadata of the sensor (DIMxDIM, regardless of targets)
        //
        #[cfg(not(feature = "ambient_per_spad"))]
        add!("DISABLE_AMBIENT_PER_SPAD");
        #[cfg(not(feature = "nb_spads_enabled"))]
        add!("DISABLE_NB_SPADS_ENABLED");

        // Always build C level with:
        //      '.nb_target_detected'
        //      '.target_status'
        //
        // Needing those for data validation: splitting measurements to 'Valid', 'SemiValid', 'Invalid'
        // enums (in results_data.rs).
        //
        //add!("DISABLE_NB_TARGET_DETECTED");
        //add!("DISABLE_TARGET_STATUS");

        // Second group: data and metadata (DIMxDIMxTARGETS)
        //
        // Note: could make 'distance_mm' a feature, but it would unnecessarily complicate the source:
        //      - it's now merged with '.target_status' and '.nb_target_detected' handling; returning
        //        a "measurement" without actual measurements would be weird.
        //      - also, presumably most use cases do want to get the distance (if they don't, it hardly
        //        matters they would just get it and ignore it)
        //
        //R#[cfg(not(feature = "distance_mm"))]
        //Radd!("DISABLE_DISTANCE_MM");     // always compile

        #[cfg(not(feature = "range_sigma_mm"))]
        add!("DISABLE_RANGE_SIGMA_MM");
        #[cfg(not(feature = "reflectance_percent"))]
        add!("DISABLE_REFLECTANCE_PERCENT");
        #[cfg(not(feature = "signal_per_spad"))]
        add!("DISABLE_SIGNAL_PER_SPAD");

        // 'motion_indicator' support is not implemented; always disable in C
        add!("DISABLE_MOTION_INDICATOR");

        // Vendor docs:
        //      "the number of target[s] per zone sent through I2C. [...] a lower number [...] means a lower RAM
        //      [consumption]."
        //
        // NOTE: In the nature of Rust features being *combinable* (the merger matters; features should not be
        //      exclusive), we use the *largest* given feature. If there are none, 1.
        //
        const TARGETS: usize =
                 if cfg!(feature = "targets_per_zone_4") { 4 }
            else if cfg!(feature = "targets_per_zone_3") { 3 }
            else if cfg!(feature = "targets_per_zone_2") { 2 }
            else { 1 };     // always one target

        defs.push(format!("NB_TARGET_PER_ZONE {TARGETS}U"));

        // Write the file. This way the last 'cargo build' state remains available, even if
        // 'make' were run manually (compared to passing individual defines to 'make');
        // also, it keeps the 'Makefile' simple.
        //
        let contents = defs.iter()
            .map(|s| format!("#define VL_{s}"))
            .join("\n");

        fs::write(CONFIG_H_NEXT, contents)
            .expect( &format!("Unable to write {CONFIG_H_NEXT}") );
    }

    // make stuff
    //
    let st = Command::new("make")
        //.arg("-B")
        .arg( format!("tmp/libvendor_uld{X}.a") )    // ULD C library
        .arg( format!("tmp/uld_raw{X}.rs") )      // generate the ULD Rust bindings
        .arg( format!("X={X}") )
        .output()
        .expect("to be able to launch `make`")   // shown if 'make' not found on PATH
        .status;

    if !st.success() {
        // Remove "tmp/config[.next].h" on failure. This tries to avoid an awkward situation where
        // the developer needs to remove them, themselves.
        //
        // tbd. what's the right thing to do? #undecided
        //
        //fs::remove_file(CONFIG_H)?;
        //fs::remove_file(CONFIG_H_NEXT)?;

        panic!("[ERROR!]: Running 'make' failed. \
            SUGGESTION: run 'make manual X={X}' on the command line to see more error information. \
        ");
    }

    // Link arguments
    //
    {
        for s in [
            "-Tlinkall.x",
            "-Tdefmt.x"     // required by 'defmt'
        ] {
            println!("cargo::rustc-link-arg={}", s);
        }

        if std::env::var("TEST").is_ok() {  // 'cargo test' run
            println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
        }
    }

    println!("cargo:rustc-link-search=tmp");
    println!("cargo:rustc-link-lib=static=vendor_uld{}", X);

    // Change in Makefile re-triggers a build
    println!("cargo::rerun-if-changed={}", "Makefile");

    Ok(())
}
