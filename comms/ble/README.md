# Bluetooth Low Energy

Exposing an ESP32 device via the *Bluetooth Low Energy* protocol.

**Background**

The BLE protocol is independent of the "Classic" Bluetooth stack (which continues to co-exist with it). Some devices (like ESP32's and all Nordic Semiconductor's only support BLE, not the classic profiles).

The "LE" stack is intended for **fitness**, **home automation** and **internet-of-things** use cases, i.e. anywhere where battery powered devices with non-frequent charging opportunities abound.

>With the advent of BLE 6 (not supported by ESP32 chips, yet; 2024), also home audio enters the application domain, via Bluetooth LE Audio.

## Sample case

![](.images/bluetooth-human-app-80%.png)

*Figure 1. Interacting with an embedded device, using a web app.*

Collecting information and/or controlling a device, via Bluetooth, off a web application. This allows a mobile phone user to wirelessly, and without installing a native application, to work with an embedded product.

Since we control the ESP32 side of things, we can freely affect what kind of Bluetooth profile is used for the link.

## Requirements

An ESP32-C6 or ESP32-C3 devkit (with [JTAG-USB cable added](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/api-guides/usb-serial-jtag-console.html) for C3). No wiring required.

Test this by:

```
$ probe-rs list
The following debug probes were found:
[0]: ESP JTAG -- 303a:1001:54:32:04:07:15:10 (EspJtag)
```

### Debug tooling

Consider installing [nRF Connect for Mobile](https://play.google.com/store/apps/details?id=no.nordicsemi.android.mcp) (Google Play store), or a similar debugging tool on your mobile phone or tablet - and learning to use it.

>The Nordic Semiconductor training material mentioned in the `Recommended Training material` section covers this tool.


## Running

### Launching the Bluetooth device

```
$ DEFMT_LOG=debug cargo run --release --features=defmt --example bas-emb
   Compiling comms-ble v0.0.0 (/home/ubuntu/ZOO.comms/comms/ble)
    Finished `release` profile [optimized + debuginfo] target(s) in 8.39s
probe-rs run --log-format '{t:dimmed} [{L:bold}] {s}' /home/ubuntu/target/riscv32imc-unknown-none-elf/release/examples/trouble-emb
      Erasing ✔ 100% [####################] 384.00 KiB @  76.86 KiB/s (took 5s)
  Programming ✔ 100% [####################] 166.78 KiB @   1.46 KiB/s (took 2m)                                                                                                   Finished in 113.91s
<time> [INFO ] Let's go!
<time> [INFO ] esp-wifi configuration EspWifiConfig { rx_queue_size: 5, tx_queue_size: 3, static_rx_buf_num: 10, dynamic_rx_buf_num: 32, static_tx_buf_num: 0, dynamic_tx_buf_num: 32, csi_enable: false, ampdu_rx_enable: true, ampdu_tx_enable: true, amsdu_tx_enable: false, rx_ba_win: 6, max_burst_size: 1, country_code: "CN", country_code_operating_class: 0, mtu: 1492, tick_rate_hz: 100, listen_interval: 3, beacon_timeout: 6, ap_beacon_timeout: 300, failure_retry_cnt: 1, scan_method: 0 }
<time> [DEBUG] BT controller compile version aa16a46
<time> [DEBUG] !!!! unimplemented srand 82
<time> [DEBUG] The btdm_controller_init was initialized
<time> [INFO ] Our address = Address { kind: AddrKind(1), addr: BdAddr([65, 90, 227, 30, 131, 231]) }
<time> [INFO ] Starting advertising and GATT service
<time> [INFO ] [host] filter accept list size: 12
<time> [INFO ] [host] setting txq to 12
<time> [INFO ] [host] configuring host buffers (8 packets of size 251)
<time> [INFO ] [host] initialized
<time> [INFO ] [adv] advertising
```

### Confirm that the service is seen (optional)

Using a Bluetooth development tool such as [nRF Connect for Mobile](https://play.google.com/store/apps/details?id=no.nordicsemi.android.mcp) (Google Play Store):

- confirm that a device named "`esp32c{3|6}`" is advertising itself
- `CONNECT` to it
- check its services and characteristics

>![](.images/ble_sniffing.png)

*Figure. Screenshot of the `nRF Connect for Mobile` Android app*

>**Exercise**
>
>If you are using the above tool, do this:
>
>- `Connect`
> 	- Tap `Unknown Service` to reveal the characteristics
>		- You can now read and write those (the DOWN/UP arrows)
>
>You can also walk away from the device, with the phone, and see how far its range reaches.
>
>In addition, you can log data:
>
>```
>156.678779 [INFO ] RECEIVED: 0 [72, 105, 32, 116, 111, 32, 121, 111, 117]
>192.729017 [INFO ] RECEIVED: 0 [72, 105, 32, 97, 103, 97, 105, 110, 33]
>```


## Recommended training material (optional)

To learn more about the potential of BLE, check these:

- [Introduction to Bluetooth Low Energy](https://learn.adafruit.com/introduction-to-bluetooth-low-energy?view=all) (Adafruit; updated Mar'14)

	Simple introduction (maybe 5-10 min).

	>Note: The text has *some* inaccuracies, for example a Bluetooth Peripheral can be in connection with multiple Centrals, at the same time.

- [Bluetooth Low Energy Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/) (DevAcademy by Nordic Semiconductor)

	- 6 lessons
	- "8–10 hours to complete"
	- Exercises use Nordic hardware, but can also just be read through.

	Author opinion: *If you only plan to attend one course, this is a good one!*

<!-- tbd. read, some day?
- [The Basic Concepts of Bluetooth Low Energy (BLE) for Beginner](https://pcng.medium.com/the-basic-concepts-of-bluetooth-low-energy-ble-for-beginner-c0fe062190c5) (Medium; Sep'19)
-->

## Why not use `bleps-macros`?

The `gatt` macro is copied from [`bleps-macros`](https://github.com/bjoernQ/bleps/tree/main/bleps-macros), but:

- `gatt!` provides a value (placed to a variable by the application), instead of assigning a fixed `gatt_attributes` variable. The `bleps-macros` approach confuses RustRover IDE syntax highlighting.

- Rust Rover has a bug in handling crates from a GitHub URL, with features enabled. This eliminates the need for examples to have `macros` feature. *Not a real hard reason, but.. something.*


## Next episode - Web 

Head over to [`../extras/ble-web-app`](../extras/ble-web-app/README.md) and you'll find a Web app that can interact with your BLE device!

Leave the device on. See you there! :)

