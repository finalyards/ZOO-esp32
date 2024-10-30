# Todo

## `LPn` pin's true meaning

Does it:

- force the chip into "Low Power Idle" mode, where ranging does not happen?  (see Table 4, "DS13754 - Rev 12" (p.9)).

- ..or does it simply disable I2C comms with the board?

Implications:

Whether we need to give each board separate I2C addresses, or can steer them simply by `LPn`?


- [ ] Tee example, joka testaa ylläolevan:
	- aloita ranging
	- pidä LPn alhaalla
	- tuleeko tuloksia? (INT)?
