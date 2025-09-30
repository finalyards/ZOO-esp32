# Background

### Web Bluetooth API

A draft of a standard. 

It's never made it to an actual standard, and likely never will. It "all" depends on Apple, which doesn't seem bothered to support it, in (mobile) Safari browsers. 

Works in:

- Chrome
- Edge
- Opera
- Samsung Internet

For iOS devices, there are [third party applications](https://apps.apple.com/us/app/bluefy-web-ble-browser/id1492822055) that implement it.

>Because of this weird status, the best introductions for the API come as Chrome for Developers articles ([here from 2015](https://developer.chrome.com/docs/capabilities/bluetooth)).

The API is really simple. Despite its name, it *doesn't* cover Bluetooth "classic" - only the BLE GATT abstraction of servers > services > characteristics. But that's enough for our/your needs! ☀️


### Bluetooth Classic vs. BLE

The BLE protocol is independent of the "Bluetooth Classic" stack (which continues to co-exist with it). It is intended for **fitness**, **home automation** and **internet-of-things** use cases, i.e. anywhere where battery powered devices with non-frequent charging opportunities abound.

While Bluetooth Classic keeps the radio powered all the time, BLE has it on only in certain communication windows. This saves energy, and is the "Low Energy" in its name.

Some devices (most of Espressif's and all of Nordic Semiconductor's) only support BLE, not the classic profiles. Likewise, some libraries are BLE only.

<!-- #blah
Writing this, first Bluetooth 6.0 devices have just come to the market (iPhone 17, iPhone Air line-up). These support:

- distance measurement (channel sounding)
- BLE Audio (enhanced hearing aids)
-->

