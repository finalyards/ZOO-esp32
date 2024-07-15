fn main() {
    // make some stuff
    //
    let st = std::process::Command::new("make")
        .arg("tmp/vendor_uld.a")    // ULD C library
        .arg("src/uld_raw.rs")      // generate the ULD Rust bindings
        .output()
        .expect("could not spawn `make`")   // shown if 'make' not found on PATH
        .status;

    st.success() || panic!("Running 'make' failed");    // shown if 'make' returns a non-zero error code

    // Link with 'tmp/vendor_uld.a'
    println!("cargo:rustc-link-search=tmp");
    println!("cargo:rustc-link-lib=vendor_uld");
}
