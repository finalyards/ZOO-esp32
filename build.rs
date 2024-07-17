
fn main() {

    // These get printed as 'cargo::rustc-link-arg=...'
    let mut rlas = vec!(        // tbd. could be a Map, to avoid duplicates
        "-Tdefmt.x",
        "-Tlinkall.x"
    );

    #[cfg(target_arch = "xtensa")]  // tbd. actual string is wrong
    {
        panic!("Not prepped for Xtensa");

        // If Xtensa (disabled; NOT TESTED)
        /***
        #rustflags = [
        #  #  # GNU LD
        #  #  "-C", "link-arg=-Wl,-Tlinkall.x",
        #  #  "-C", "link-arg=-nostartfiles",
        #  #
        #  #  # LLD
        #  #  # "-C", "link-arg=-Tlinkall.x",
        #  #  # "-C", "linker=rust-lld",
        #]
        ***/
    }

    if cfg!(feature = "esp32c6") || cfg!(feature = "esp32h2") {
        rlas.push("-Trom_coexist.x");      // println!("cargo::rustc-link-arg=-Trom_coexist.x");
        rlas.push("-Trom_functions.x");    // println!("cargo::rustc-link-arg=-Trom_functions.x");
        rlas.push("-Trom_phy.x");          // println!("cargo::rustc-link-arg=-Trom_phy.x");
    }

    /* disabled (keep)
    if cfg!(feature = "esp-wifi") {
        rlas.push("-Trom_functions.x");    // println!("cargo::rustc-link-arg=-Trom_functions.x");
    }*/

    rlas.iter().for_each(|s| {
        println!("cargo::rustc-link-arg={}", s);
    })
}
