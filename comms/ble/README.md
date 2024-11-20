# Bluetooth Low Energy

Exposing an ESP32 device via the [Bluetooth Low Energy](...) protocol, and interacting with it using [Web Bluetooth](...).

<!-- tbd. picture here, perhaps from parent -->

## Requirements

- ESP32-C6 devkit
- Android phone or 
   - desktop/laptop with Chrome browser

No wiring is needed.

>Apple note:
>
>Web Bluetooth is not supported by Apple's WebKit, but there are [third party applications](https://www.apple.com/us/search/web-bluetooth?src=globalnav) that allow Web Bluetooth on iOS/iPad devices.
 

## Running

### Launching the Bluetooth device

```
$ DEFMT_LOG=debug cargo run --release --example x-emb
[...]
0.793924 [INFO ] esp-wifi configuration EspWifiConfig { rx_queue_size: 5, tx_queue_size: 3, static_rx_buf_num: 10, dynamic_rx_buf_num: 32, static_tx_buf_num: 0, dynamic_tx_buf_num: 32, csi_enable: false, ampdu_rx_enable: true, ampdu_tx_enable: true, amsdu_tx_enable: false, rx_ba_win: 6, max_burst_size: 1, country_code: "CN", country_code_operating_class: 0, mtu: 1492, tick_rate_hz: 100, listen_interval: 3, beacon_timeout: 6, ap_beacon_timeout: 300, failure_retry_cnt: 1, scan_method: 0 }
0.853086 [DEBUG] The ble_controller_init was initialized
0.853120 [INFO ] Connector created
0.857462 [INFO ] ble_npl_eventq_remove 0x4080c08c 0x4080e730
0.877252 [INFO ] "Ok(CommandComplete { num_packets: 1, opcode: 3075, data: [0] })"
0.887373 [INFO ] "Ok(CommandComplete { num_packets: 1, opcode: 8198, data: [0] })"
0.907548 [INFO ] "Ok(CommandComplete { num_packets: 1, opcode: 8200, data: [0] })"
0.918109 [INFO ] "Ok(CommandComplete { num_packets: 1, opcode: 8202, data: [0] })"
0.918306 [INFO ] started advertising
```

### Confirm that the service is seen (optional)

You can now use a Bluetooth development tool (such as [nRF Connect for Mobile](https://play.google.com/store/apps/details?id=no.nordicsemi.android.mcp)), to:

- confirm that a device named "`esp32c6`" is advertising itself
- `CONNECT` to it
	- check its services and characteristics

<!-- with screenshots:
	>Try to a) read the "Hello Bare Metal" string
	>b) Change it to something else
-->	 

Once you've written data on the tool, observe these in the `defmt` output:

```
156.678779 [INFO ] RECEIVED: 0 [72, 105, 32, 116, 111, 32, 121, 111, 117]
192.729017 [INFO ] RECEIVED: 0 [72, 105, 32, 97, 103, 97, 105, 110, 33]
```

