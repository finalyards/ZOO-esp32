
## Learning material (optional)

The Bluetooth Low Energy ecosystem is more complex than normal sensors would be. Thus, we want to offer a list of in-depth dive to the protocols.

- [Introduction to Bluetooth Low Energy](https://learn.adafruit.com/introduction-to-bluetooth-low-energy?view=all) (Adafruit; updated Mar'14)

	Simple introduction (maybe 5-10 min).

	>Note: The text has *some* inaccuracies, for example a Bluetooth Peripheral can be in connection with multiple Centrals, at the same time.

- [Bluetooth Low Energy Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/) (DevAcademy by Nordic Semiconductor)

	- 6 lessons
	- "8–10 hours to complete"
	- Exercises use Nordic hardware, but can also just be read through.

	Author opinion: *If you only plan to attend one course, this is a good one!*

You will be "living" on the "host" side of the HCI (Host/Client Interface). Most BLE devices (ESP32 included) have such an interface, even when the same chip would handle both roles. The interface is standardized, so essentially it means serializing/deserializing whatever goes on in BLE. The [`trouble´](...) library takes care of this for you, but it's good to know what's under the hood...


## References

- [Part A. Data Types Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/CSS_v11/out/en/supplement-to-the-bluetooth-core-specification/data-types-specification.html) (Bluetooth.com; very official)

	Look up here for using the right terms.
	
	<!-- #hizzz -->
	>*Unfortunately individual chapters aren't addressable; why, what kind of HTML is this??! <font size=+3>🧌</font>*
	
