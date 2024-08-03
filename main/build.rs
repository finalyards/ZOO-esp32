use esp_build::assert_unique_used_features;

fn main() {

    // Similar test as in 'esp-hal' itself
    //
    // Note: Without this, we'd still get the error from 'esp-hal'. This just allows us to have
    //      a saying on how it's formulated.
    //
    assert_unique_used_features!(
        "esp32c2", "esp32c3", "esp32c6", "esp32h2"      // tested only on C3, C6
    );

    // These get printed as 'cargo::rustc-link-arg=...'
    #[allow(unused_mut)]
    let mut rlas: Vec<&str> = vec!(
        "-Tlinkall.x"
    );

    //#[cfg(feature = "log2")]
    rlas.push("-Tdefmt.x");     // now always linked; if 'log1' also uses 'defmt'

    /*** #[allow(unexpected_cfgs)]   // otherwise warns if 'xtensa' toolchain isn't installed
    #[cfg(target_arch = "xtensa")]
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
    ***/

    /* tbd. Why would we need these?  Where are they from? :)
    #[cfg(any(feature = "esp32c6", feature = "esp32h2"))
    {
        rlas.push("-Trom_coexist.x");
        rlas.push("-Trom_functions.x");
        rlas.push("-Trom_phy.x");
    } */

    /* disabled (keep)
    #[cfg(feature = "esp-wifi")
    rlas.push("-Trom_functions.x");
    */

    // #screw it - ENABLE IF YOU HAVE ANY PUSHES ABOVE!!!
    rlas.iter().for_each(|s| {
        println!("cargo::rustc-link-arg={}", s);
    });
}
