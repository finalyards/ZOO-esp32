# Wiring

This wiring diagram serves both the `vl_api` and `vl_uld` levels.

## Wiring for VL53L8

![](.images/wiring-l8.png)

*Figure 1. Wiring for ESP32-C6 and an L8*


## Wiring for two VL53L5CX sensors

![](.images/wiring-l5cx-2.png)

*Figure 2. Wiring for ESP32-C6 and two L5CX sensors*


## Wires

|wire|comment|
|---|---|
|`INT`|All boards share the same interrupt wire. It's an open drain wire where any of the sensors can pull it down to indicate fresh data. The pulling down stops automatically after 100us (both L8 and L5CX)<sup>`|2|`</sup>.|
|`LPn{01}`|*"Drive this pin to logic 0 to disable the I2C comms."*<sup>`|1|`</sup><p />In particular, the pin *does not have anything to do with the Low Power mode*, despite its name. We use it as a chip select, which it is.|
|`PWR_EN`|Enables the regulators on the SATEL board. To reset the board, drive it low for 10ms.|
|`SCL`, `SDA`|The I2C bus|
|`SPI_I2Cn`|L8 specific: selects between I2C (low) or SPI (high) protocol. Connect to ground.|

<small>
`|1|` [DS13754](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) - Rev 12 - Table 3<br />
`|2|` [UM3109](https://www.st.com/resource/en/user_manual/um3109-a-guide-for-using-the-vl53l8cx-lowpower-highperformance-timeofflight-multizone-ranging-sensor-stmicroelectronics.pdf) - Rev 12 - Chapter 5.3
</small>

<!-- #hidden; not relevant and if were, find a better place
## Using in the `vl_uld` sub-project

>[!NOTE]
>
>You don't need to run the `vl_uld` examples (and thus, be concerned with wiring there), unless you do development of that library. If you just use it as an artefact, you can skip this section.

The ULD side is simpler than the `vl_api`; the wiring is planned so that you **do not need to change physical routing** if switching between the two projects. However, here are some details it's good to be aware of:

- `LPn` wouldn't be needed for ULD (just using a single board), but if enabled in `pins.toml`, it's driven also there.
- `SYNC` pin is not used (it's an input)

The rest of the pins work the same.
-->

## Need of pull-up resistors

In the wiring diagrams, there are no pull-up resistors. This is because the SATEL boards provide such, by default. 

To aim for faster I2C speeds, and/or to increase the number of boards on the bus, you need to understand the role of these pull-ups.


### SATEL-VL53L8

The L8 SATEL has two "level translator" circuits between the actual sensor (marked "DUT" in the vendor schematics) and the host. These level translators (PI4ULS3V204 to be precise<sup>`|1|`</sup>) have integrated 10kΩ pull-ups for all their signals. This includes the SDA and SCL.

10kΩ is not ideal for I2C, but it works.

|freq|requirement|
|---|---|
|50k|stable|
|100k||
|400k||
|1M||


This means that e.g. the I2C bus is isolated from the sensors themselves. 

What matters are the PI4ULS3V204 <sup>`|1|`</sup> level translators. They have "integrated 10kΩ pull-up resistors" at each pin, which seems enough for a single board operation at <font color=red>XXX</font> kHz.

When you add more boards on the same bus, the pull-up strengthens (its resistance reduces). I2C bus is recommended to have 330..1kΩ of resistance; this means we could have 10 boards and still be within the specs.

<small>
`|1|`: [datasheet](https://www.mouser.com/datasheet/2/115/PI4ULS3V204-1108185.pdf)
</small>





If you use more than two boards, you'll need to **disable some of the pull-up resistors**. See the SATEL board schema<sup>`|2|`</sup>, and notice that there are following pull-ups on each mini-board:

||ohm|
|---|---|
|`INT`|47k|
|`LPn`|47k|
|`SCL`|2.2k|
|`SDA`|2.2k|
|`SYNC` (L8 only)|47k|

These values are such that use of two unmodified boards still works. "Somewhere"<sup>`|*|`</sup> it was mentioned that I2C pull-ups above 1k should be fine. This means if you were to add a third board, you likely need to solder off `SB5` and `SB7` on at least one board. 

Do mark the boards that have received such modification, for your own good!!

<small>
`|2|` [PCB4109A, version 12, variant 00B](https://www.st.com/resource/en/schematic_pack/pcb4109a-00b-sch012.pdf) (ST.com; 2021; PDF 2pp.)

`|*|`: *the author regrets not finding the source*</small>
</small>
