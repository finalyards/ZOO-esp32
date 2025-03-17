# Troubles

## Initializes, but no results

```
0.200852 [INFO ] Target powered off and on again.
0.204445 [DEBUG] Ping succeeded: 0xf0,0x02
2.587365 [INFO ] Init succeeded
```

Check that the `INT` pin is wired.


## [ESP32-C3] I2C `TimeOut`

```
0.956520 [INFO ] Target powered off and on again.
0.960236 [DEBUG] Ping succeeded: 0xf0,0x02
1.522238 [ERROR] panicked at 'I2C write to 0x0bd0 (252 bytes) failed: TimeOut'
1.522361 [ERROR] ====================== PANIC ======================
```

This happens with latest versions of `probe-rs`.

- the problem is [wont-fix](https://github.com/probe-rs/probe-rs/issues/2818#issuecomment-2358791448), unless they get news from Espressif

If you need to work on ESP32-C3, you can install commit `6fee4b6` of `probe-rs`. That should work, but you won't get updates to the tool.

>More details in -> [`../../TROUBLES.md`](../../TROUBLES.md).

## `VL53L5CX_Configuration` size dispute

>This *is* a bug of the build system, but happens rather rarely and the author hasn't been able to fix it. Please try! :)

```
$ cargo build --release --lib
   Compiling vl53l5cx_uld v0.0.0 (/home/ubuntu/ZOO.tof/tof/vl53l5cx_uld)
error[E0080]: evaluation of constant value failed
  --> src/uld_raw.rs:38:10
   |
38 |         [::core::mem::size_of::<VL53L5CX_Configuration>() - 2328usize];
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute `2320_usize - 2328_usize`, which would overflow

For more information about this error, try `rustc --explain E0080`.
```

The reason is something's gotten confused with the `tmp/config.h[.next]` files. Remove them, and retry the build:

```
$ rm tmp/config.h*
$ cargo build		# might be needed
$ make manual
$ make -f Makefile.dev m3
```

That should fix it.


<!-- REMOVE
## Stuck, reading `0x2c00`

Without tracing:

```
$ FEATURES=esp-hal-0_22 DEFMT_LOG=debug make -f Makefile.dev m3
[...]
13.879729 [ERROR] ====================== PANIC ======================
13.879756 [ERROR] panicked at examples/m3.rs:161:10:
```

With tracing:

```
$ FEATURES=esp-hal-0_22 DEFMT_LOG=trace make -f Makefile.dev m3
[…]
23.669357 [TRACE] I2C read: 0x2c00 -> [0x00, 0x00, 0x00, 0x00]
23.669405 [TRACE] 🔸 10ms
23.681221 [TRACE] I2C read: 0x2c00 -> [0x00, 0x00, 0x00, 0x00]
23.681269 [TRACE] 🔸 10ms
23.693085 [TRACE] I2C read: 0x2c00 -> [0x00, 0x00, 0x00, 0x00]
23.693133 [TRACE] 🔸 10ms
23.706749 [TRACE] I2C read: 0x2c04 -> [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]... (24 bytes)
23.706865 [ERROR] ====================== PANIC ======================
23.706889 [ERROR] panicked at examples/m3.rs:161:10:
to start ranging: Error(255)
<<
```

The VL firmware tries to read `0x2c00` and doesn't get the values it wants.

Reason unknown.

Workaround: none??

**TO BE STUDIED**

**EDIT:**

Just relaunching seems to *sometimes* avoid this!
-->
