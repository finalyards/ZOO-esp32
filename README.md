# Embedded Zoo - Sensors and Actuators 

## ..for [Rust](https://www.rust-lang.org) and [Embassy](https://embassy.dev).

<!-- tbd. Zoo picture & styling -->

It's quite elaborate to take in new sensors to a project. It's about:

- selecting the right part(s)
   - considering availability, price, features
- reading the documentation
- ensuring drivers fit
- learning the quirks that are not necessarily documented, anywhere

This repo covers various sensors (things that measure stuff) and actuators (things that move stuff), interesting to its author, and provides reliable, maintained Rust bindings to them.

>**Why Embassy?**
>
>Rust provides `async/await` support, similar to what you might know from the .NET or JavaScript ecosystems. Embassy is an executor for such `async` functions; and an ecosystem of other, related stuff.
>
>In short, `async` is GREAT!!! Making async code allows concurrency to be coded and understood as if it was linear. This is a huge boost to speed of coding and maintainability of any embedded, non-trivial project. Thus, Embassy has its place as a fundamental component of this repo.


## Menu

||folder|what is it?|stability|comments|
|---|---|---|---|---|
|**Comms**|
|![](.images/about/ble.png)|[`comms/ble`](comms/ble/README.md)|Working as custom Bluetooth (BLE) service|alpha||
||[`comms/extras/ble-web-app`](comms/extras/ble-web-app/README.md)|Web app for interfacing with the sample BLE service|--||
|**Development kits**|
|![](.images/about/devkit.png)|[`devkit/rgb`](devkit/rgb/README.md)|RGB LED|WIP||
||[`devkit/button`](devkit/button/README.md)|Button|--||
|**DC&nbsp;motor&nbsp;controllers&nbsp;(brushed)**|
|![](.images/about/drv8871.png)|[`dc-motors/drv8871`](dc-motors/drv8871/README.md)|Controller for brushed (simple) DC motors - 6.5..45V, 3.6A max|WIP||
|**Time of flight** = distance sensors|
|![](.images/about/vl53l5cx_uld.png)|[`tof/vl53l5cx_uld`](tof/vl53l5cx_uld/README.md)|Time-of-flight distance sensor; 4x4 (60Hz) or 8x8 (15Hz), up to 400cm|beta||
|*tbd.* Brushless motor controllers (VESC)|


### Folder structure

Each subfolder contains a structure similar to this:

```
built-in/
├── Cargo.toml      # Cargo build file
├── Makefile.dev    # dev helper (optional)
├── README.md       # you-know
├── [WIRING.md]     # wiring of peripherals to MCU
├── [pins.toml]     # configuration of MCU pins
├── build.rs        # additional build details
├── examples
│   └── button.rs   # examples you can run (if HW is properly set up)
├── [src]           # library code (not every folder has it)
└── set-target.sh   # (link to a) tool to switch between MCU's
```

By keeping the folder structure similar, getting to speed with a new kind of sensor/actuator should be as swift as possible. The steps should be familiar.


### MCU coverage

The repo **focuses on ESP32 series MCU's**, but this is mainly so that the stated support remains maintained and tested. **If you are ready to take on maintenance for other MCU's, let the author know**. The Rust / Embassy ecosystem, as such, provides the possibility to keep the repo *very* MCU independent. That alone is great. 🎉🎉🎈🎈

|MCU|dev board|support level|notes|
|---|---|---|---|
|ESP32&#x2011;C3|[ESP32&#x2011;C3-DevKitC-02](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c3/esp32-c3-devkitc-02/user_guide.html)|used regularly|See below|
|ESP32&#x2011;C6|[ESP32-C6-DevKitM-01](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)|used regularly|No problems!|

<!-- #hidden
|(ESP32)|[Adafruit ESP32 Feather V2](https://www.adafruit.com/product/5400)|2nd tier|ESP32 code is in a branch that's updated *on request*.<br />Rust support for Xtensa (which this one is) still needs a separate `espup` utility, unlike RISC V targets, and `stable` Rust cannot be used on it. This is only an initial speed bump, however. Instructions for setting up the toolchain are in the particular branch.<br />*Doesn't have built-in JTAG* support so you'll need to purchase an adapter, e.g. [ESP-Prog](https://docs.espressif.com/projects/esp-iot-solution/en/latest/hw-reference/ESP-Prog_guide.html), for debug logging.|
-->

>Note: The repo does debug logs using [defmt](https://defmt.ferrous-systems.com) which means JTAG support is essential, for any development work.

#### ESP32-C3 considerations!!

- The chip does provide a built in USB/JTAG circuitry, but the suggested devkit doesn't have a connector for it. You can solder one, or (perhaps a better approach?), attach a cable to breakout pins on a breadboard. 

	*tbd. image pending*

- I2C functionality - and possibly other time sensitive functionality - is known to suffer from a JTAG *specification issue* that affects `probe-rs`'s ability to interface with the chip. If you need long I2C communications, ESP32-C6 is the better chip to target.

	Note: The problem only involves debugging/logging over JTAG. You can quite well do long I2C transactions in production.

<!-- remove? / #hidden
#### ESP32 considerations!!

The support will be available as a separate branch, because it needs more changes than simply changing the `target` (`.cargo/config.toml`) and `feature` (`Config.toml`) that our chip selection script manages.
-->

## Changing the target

Projects such as `esp-hal` use Cargo [features mechanism](https://doc.rust-lang.org/cargo/reference/features.html) for selecting the target MCU (e.g. `esp32c3` vs. `esp32c6`). While this seems to be the norm, this repo deviates from it.

The main reason is philosophical. Features should be used for - well - features of the code base, and they should be cumulative (by the Rust recommendations). MCU features, on the other hand, are exclusive.

**So what do we do?**

There are `.config/cargo.toml` files under certain folders in the repo. These decide the MCU chip *for the particular folder* and its subfolders.

This means that you can, for example:

- use `dc-motors` code, targeting ESP32-C3, while..
- using `tof` code, targeting ESP32-C6

This at least matches the way the author works; there's always a particular breadboard connected with such software.

To change the folder's MCU target, 

```
$ cd dc-motors
$ $ ./set-target.sh 
1) esp32c3
2) esp32c6
Pick your target: 1

Going to edit:
- .cargo/config.toml
- ./drv8871/Cargo.toml

The edit is safe. You can see the changes made by 'git diff' (and revert by 'git restore --staged .cargo/config.toml ./drv8871/Cargo.toml').

Proceed? (Y/n) Y

Files '.cargo/config.toml' and './drv8871/Cargo.toml' now using:

   MCU:    esp32c3
   TARGET: riscv32imc-unknown-none-elf

Please 'cargo build' or 'cargo run', as usual.
```

You can see the change by `git diff`, i.e. the selected (default) MCU type gets stored in the version control.

This approach is a bit intrusive, but having used it for a while the author does prefer it over the `features` approach, at least for the case of the Zoo (which is full of examples). For publishing a library, using the feature mechanism is likely preferrable (or only working solution).


## Requirements (hardware)

- One of the dev kits mentioned above
- The necessary sensors (see particular subfolder)
- Breadboard
- Wires

Each sensor's subsection has a `WIRING.md` that shows suggested wiring. You can change the pin layout by editing `pins.toml` files found in each subfolder.

### The computers setup

![](.images/layout.png)

The repo can be used in two ways, depending on where `probe-rs` is located:

- **`probe-rs` remotely**

	This is the recommended approach, pictured above. You have a separate computer for interfacing with the MCU, and connect to its `probe-rs` tool over `ssh`. Instructions for installation are provided below.

- **`probe-rs` over USB/IP (for WSL2)**

	This is useful if you have a single (Windows) laptop. Here, USB/IP is used to bring the host's USB port to the VM (WSL2), and `probe-rs` runs within your development VM. 
	
Since the project files simply refer to `probe-rs`, either of these approaches works. With the WSL2 approach, you sacrifice *galvanic isolation*, i.e. your electronics are directly connected to the laptop. Consider using a [USB Isolator](https://www.triosoft.fi/p8024-delock-usb-isolator-with-5-kv-isolation-fi.html). `|2|`

<small>
`|2|`: The author would like to hear any experience on use of USB isolators; any products you can recommend, perhaps?
</small> 	

*There are no instructions on setting up the USB/IP. Ask the author...*

---

Let's get back to the default setup.

- Code editing happens on a host (Mac), using an IDE ([Rust Rover](https://www.jetbrains.com/rust/))
- Compilation happens in a virtual machine (using [Multipass](https://multipass.run) for this); the whole Rust and embedded toolchain *only needs to be installed within here*.
- Hardware devices (MCU + sensors) are connected to *a Raspberry Pi* that runs `ssh` and has [`probe-rs`](https://probe.rs) installed.

>Note: Due to using WLAN, the software development and hardware setups are fully air-gapped from each other.

<p />

>Note: Originally, the author used USB/IP for the setup. This, however, leads to very slow flash times.

### Shortcomings of Multipass

It's maintained by Canonical, but on somewhat limited resources. Some other virtualization product might suit you better, if you are a company. Also, it doesn't have USB pass-through but since we anyways prefer *physical isolation* from the MCU, that's not really an issue.

>Note! Since 1.14.0, the author has found Multipass to be somewhat more fragile than before! This is, however, under control, and should not keep you from using the solution. More info is available in the [`mp´](https://github.com/akauppi/mp) repo.

<!-- disabled
#### Running on a single computer?

Fully possible. You just point `ssh` to point to your host, from the VM, and have `probe-rs` installed on the host.

Again, the benefit is that your Rust / embedded toolchains are sandboxed. Only `probe-rs` remains on the host, and is easy to install and update there.


#### Windows + WSL2

Should probably write a whole new section on WSL2... Since it's Ubuntu to begin with, and sandboxed (kind of; not as much as Multipass), the author **only uses WSL2** and not Multipass on his Windows 10 laptop.

Also, `probe-rs` in this case is installed within the WSL2 VM, and accesses the devkit via USB/IP. USB/IP between a Windows host and WSL2 is *very smooth* and there's no reason for the `ssh` stuff.

This scenario is ideal if you want to work on a single computer, e.g. for taking demos out on the field a Windows laptop is sufficient!
-->

#### Multipass and Windows

For running Multipass on Windows, please note that **only Windows Pro has Hyper-V** hypervisor support. The author hasn't run Multipass on Windows (since WSL2 is there) but if he would, he'd pick a Pro license. You *can* run Multipass with Windows Home, but that involves VirtualBox 👎. Unless.. <sub>[hint](https://github.com/canonical/multipass/issues/1810), [hint#2](https://medium.com/@antongslismith/multipass-with-hyper-v-on-windows-10-home-7fd783d83978)</sub>

---

As you can see, there are some different ways to set up the toolchain.

What's important to take home from this is that

🅰️ The ZOO runs on Ubuntu Linux only. With virtualization, however, this should not limit your choices.

🅱️ `probe-rs` is the tool of choice for interacting with your development kit. The build scripts don't care whether such command leads to USB/IP or a `ssh`-bridged installation.

Good luck! ☀️☀️☀️



## Requirements (software)

These two repos help you to set up the environment discussed above:

- [`mp:/rust+emb`](https://github.com/akauppi/mp)

	Helps you create a Multipass VM that has the tools (Rust, Cargo, `probe-rs-remote`) used in the ZOO.
	
	This doesn't cover exactly *all* the tools, but gives a solid foundation. If additional tools are needed, they are mentioned in the subfolder `README`s (for example, the `tof/vl53l5cx_uld` needs `clang` and `bindgen` CLI to be separately installed).


- [`probe-rs-remote`](https://github.com/lure23/probe-rs-remote)

	The `probe-rs` over `ssh` bridging.
	
	Follow these instructions to set up your Raspberry Pi (or other such secondary computer) that runs `probe-rs`.
	
	>Note: The front side setup is not needed if you use `mp` `rust+emb/prep.sh` - it's already covered there.
	

## Next steps

Visit the subfolders, pick one you'd like to try. The instructions are within their particular `README.md`.

Please please PLEASE [give feedback](https://github.com/lure23/ZOO-esp32/issues) on the GitHub! 

---

Developed on:

```
macOS 15.2
Multipass 1.15.0
ESP32-C3-Devkit-C02 (revision 0.4) 
ESP32-C6-DevKitM-1
```
