```
~~ Once we have a clear grasp of TrouBLE (and our own example works), could remove this?
```

# Walkthrough of [TrouBLE](https://github.com/embassy-rs/trouble)

TrouBLE is the BLE host stack that we use. Because (as of Dec'24) it doesn't have clear internal documentation, here's some guidance to what it consists of.

## File structure

```
$ tree -I target
.
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── ci.sh
├── examples
│   ├── apache-nimble
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── memory.x
│   │   └── src
│   │       └── bin
│   │           └── ble_bas_peripheral.rs
│   ├── apps
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── ble_advertise_multiple.rs
│   │       ├── ble_bas_central.rs
│   │       ├── ble_bas_peripheral.rs
│   │       ├── ble_l2cap_central.rs
│   │       ├── ble_l2cap_peripheral.rs
│   │       ├── fmt.rs
│   │       └── lib.rs
│   ├── esp32
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   └── src
│   │       └── bin
│   │           └── ble_bas_peripheral.rs
│   ├── nrf-sdc
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── memory-nrf52832.x
│   │   ├── memory-nrf52833.x
│   │   ├── memory-nrf52840.x
│   │   └── src
│   │       └── bin
│   │           ├── ble_bas_central.rs
│   │           ├── ble_bas_peripheral.rs
│   │           ├── ble_l2cap_central.rs
│   │           └── ble_l2cap_peripheral.rs
│   ├── rp-pico-w
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── memory.x
│   │   └── src
│   │       └── bin
│   │           ├── ble_bas_central.rs
│   │           └── ble_bas_peripheral.rs
│   └── serial-hci
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── bin
│               └── ble_bas_peripheral.rs
├── host
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── build.rs
│   ├── gen_config.py
│   ├── src
│   │   ├── advertise.rs
│   │   ├── att.rs
│   │   ├── attribute.rs
│   │   ├── attribute_server.rs
│   │   ├── central.rs
│   │   ├── channel_manager.rs
│   │   ├── codec.rs
│   │   ├── command.rs
│   │   ├── config.rs
│   │   ├── connection.rs
│   │   ├── connection_manager.rs
│   │   ├── cursor.rs
│   │   ├── fmt.rs
│   │   ├── gap.rs
│   │   ├── gatt.rs
│   │   ├── host.rs
│   │   ├── l2cap
│   │   │   └── sar.rs
│   │   ├── l2cap.rs
│   │   ├── lib.rs
│   │   ├── mock_controller.rs
│   │   ├── packet_pool.rs
│   │   ├── pdu.rs
│   │   ├── peripheral.rs
│   │   ├── scan.rs
│   │   └── types
│   │       ├── gatt_traits.rs
│   │       ├── l2cap.rs
│   │       ├── mod.rs
│   │       ├── primitives.rs
│   │       └── uuid.rs
│   └── tests
│       ├── common.rs
│       ├── gatt.rs
│       ├── gatt_derive.rs
│       ├── l2cap.rs
│       └── service_attribute_macro.rs
├── host-macros
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── characteristic.rs
│       ├── ctxt.rs
│       ├── lib.rs
│       ├── server.rs
│       ├── service.rs
│       └── uuid.rs
├── rust-toolchain.toml
└── rustfmt.toml
```

`examples/esp32` holds the main files for ESP32 examples. These, alongside other `examples/*` samples, refer to `examples/apps` for common code.

>In fact, the host logic is within `examples/apps`, e.g. `ble_bas_peripheral.rs` and the hardware specific `examples/esp32` is a *cradle* for such code to work. Rust doesn't have *dependency injection*; otherwise the `apps` could clearly be marked as *the* place to see, and the others be supportive actors.
>
>Worth noting that also the `Cargo.toml`s matter for the hardware specific projects. The variation on what needs to be linked in is huge. This alone, may be enough of a reason to keep them - instead of `apps` - as the topmost level.

`host` has the bulk of the TrouBLE code base.

`host-macros` - you can ignore this?


<!--
`examples/esp32/src/bin/ble_bas_peripheral.rs`
-->
