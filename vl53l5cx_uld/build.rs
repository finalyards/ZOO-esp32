use itertools::Itertools;
use esp_build::assert_unique_used_features;
#[allow(unused_imports)]
use std::{env, fs, fmt::format};

const FN: &str = "tmp/config.h";
const MAKEFILE_INNER: &str = "Makefile.inner";

/*
* Note:
*   There isn't (cargo 1.80.0) *any* way we can differentiate from within here, whether the cargo
*   command was '--lib' or '--example'. The environments, for one, are IDENTICAL, except for a
*   hash in 'OUT_DIR'.
*
*       - makes us need to have an 'EXAMPLES=1' env.var. from the command line, or _not_ need
*         to differentiate between the two..
*
*   Could use that e.g. for enforcing 'defmt' as a default feature for all examples (but keep it
*   optional, for the 'lib').
*
*   EDIT: The above might be missing the point. 'build.rs' might not even *get run* separately
*       for an '--example' build, only for the library.
*
* References:
*   - Environment variables set
*       -> https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
*/
fn main() {
    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    #[allow(non_snake_case)]
    let _IDE_RUN = std::env::var("__CFBundleIdentifier").is_ok();

    /*** disabled
    if !_IDE_RUN {
        // DEBUG: Show what we know about the compilation.
        let tmp = env::vars().map(|(a, b)| format!("{a}={b}")).join("\n");

        fs::write("err.log", tmp)
            .expect("Unable to write a file");
        panic!("!!")
    }
    **/

    // THIS IS NOT POSSIBLE. Here for reference - would allow us to turn "defmt" feature on, when
    // building examples.
    //
    // TRACKed in:
    //  - "Add `example`, `bin` configurations"
    //      -> https://github.com/rust-lang/cargo/issues/14378
    //
    // Work-around: we now ask the user to manually insert '--features=defmt' when running examples.
    //
    //#[cfg_attr(example, cfg(all()))]
    // println!("cargo::rustc-cfg=feature=defmt");

    //---
    // Config sanity checks

    // Pick 1
    assert_unique_used_features!(
        "targets_per_zone_1",   // tbd. 2,3,4
    );

    //---
    // Create a C config header, based on the features from 'Cargo.toml'.
    {
        let mut defs: Vec<&str> = vec!();

        // Output-enabling features (in Rust, we have them enabling; in C they are disable flags). Same thing.
        #[cfg(not(feature = "ambient_per_spad"))]
        defs.push("VL53L5CX_DISABLE_AMBIENT_PER_SPAD");
        #[cfg(not(feature = "nb_spads_enabled"))]
        defs.push("VL53L5CX_DISABLE_NB_SPADS_ENABLED");
        #[cfg(not(feature = "signal_per_spad"))]
        defs.push("VL53L5CX_DISABLE_SIGNAL_PER_SPAD");
        #[cfg(not(feature = "range_sigma_mm"))]
        defs.push("VL53L5CX_DISABLE_RANGE_SIGMA_MM");
        #[cfg(not(feature = "distance_mm"))]
        defs.push("VL53L5CX_DISABLE_DISTANCE_MM");
        #[cfg(not(feature = "reflectance_percent"))]
        defs.push("VL53L5CX_DISABLE_REFLECTANCE_PERCENT");
        #[cfg(not(feature = "motion_indicator"))]
        defs.push("VL53L5CX_DISABLE_MOTION_INDICATOR");

        #[cfg(feature = "use_raw_format")]
        defs.push("VL53L5CX_USE_RAW_FORMAT");

        // In brief,
        //  "the number of target[s] per zone sent through I2C. [...] a lower number [...] means
        //  a lower RAM [consumption]. The value must be between 1 and 4."
        //
        #[cfg(feature = "targets_per_zone_1")]
        defs.push("VL53L5CX_NB_TARGET_PER_ZONE 1U");
        //#[cfg(feature = "targets_per_zone_2")]
        //defs.push("VL53L5CX_NB_TARGET_PER_ZONE 2U");
        //#[cfg(feature = "targets_per_zone_3")]
        //defs.push("VL53L5CX_NB_TARGET_PER_ZONE 3U");
        //#[cfg(feature = "targets_per_zone_4")]
        //defs.push("VL53L5CX_NB_TARGET_PER_ZONE 4U");

        // Write the file. This way the last 'cargo build' state remains available, even if
        // 'make' were run manually (compared to passing individual defines to 'make');
        // also, it keeps the 'Makefile' simple.
        //
        let contents = defs.iter()
            .map(|s| format!("#define {s}"))
            .join("\n");

        fs::write(FN, contents)
            .expect("Unable to write a file");  // note: cannot pass 'FN' here; tbd.
    }

    // make stuff
    //
    let st = std::process::Command::new("make")
        .arg("-f").arg(MAKEFILE_INNER)
        .arg("tmp/libvendor_uld.a")    // ULD C library
        .arg("src/uld_raw.rs")      // generate the ULD Rust bindings
        .output()
        .expect("could not spawn `make`")   // shown if 'make' not found on PATH
        .status;

    assert!(st.success(), "[ERROR]: Running 'make' failed");    // shown if 'make' returns a non-zero

    // Link arguments
    //
    // Note: Is it okay to do this in a lib crate?  We want it to affect at least the 'examples'.
    {
        #[allow(unused_mut)]
        let mut link_args: Vec<&str> = vec!(    // 'mut' in case we wish to conditionally add stuff
            "-Tlinkall.x",
            "-Tdefmt.x"     // required by 'defmt'
        );

        link_args.iter().for_each(|s| {
            println!("cargo::rustc-link-arg={}", s);
        });
    }

    println!("cargo:rustc-link-search=tmp");
    println!("cargo:rustc-link-lib=static=vendor_uld");

    // Allow using '#[cfg(disabled)]' for block-disabling code
    println!("cargo::rustc-check-cfg=cfg(disabled)");
}
