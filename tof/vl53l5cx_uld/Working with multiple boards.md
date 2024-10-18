# Working with multiple VL53L5CX boards

If you want to work with multiple VL53L5CX sensors at the same time, you'll need to change their I2C addresses.



## Changing I2C addresses

The VL53L5CX boards **do not** persist their given I2C addresses. Every board, after each restart, starts with the address `0x52`.

In order to work with multiple boards, one needs to either:

- have them on separate I2C buses (not practical)
- initialize them with different I2C addresses, on the same bus
- switch them on/off using the `LPn` line, for each communications

In order to initialize the boards to be on the same I2C bus, one needs N wires from the MCU, in order to gradually bring them on the bus.

![](.images/wiring-2-satel-boards.png)


We choose the same-bus, gradually bring chips onto it -approach.

## Exercise!

1. Create the wiring shown above (2 or more boards)
2. Run 

   ```
   $ make multiboard
   ```


