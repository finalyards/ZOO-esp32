# `probe-rs` speed

Speed of `probe-rs` flashing is not great:

```
      Erasing ✔ [00:00:03] [################] 256.00 KiB/256.00 KiB @ 77.54 KiB/s (eta 0s )
  Programming ⠄ [00:00:46] [############----] 92.62 KiB/103.35 KiB @ 1.94 KiB/s (eta 6s )
```

Erasing: 50..110 kBps.<br />
Programming: ~2..3 kBps<br />

Finished in: 30..40s

This is due to USB/IP and/or Multipass virtualization. Running the same natively, or under WSL2, gives better results.

Let's keep an eye on this. Below is a table of environments we know about (glad to take your data):

|env|VM|MCU/JTAG|USB/IP|erasing (KiB/s)|flashing (KiB/s)|finished (s)|
|---|---|---|---|---|---|---|
|macOS 14.6 (Intel)|Multipass 1.14.0|Over WLAN to Windows 10 Home|esp32c3 (JTAG, added wire)|**100**|**3.36**|**30**|
|-''-|-''-|-''-|esp32c6 (dedicated JTAG port)||||

<!-- tbd. fill in the C6, next time -->

## Help?

If you know of ways, how to speed up the flashing, let us know!

Tried:

- [x] `--speed` parameter 

   Doesn't affect (obviously, since the bottleneck is likely the USB/IP).

