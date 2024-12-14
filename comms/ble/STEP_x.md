<!-- tbd. finalize -->

## Learning material (optional)

The Bluetooth Low Energy ecosystem is more complex than normal sensors would be. Thus, we want to offer a list of in-depth dive to the protocols.

>Notice: you mostly need to focus on the "host" side of BLE. HCI is a standardized interface between that (close to your application) and the client (the stack handling actual hardware).

- [Introduction to Bluetooth Low Energy](https://learn.adafruit.com/introduction-to-bluetooth-low-energy?view=all) (Adafruit; updated Mar'14)

	Simple introduction (maybe 5-10 min).

	>Note: The text has *some* inaccuracies, for example a Bluetooth Peripheral can be in connection with multiple Centrals, at the same time.

- [Bluetooth Low Energy Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/) (DevAcademy by Nordic Semiconductor)

	- 6 lessons
	- "8–10 hours to complete"
	- Exercises use Nordic hardware, but can also just be read through.

	Author opinion: *If you only plan to attend one course, this is a good one!*

<!-- 
tbd. Add resources on:
- HCI data structures

	Something that helps understanding the `trouble` source code.
-->

---

## Next - Web app

Head over to [`../extras/ble-web-app`](../extras/ble-web-app/README.md) and you'll find a Web app that can interact with your BLE device!

Leave the device on, running the custom service. See you there! :)
