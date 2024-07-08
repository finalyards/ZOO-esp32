# def

This folder has originally been done using `cargo generate`. Some tuning has been made (and some obviously still remains!).

## Todo

```
$ tree -I target def
def
├── Cargo.lock
├── Cargo.toml
├── LICENSE-MIT
├── README.md
├── build.rs
├── rust-toolchain.toml
└── src
    └── main.rs
```

**`.cargo/config.toml`**

The `cargo generate esp-rs/esp-template` asks for the target board type, and makes shorter, customized, output, in return.

The `esp-hal/examples` corresponding file covers all ESP-32 targets.

We might want to keep the repo bendy to a few (or even all?) ESP-32 variants, instead of the lock-in to just one...

<details><summary>Contents by `esp-template` (for C3)<summary>

```
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash --monitor"

[env]
ESP_LOGLEVEL="DEBUG"

[build]
rustflags = [
  # Required to obtain backtraces (e.g. when using the "esp-backtrace" crate.)
  # NOTE: May negatively impact performance of produced code
  "-C", "force-frame-pointers",
]

target = "riscv32imc-unknown-none-elf"

[unstable]
build-std = ["core"]
```
</details

<details><summary>Contents in `esp-hal/examples`</summary>

```
[alias]
esp32   = "run --release --features=esp32   --target=xtensa-esp32-none-elf"
esp32c2 = "run --release --features=esp32c2 --target=riscv32imc-unknown-none-elf"
esp32c3 = "run --release --features=esp32c3 --target=riscv32imc-unknown-none-elf"
esp32c6 = "run --release --features=esp32c6 --target=riscv32imac-unknown-none-elf"
esp32h2 = "run --release --features=esp32h2 --target=riscv32imac-unknown-none-elf"
esp32s2 = "run --release --features=esp32s2 --target=xtensa-esp32s2-none-elf"
esp32s3 = "run --release --features=esp32s3 --target=xtensa-esp32s3-none-elf"

[target.'cfg(target_arch = "riscv32")']
runner    = "espflash flash --monitor"
rustflags = [
  "-C", "link-arg=-Tlinkall.x",
  "-C", "force-frame-pointers",
]

[target.'cfg(target_arch = "xtensa")']
runner    = "espflash flash --monitor"
rustflags = [
  # GNU LD
  "-C", "link-arg=-Wl,-Tlinkall.x",
  "-C", "link-arg=-nostartfiles",

  # LLD
  # "-C", "link-arg=-Tlinkall.x",
  # "-C", "linker=rust-lld",
]

[env]
ESP_LOGLEVEL = "info"
SSID = "SSID"
PASSWORD = "PASSWORD"
STATIC_IP = "1.1.1.1 "
GATEWAY_IP = "1.1.1.1"
HOST_IP = "1.1.1.1"

[unstable]
build-std = ["alloc", "core"]
```
</details>


**`Cargo.toml`**

These sections: 

- What is worth keeping?  What is not?

```
#[profile.dev]
## Rust debug is too slow.
## For debug builds always builds with some optimization
#opt-level = "s"

[profile.release]
codegen-units = 1 # LLVM can perform better optimizations using a single thread
#debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 's'
overflow-checks = false
```

**`rust-toolchain.toml`**

Changed:

```diff
-channel = "nightly"
+channel = "stable"
```

RISC V variants are fine with stable (Jul'24). Eventually, also Xtensa variants will follow (that work is progressing).
