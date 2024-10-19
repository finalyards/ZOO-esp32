# DC motors

You can use pretty much any DC motor you have access to, and any power source, starting from a 9V battery, to follow this section.

Controller details are within the [`drv8871`](drv8871/README.md) folder, but let's look at some sample motors, first.

---

>WARN. When dealing with voltages above 5V, danger of frying your MCU board is higher than when doing pure logic. 
>
>- Check your specs, your wiring, twice.
>- If using an adjustable power source, START IT UP UNWIRED and adjust the voltage first! Perhaps start with around 5..6V first time, then raise once you have more confidence in the wiring.
>
>The author burnt the following units:
>
>|hardware|pcs|reason|
>|---|---|---|
>|ESP32-C3-DevKitC-02|1|being studied|

---

Let's go!


## Motors

Here are the two samples the author has access to:

### Salvaged vacuum robot motor

![](.images/small-motor.png)

A traditional DC motor with an integrated reduction gear, and a two-quadrant Hall sensor board for observing actual speed (and direction).

**Specs???**

No specs, but seems similar to [this motor-and-encoder](https://www.servocity.com/110-rpm-micro-gear-motor-w-encoder/) online. *By "similar", the author means that the number and coloring of wires matches.*
 
|||
|---|---|
|voltage range|2.6..15V (tried out)|
|current (free running)|0.17A (tried out)|
|current (slight resistance)|0.2A (tried out)|
|current (stall)|(not tried)|

One can run the motor by directly attaching power to its two main pins, but it's preferable to use the connector.

![](.images/hall.jpg)

```
PINOUT:
.-----.-----.------.------.------.-----.
|M2 ⚫️|M1 🔴|+5V 🟤|GND 🟢|H2 🔵|H1 🟣|
`-----'-----'------'------'------'-----´
  |     |      |      |      |     |
  |     |      |      |      `------- Hall sensors (input)
  |     |      `-------- power to sensor board
  `------- power to the motor
```

**Get to know your motor**

- 👉 Connect just the motor inputs (M1, M2) to a power source, e.g. a 9V battery. Does it run?

- 👉 If you have an oscilloscope, what are the signals in the H1 and H2 inputs, while it's running?

   <!-- tbd. add oscilloscope picture here; with 9V -->

With this covered, actual control of the motor (speed and direction) is covered in the `drv8871` folder. You can check that out, or continue the read.

**Hall sensors**

The two 90° arranged hall sensors allow speed *and* direction of rotation to be sniffed.<sup>[`[How Encoders Work]`](https://www.youtube.com/watch?v=uGWfWRt6MwE)</sup>

Feeding the board consumes only 5mA of current (measured).

The output signal is:

<!-- tbd. descibe -->


**Use of such a motor**

Slow-moving robots. Software development prototyping.



### Nidec BETA V TA600DC (24V; 1.4A)

The second sample motor is a fan. It's rather large (7" diameter, 2" thick), and takes max. 24V input. 
 
![](.images/ta600_top.jpg)

**Specs**

This one does have specs available, e.g. [here](https://www.elecok.com/nidec-ta600dc-a34438-59-24v-1-4a-3wires-cooling-fan.html).

|||
|---|---|
|voltage|24V (label); 12.8..22V (tried out)|
|current intake|1.4A (label); 1..1.6A (tried out)|
|manufacturer|[Nidec](https://www.nidec.com/en/)|

This is a pretty powerful fan. It starts rotating around 12.8V (1A) and rotates ever faster when voltage is increased. Without having it properly mounted, didn't even want to try with 24V.

There is one hall effect input (blue cable).

Unlike the previous motor, where the Motor and Sensor grounds are clearly separate, here there are just *three* cables.

<font size=5>
*tbd. What waveform (voltage) and frequencies on the hall effect cable?*

*tbd. How should one connect such a signal to an MCU? (separate grounds)*
</font>


## Power source(s)

The author used:

- a 9V battery (for small motor)
- an adjustable power supply
- a fixed 24V DC power supply (for the larger motor)


<!-- leave out?
## Motor controller

A motor controller is a fairly simple circuit that keeps your logic voltage separate from the (higher) voltage fed to the motor(s). It also allows the speed of the motor be controlled via (logic side) PWM inputs.

The example we've selected is discussed under:

- [`drv8871/README`](drv8871/README.md)
-->


## Next...

Head to [`drv8871/README`](drv8871/README.md), where we discuss the Motor Controller and have some source code.


## References

- ["Driving DC Motors with Microcontrollers"](https://www.youtube.com/watch?v=ygrsIqWOh3Y) (Youtube, 2023; 1:04:31)

	Includes a [section on the Adafruit DRV8871](https://www.youtube.com/watch?v=ygrsIqWOh3Y&t=1524s) controller that we selected, but also works as a wholistic introduction to DC motors and drivers.

- ["How Encoders Work - with Jason"](https://www.youtube.com/watch?v=uGWfWRt6MwE) (Youtube, 2016; 6:16)
