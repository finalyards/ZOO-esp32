# Wiring

This document applies to a version of the [VL53L5CX-SATEL](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html#documentation) development boards marked with "PCB4109A" on the bottom side.

<!-- tbd. image here -->

|SATEL pin|ESP32 pin|comments|SATEL|What can I do with it?|
|---|---|---|---|---|
|INT|---|Active low (open drain)<strike>; 47 kΩ pullup resistor to IOVDD needed.</strike>|Pulled via 47k to IOVDD. Enough to listen to the low pulse. <font color=orange>*(tbd. confirm by testing)*</font>|Make the waiting for a new frame `async` (non-polling).|
|I2C_RST|---|Active high. Toggle `0->1->0` to reset the I2C target. <strike>"Connect to GND via 47 kΩ resistor."</strike>|**Pulled via 47k to IOVDD.** This means the MCU should steer it as an open drain output (pull down).|Reset the I2C side by pulling the pin down (doesn't seem to be needed).|
|SDA|GPIO4|same pin as in a `esp-hal` I2C example; <strike>"2.2 kΩ pullup resistor to IOVDD required"</strike>|Has 2.2K built-in pull-ups for this. If you chain multiple boards, remove extra pull-ups by soldering open `SB5`, `SB7` on the underside of boards 2..n.|Talk with the device.|
|SCL|GPIO5|-''-|-''-|-''-|
|LPn|---|Chip enable, active high. <strike>"47 kΩ pullup resistor to IOVDD is required"</strike>|provides 47K pull-up to IOVDD <sup>[`[3]`]()</sup>|Connect to GND to momentarily disable that chip. Can be used for programming non-default I2C addresses to a certain chip (if you cannot detach them from the bus).|
|PWR_EN|(GPIO0)|47K to IOVDD or drive directly high with a GPIO pin|drives the `CE` (chip enable) of the larger board's regulator <sup>[`[3]`]()</sup>|Control the power cycle of the VC53L5CX chip. Up = powered; Low = off.|
|AVDD|5v|
|IOVDD|3v3|Supply for digital core and I/O||
|GND|Gnd|

>For detailed description of the pins, see [`[1]`](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) table 3 ("VL53L5CX pin description").
>
>`PWR_EN` is a pin specific to the larger part (5v -> 3v3 conversion) of the SATEL board.

**Quotes**

|*"[...] SATEL has already the pull ups while the breakout board doesn't."* <sub>[source](https://community.st.com/t5/imaging-sensors/vl53l5cx-satel-won-t-respond-to-i2c/td-p/597080)</sub>|
|---|
|<strike>i.e. if you use the *larger* board, not SDA or SCL pull-ups are required. If you use the *smaller* board, you shall add those.</strike> This is clear in the Figures 1 and 2 or `[2]`.|
|This is **NOT ACCURATE** on the "PCB4109A" revision. There all the pull-ups are on the side of the *smaller* board.|

|*"EDIT got it working by pulling up the PWREN line. Tho this was not written in the datasheet of the satel and [...]"* <sub>[source](https://community.st.com/t5/interface-and-connectivity-ics/i-cannot-see-the-vl53l5cx-device-on-the-i2c-bus-i-m-tried-it/td-p/231586)</sub>|
|---|
|the Other Person had used "22K pull up to AVDD (5V)". That likely works, but so does pulling up to IOVDD. The schematic for "PCB4109A" [`[3]`]() shows a dimmed (not installed) 47k pull-up to IOVDD. Thus, that seems recommendable.|

**Other users**

[This example](https://github.com/stm32duino/VL53L5CX/blob/main/README.md) (Arduino library) has connected the pins like mentioned in the Nucleo document. But that seems overkill - we'd rather connect as few to active GPIO as possible (leaving that to the application engineer).

Based on [this ESP-focused library](https://github.com/RJRP44/VL53L5CX-Library) we should be able to get going with:

- pull-ups (2.2K) in both `SDA` and `SCL` *(only needed if using the smaller board)*
- pull-up (47K) in `LPn` (he's tied it up, but vendor docs demand a resistor)

In particular, the following had been left unconnected: `INT`, `I2C_RST`, `PWR_EN`

## Wiring multiple boards

*tbd. Do it!*

>Haven't tried this yet, but once I do, I will:
>
>- change boards 2..n I2C addresses by connecting them *individually* to the MCU (no playing with the `LPn` lines), and programming the new address in.
>- disabling the SDA & SCL pull-ups from all but one board, by removing `SB5`, `SB7` solder jumps (*tbd. image; backside of the smaller board*)

<!-- tbd. once done, edit the above -->


## References

- `[1]`: [VL53L5CX Product overview](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) (ST.com DS13754, Rev 12; April 2024)
- `[2]`: [How to setup and run the VL53L5CX-SATEL using an STM32 Nucleo64 board]() (ST.com AN5717, Rev 2; Dec 2021)
- `[3]`: [PCB Schematic VL53L5CX-SATEL](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html#cad-resources) (ST.com; Rev A, ver 012, 2021)

	The schematics of the SATEL board (PCB4109A).

