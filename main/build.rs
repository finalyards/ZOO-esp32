use esp_build::assert_unique_used_features;

fn main() {

    // Similar test as in 'esp-hal' itself
    //
    // Note: Without this, we'd still get the error from 'esp-hal'.
    //
    assert_unique_used_features!(
        "esp32c2", "esp32c3", "esp32c6", "esp32h2"      // tested only on C3, C6
    );

    // These get printed as 'cargo::rustc-link-arg=...'
    #[allow(unused_mut)]
    let mut link_args: Vec<&str> = vec!(
        "-Tlinkall.x",
        "-Tdefmt.x"     // something special required by 'defmt'
    );

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
        link_args.push("-Trom_coexist.x");
        link_args.push("-Trom_functions.x");
        link_args.push("-Trom_phy.x");
    } */

    /* disabled (keep)
    #[cfg(feature = "esp-wifi")
    link_args.push("-Trom_functions.x");
    */

    link_args.iter().for_each(|s| {
        println!("cargo::rustc-link-arg={}", s);
    });
}
