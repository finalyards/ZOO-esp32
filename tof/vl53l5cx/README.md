# `v53l5cx`

Higher level abstraction for the ST.com [VL53L5CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html) sensors.

APIs for using either a single, or multiple such sensors at once.


## Requirements

- Follow the steps in the `../vl53l5cx_uld/README.md` 


## Running examples

### Single board

```
$ cargo build --release --features=single,distance_mm,defmt --example single-emb
```

### Multiple boards

```
$ EMBASSY_EXECUTOR_TASK_ARENA_SIZE=50000 \
  cargo build --release --features=flock,distance_mm,defmt --example many-emb
```

>NOTE!
>
>At the moment, there's [some glitch](https://github.com/embassy-rs/embassy/issues/3537) with `embassy-time`. Use the non-release version for a good run:
>
>```
>$ make -f Makefile.dev md
>[...]
>```

#### Serial output

To see the serial output:

- switch to using the UART USB port (not JTAG) 
- use `espflash` to flash and monitor (`probe-rs` being JTAG-only)

>Obviously, you'll need to reattach the device if using USB/IP: `usbip attach -r [...] -b [...]` 

**Check connection (optional)**

```
$ espflash board-info
[...]
Chip type:         esp32c6 (revision v0.0)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       54:32:04:07:15:10
```

**Flash and run**

```
$ make -f Makefile.dev mds
[...]
espflash flash -f 80mhz --monitor /home/ubuntu/target/riscv32imac-unknown-none-elf/debug/examples/many-emb
[...]
ESP-ROM:esp32c6-20220919
Build:Sep 19 2022
rst:0x1 (POWERON),boot:0xc (SPI_FAST_FLASH_BOOT)
SPIWP:0xee
mode:DIO, clock div:2
load:0x4086c410,len:0xd48
load:0x4086e610,len:0x2d68
load:0x40875720,len:0x1800
entry 0x4086c410
I (23) boot: ESP-IDF v5.1-beta1-378-gea5e0ff298-dirt 2nd stage bootloader
I (24) boot: compile time Jun  7 2023 08:02:08
I (25) boot: chip revision: v0.0
I (29) boot.esp32c6: SPI Speed      : 40MHz
I (33) boot.esp32c6: SPI Mode       : DIO
I (38) boot.esp32c6: SPI Flash Size : 4MB
I (43) boot: Enabling RNG early entropy source...
I (49) boot: Partition Table:
I (52) boot: ## Label            Usage          Type ST Offset   Length
I (59) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (67) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (74) boot:  2 factory          factory app      00 00 00010000 003f0000
I (82) boot: End of partition table
I (86) esp_image: segment 0: paddr=00010020 vaddr=42000020 size=122a4h ( 74404) map
I (109) esp_image: segment 1: paddr=000222cc vaddr=40800000 size=00014h (    20) load
I (110) esp_image: segment 2: paddr=000222e8 vaddr=420122e8 size=19910h (104720) map
I (137) esp_image: segment 3: paddr=0003bc00 vaddr=40800014 size=008e0h (  2272) load
I (138) esp_image: segment 4: paddr=0003c4e8 vaddr=408008f8 size=004b0h (  1200) load
I (143) boot: Loaded app from partition at offset 0x10000
I (148) boot: Disabling RNG early entropy source...
[...]
FlockResults { board_index: 0, res: ResultsData { target_status: [[[Valid, SemiValid(9), Valid, Valid], [Valid, Valid, SemiValid(9), Valid], [Valid, Valid, SemiValid(9), Valid], [Valid, Valid, SemiValid(9), Valid]]], distance_mm: [[[262, 642, 575, 565], [771, 858, 814, 750], [877, 973, 654, 897], [594, 675, 620, 644]]] }, temp_degc: TempC(29), time_stamp: Instant { ticks: 4056733 } }
FlockResults { board_index: 0, res: ResultsData { target_status: [[[Valid, SemiValid(9), Valid, Valid], [Valid, Valid, Valid, Valid], [Valid, SemiValid(9), Valid, Valid], [Valid, Valid, Valid, Valid]]], distance_mm: [[[252, 649, 567, 557], [774, 861, 756, 740], [882, 954, 701, 1481], [627, 684, 611, 687]]] }, temp_degc: TempC(29), time_stamp: Instant { ticks: 4155893 } }
```

The output is in Rust `Debug` streaming. It's similar to JSON, but includes the struct/enum names. This is just an example - you can convert to JSON if you wish.


## References

Please see [`../vl53l5cx_uld/README`](../vl53l5cx_uld/README.md) > `References`.

