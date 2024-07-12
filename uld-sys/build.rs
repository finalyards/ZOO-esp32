use std::env;
use std::path::PathBuf;

fn main() {
    // Look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=/path/to/lib");

    let bindings = bindgen::Builder::default()
        // input header we would like to generate bindings for
        .header("wrapper.h")
        // invalidate the built crate whenever any of the included header files change
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // generate the bindings
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the '$OUT_DIR/bindings.rs' file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
