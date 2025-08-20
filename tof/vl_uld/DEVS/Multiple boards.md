# Multiple boards

## Power budget

You can connect multiple SATEL boards to the same MCU, via the I2C bus.

Let's see, how many can be powered without an external power source.

**Facts**

||mA|V|mW|
|---|---|---|---|
|**Drains**|
|&nbsp;&nbsp;ESP32-C6-DevKitM1|38mA (radio off) .. 189mA (BLE sending at 9.0 dBm)<sup>`|1|`</sup>|3.3|<nobr>124 .. 624</nobr>|
|&nbsp;&nbsp;SATEL board|130mA (AVDD + IOVDD; 50 + 80)|3.3|429|
|&nbsp;&nbsp;external pull-ups|2k2: 1.5mA x 2 = 3mA|3.3|10|
|**Source**|
|&nbsp;&nbsp;Power available from a USB 2.0 port (Raspberry Pi 3B)|500mA|5.0|2500|

For four SATEL boards:

- no radio: 124 + 4*429 + 10 = 1850 mW < 2500 mW
- BLE sending: (above) + 500 = 2350 mW < 2500 mW

This seems to indicate we should be able to scan on four SATEL boards, and send on BLE, simultaneously. (If that were not the case, distance scans could be paused for the duration of BLE sends, or the sensors powered separately.)

>Note: The above calculation is based on peak values. ST.com datasheet itself marks "continuous mode" actual consumption as 313mW, and by using "autonomous mode" (as we do in the code), the value drops below 200 (depends on the frame rate). This means powering even up to eight boards could be a thing, but needs to be confirmed by measurements.

<small>
`|1|`: [ESP32-C6-MINI-1 & MINI-1U Datasheet v1.2](https://www.espressif.com/sites/default/files/documentation/esp32-c6-mini-1_mini-1u_datasheet_en.pdf) > 5.4 "Current consumption characteristics" <br />
`|2|`: [VL53L5CX Datasheet: DS13754 - Rev 13](https://www.st.com/resource/en/datasheet/vl53l5cx.pdf) > 6.4 "Current consumption"
</small>

