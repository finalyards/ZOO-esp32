# I2C address

This deserves a little side mention, since the way it's implemented was surprising to the author; perhaps this is how things should be..

## Address does not persist

If you set the I2C address of a VL53L5CX sensor, *it does not persist* over restarts.

Having the `PWR_EN` line (of a SATEL board) go low is enough to reset the chip (obviously, since it turns its power off), and once it comes back, it's also back to `0x52` I2C address.

This is likely because *there is no persistent memory* within the chip's capabilitities. This may be fine, but **would deserve a mention in the vendor docs**.

### Implications

Instead of being able to prepare a number of chips, with unique I2C addresses, and route them with minimal wires, one now needs a `LPn` wire to each individual chip. Always.

## Alternatives?

`<speculation>`

- Ideally, the chip would have at least one byte of persistent memory, to store its I2C address.
- It could have e.g. three pins to select parts of the (default) address. This is a pretty common method.
- It could have an output pin that's low if the chip is running with the default I2C address. This pin could be then routed to the `LPn` pin of the next chip (and the next..), meaning *only one chip with the default address* would ever be active.

   Heck, this *could* be done even with existing boards (with a firmware change?) since there are some unused output pins (looking at `RSVD5`, `RSVD6`).
   
   The benefit would be that no extra wiring from the MCU would be needed, to facilitate the initialization. 😀😀

`</speculation>`
