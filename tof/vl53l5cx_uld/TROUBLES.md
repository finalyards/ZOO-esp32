# Troubles

<!-- #hidden; not seen recently (Jan'25)
## "Driver or hardware level error (66)"

```
Failed with ULD error code: Driver or hardware level error (66)
```

This means the ULD vendor driver has a problem.

The author saw this once, after started to use a pull-up for `PWR_EN` pin, instead of actively pulling it up - and back down again.

If you see this repeatedly, consider driving the `PWR_EN` pin before your application.
-->

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
