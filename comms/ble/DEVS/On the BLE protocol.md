# On the BLE Protocol

We'll give a short summary and some references for further reading / viewing.

This text is meant *only* for being able to understand the source code. 


## What is it?

The BLE protocol is a separate protocol from "classic" Bluetooth (also called BR/EDR, regarding its data rates). BLE became a standard in 2010, and thereafter most improvements to Bluetooth have been on the BLE "side" of things.

A device can be Classic only, BLE only, or support parts from both.

Some tools and/or companies might be "BLE only", e.g.

- Espressif ESP32's MCU's
- Nordic Semiconductor

The main difference (reason for existence!) between BLE and "classic" Bluetooth is that in BLE, even when connected, the radio presence is part-time, allowing power savings for both the peripheral and central (BLE terms). Also, it treats the parties asymmetrically, placing more (power, processing) burden on the "central" devices (such as one's mobile phone or a tablet).

## Role in IoT

BLE was mentioned to be designed for fitness (think: watches, medical monitoring), home automation (think: thermostats) and IoT (any battery powered ubiquituos devices). IoT is in the essence of BLE, and the companies making products for it!

>Note: IoT of course is just a marketing term, and a bit aged at that. We try to not mention it. :)

What the author finds great for <strike>IoT</strike> *connected embedded* devices is that one can create *custom profiles*, not needing to piggy-back on existing ones - like the author thinks is the case with Bluetooth Classic (using RFCOMM to build your profile on top). This is the domain that the repo needs and explores!!!

<!-- moved
## Terminology!

*The Nordic training <sup>[`|1|`](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/)</sup> has better pictures, for this!*

### Devices: Peripheral, central vs. Server, client

A **peripheral** is the more power-challenged device, often a battery-operated sensor. It can advertise its presence and features. 

A **central** has more power, and has a wider role e.g. in the connection process. This is normally your phone, tablet or computer.

A **server** provides the data. It stores e.g. the latest messages and provides them to the radio waves once a discussion opportunity presents itself with the client (every `X` ms).

A **client** knows what to do with the peripheral. It is closer to the humand or automation, and can e.g. provide a UI/API to steer the device, or pass the data for longer storage/analysis. Think home automation hub.

Usually, a peripheral is the server, and a central is the client. Which term is being used depends on which aspect of their existence is being focused upon.

A peripheral can connect to multiple centrals, and a central to multiple peripherals.


### Service

A BLE peripheral provides one or more services. These can be standardized (so "any" software knows to deal with the device) or custom (only your software knows the tricks).


### Capabilities

A service consists of one or more capabilities. These stand for certain data to be read, written or notified when it changes.
-->

## Security!

BLE offers multiple levels of security, depending on your use case needs.

<!-- #later?
|use cases|advertisement|encryption|authentication|comments|
|---|---|---|---|---|
|read-only beacons|yes|no|no|
-->

The radio communication can be encrypted. Often, a shared 6-digit number is used to establish the connection (and avoid man-in-the-middle attacks), but also other means to provide the keys exist ("out of band", via e.g. NFC).

>Using a (freely available) Bluetooth sniffer is a good starting point to see what your Bluetooth environment already is.

Reading e.g. the Nordic training course (link below) is invaluable and enlightening, when you have your own application in mind!!!

## References

<!-- ~~
### Courses

- [Introduction to Bluetooth Low Energy](https://learn.adafruit.com/introduction-to-bluetooth-low-energy?view=all) (Adafruit)

	Simple introduction (maybe 5-10 min).

	>Note: The text has *some* inaccuracies, for example a Bluetooth Peripheral can be in connection with multiple Centrals, at the same time.

<!_-- #whisper
	>Note: Some details may have been rounded off, to make the write short. For example, it is possible for a BLE peripheral to connect to two centrals at the same time. This is, however, something a basic introduction like this does not need to cover. Just don't take it as 100% correct, but check from other sources, as well.
--_>

- `|1|`: [Bluetooth Low Energy Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/) (DevAcademy by Nordic Semiconductor)

	- 6 lessons
	- "8–10 hours to complete"
	- Exercises use Nordic hardware, but can also just be read through.

	Author opinion: *If you only plan to attend one course, this is a good one!*

### Other
-->

- [Bluetooth Technology Overview](https://www.bluetooth.com/learn-about-bluetooth/tech-overview/) (Bluetooth SIG)

- [Bluetooth Low Energy Primer](https://www.bluetooth.com/wp-content/uploads/2022/05/the-bluetooth-le-primer-v1.2.0.pdf) (Bluetooth SIG; updated Mar'24; PDF 75 pp.)

	<!-- ^-- tbd. read & review/comment -->
	
<!--
- [Introduction to Bluetooth Low Energy](https://www.youtube.com/watch?v=5TxUnbsHsR8) (a Webinar recording by Nordic Semiconductor, May'20; Youtube 1:28:40)
-->