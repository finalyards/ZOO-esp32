# `probe-rs` speed

Speed of `probe-rs` flashing is not great:

```
      Erasing ✔ [00:00:03] [################] 256.00 KiB/256.00 KiB @ 77.54 KiB/s (eta 0s )
  Programming ⠄ [00:00:46] [############----] 92.62 KiB/103.35 KiB @ 1.94 KiB/s (eta 6s )
```

```
Erasing:		50..110 kBps
Programming:	~2..3 kBps
Finished in:	30..40s
```

This is due to USB/IP and/or Multipass virtualization. Running the same natively, or under WSL2, gives better results.

Let's keep an eye on this. Below is a table of environments we know about (glad to take your data):

|env|VM|USB/IP|MCU/JTAG|erasing (KiB/s)|flashing (KiB/s)|finished (s)|
|---|---|---|---|---|---|---|
|macOS 14.6 (Intel)|Multipass 1.14.0|Over WLAN to Windows 10 Home|esp32c3 (JTAG;&nbsp;soldered)|**100**|**3.36**|**30**|
|-''-|-''-|-''-|esp32c6 (JTAG)||||
|Windows 10 Home, WSL 2|-|Over to WSL2|esp32c3 (JTAG;&nbsp;soldered)|**269**|**18.5**|**5.7**|
|Windows 10 Home, WSL 2|-|Over to WSL2|esp32c6 (JTAG)|**100**|**6**|**17**|

<!-- tbd. fill in the C6, next time -->

The speed degredation when using USB/IP over the network is about 50% (~15s). It feels slower.

## Help?

If you know of ways, how to speed up the flashing, let us know!

Tried:

- [x] `--speed` parameter 

   Doesn't affect (obviously, since the bottleneck is likely the USB/IP).

