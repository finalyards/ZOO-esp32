# Wiring

|SATEL|ESP32|comments|SATEL|
|---|---|---|---|
|INT|---|"Open drain", ___ on interrupt; "47 kΩ pullup resistor to IOVDD required"|
|I2C_RST|---???|Active high. Toggle `0->1->0` to reset the I2C target. "Connect to GND via 47 kΩ resistor."|
|SDA|GPIO4|same pin as in a `esp-hal` I2C example; "2.2 kΩ pullup resistor to IOVDD required"|
|SCL|GPIO5|-''-|
|LPn|??|Chip enable, active high. "47 kΩ pullup resistor to IOVDD is required"|Does SATEL pull it up?|
|PWR_EN|--|22K pull-up to AVDD| 
|AVDD|5v|
|IOVDD|3v3|Supply for digital core and I/O||
|GND|Gnd|

>For detailed description of the pins, see [`[1]`](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) table 3 ("VL53L5CX pin description").
>
>`PWR_EN` is a pin specific to the larger part (5v -> 3v3 conversion) of the SATEL board.

**Quotes**

*"[...] SATEL has already the pull ups while the breakout board doesn't."* <sub>[source](https://community.st.com/t5/imaging-sensors/vl53l5cx-satel-won-t-respond-to-i2c/td-p/597080)</sub>

- i.e. should be okay to start without *any* pullups?

	<!-- #whisper detached SATEL has 2.2kOhm between both SDA-3v3 and SCL-3v3 -->

*"EDIT got it working by pulling up the PWREN line. Tho this was not written in the datasheet of the satel and [...]"* <sub>[source](https://community.st.com/t5/interface-and-connectivity-ics/i-cannot-see-the-vl53l5cx-device-on-the-i2c-bus-i-m-tried-it/td-p/231586)</sub>

- the Other Person had used "22K pull up to AVDD (5V)", so likely we do the same?

[This example](https://github.com/stm32duino/VL53L5CX/blob/main/README.md) (Arduino library) has connected the pins like mentioned in the Nucleo document. But that seems overkill - we'd rather connect as few to active GPIO as possible (leaving that to the application engineer).

Base on [this ESP-focused library](https://github.com/RJRP44/VL53L5CX-Library) we should be able to get going with:

- pull-ups (2.2K) in both `SDA` and `SCL`
- pull-up (47K) in `LPn` (he's tied it up, but vendor docs demand a resistor)

>In particular, the following had been left unconnected: `INT`, `I2C_RST`, `PWR_EN`

## Other uses




## Open issues

>*tbd. Write here uncertainties about pull-ups etc.*

## References

- `[1]`: [VL53L5CX Product overview](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) (ST.com DS13754 - Rev 12 - April 2024; PDF 38pp.)
