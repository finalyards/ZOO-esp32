# On the BLE Protocol

Short summary and some references for further reading / viewing. 

>Disclaimer: This text is meant *only* for being able to understand the source code.


## What is it?

The BLE protocol is a separate protocol from "classic" Bluetooth (which is also called BR/EDR, regarding its data rates). BLE became a standard in 2010, and thereafter **most improvements to Bluetooth have been on the BLE side of things**.

A device can be Classic only, BLE only, or support parts from both. Some tools and/or companies might be "BLE only":

- Most Espressif MCU's (Bluetooth 5.0)
- Nordic Semiconductor company, tools and products

The main difference (reason for existence!) between BLE and "classic" Bluetooth is that in BLE, even when connected, the radio presence is part-time, allowing power savings for both the peripheral and central (BLE terms). Also, it treats the parties asymmetrically, placing more (power, processing) burden on the "central" devices (such as one's mobile phone, or a tablet).

## Role in IoT

BLE was mentioned to be designed for **fitness** (think: watches, medical monitoring), **home automation** (think: thermostats) and **IoT** (any battery powered devices). IoT is in the essence of BLE, and the companies making products for it!

>Note: IoT of course is just a marketing term, and a bit aged at that. The author tries not to mention it, again. :)

What the author finds great for <strike>IoT</strike> *connected embedded devices* is that one can create *custom profiles*, not needing to piggy-back on existing ones. This is the domain that the ZOO repo needs and explores!!!


## Security

BLE offers multiple levels of security, depending on your use case.

<!-- #later?; perhaps a diagram? tbd.
|use cases|advertisement|encryption|authentication|comments|
|---|---|---|---|---|
|read-only beacons|yes|no|no|
-->

The radio communication can be encrypted. Often, a shared 6-digit number is used to establish the connection (and avoid man-in-the-middle attacks), but also other means to provide the keys exist ("out of band", via e.g. NFC).

Attending e.g. the [Bluetooth Low Energy Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/) (DevAcademy by Nordic Semiconductor) Nordic training course is invaluable and enlightening, when you have your own application in mind!


## References

### Courses

- [Introduction to Bluetooth Low Energy](https://learn.adafruit.com/introduction-to-bluetooth-low-energy?view=all) (Adafruit)

	Simple introduction (maybe 5-10 min).

	>Note: The text has *some* inaccuracies, for example a Bluetooth Peripheral can be in connection with multiple Centrals, at the same time.

<!-- #whisper
	>Note: Some details may have been rounded off, to make the write short. For example, it is possible for a BLE peripheral to connect to two centrals at the same time. This is, however, something a basic introduction like this does not need to cover. Just don't take it as 100% correct, but check from other sources, as well.
-->

- [Bluetooth Low Energy Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/) (DevAcademy by Nordic Semiconductor)

	- 6 lessons
	- "8–10 hours to complete"
	- Exercises use Nordic hardware, but can also just be read through.

	Author opinion: *If you only plan to attend one course, this is a good one!*

- [Bluetooth Classic & BLE with ESP32](https://dronebotworkshop.com/esp32-bluetooth/) (article + Youtube video (37:59); May 2024)

	*If you plan to attend TWO courses, or are deprived of time, take this!*


### Other

- [Bluetooth Technology Overview](https://www.bluetooth.com/learn-about-bluetooth/tech-overview/) (Bluetooth SIG)

- [Bluetooth Low Energy Primer](https://www.bluetooth.com/wp-content/uploads/2022/05/the-bluetooth-le-primer-v1.2.0.pdf) (Bluetooth SIG; updated Mar'24; PDF 75 pp.)

	<!-- ^-- tbd. read & review/comment -->

