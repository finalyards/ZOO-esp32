# `probe-rs` speed

The `--speed` parameter of the `probe-rs run` doesn't seem to affect much. 

```
      Erasing ✔ [00:00:03] [################] 256.00 KiB/256.00 KiB @ 77.54 KiB/s (eta 0s )
  Programming ⠄ [00:00:46] [############----] 92.62 KiB/103.35 KiB @ 1.94 KiB/s (eta 6s )
```

Erasing: 77..94 kBps.<br />
Programming: ~2 kBps

Is the programming speed "nominal", or is there something I can do to raise it?

