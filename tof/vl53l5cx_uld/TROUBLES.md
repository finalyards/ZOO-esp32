# Troubles

## "Driver of hardware level error (66)"

```
Failed with ULD error code: Driver or hardware level error (66)
```

This means the ULD vendor driver has a problem.

The author saw this once, after started to use a pull-up for `PWR_EN` pin, instead of actively pulling it up - and back down again.

If you see this repeatedly, consider driving the `PWR_EN` pin before your application.
