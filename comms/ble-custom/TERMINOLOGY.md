# Terminology

*This gives you the basic BLE terminology, in order to follow the source code, better. For more in-depth knowledge, see Learning (in the README).*

## Peripheral, central vs. Server, client

A **peripheral** is the more power-challenged device, often a battery-operated sensor. It can advertise its presence and features. 

A **central** has more power, and has a wider role e.g. in the connection process. This is normally your phone, tablet or computer.

A **server** provides the data. It stores e.g. the latest messages and provides them to the radio waves once a discussion opportunity presents itself with the client.

A **client** knows what to do with the data. It is closer to the human or automation, and can e.g. provide a UI/API to steer the device, or pass the data for longer storage/analysis. Think home automation hub.

Usually, a peripheral is the server, and a central is the client. Which term is being used depends on which aspect of their existence is being focused upon.

A peripheral can connect to multiple centrals, and a central to multiple peripherals.


## Services

A BLE device provides one or more services. These can be standardized (so multiple, unrelated software knows, how to deal with the device) or custom (only your software knows the right tricks).

Services can run either on a peripheral, or on a central device.

>Note: "server" and "service" are a bit too close, and seem to confuse at least the author's brain. The service really is a capability of a given server, and a server can have multiple of them - such services can be related, or totally unrelated.
>
>For example, you can make your custom "server", running 1..n custom "services" (up to you how you wish to split/merge them), and you can also run some standard service, such as battery level, alongside the custom ones.


### Characteristic

A service consists of one or more characteristics. These stand for certain data to be read, written or notified when it changes.

In turn, "characteristics [can be considered as] groups of information called attributes." <sup>`[TI-GATT]`</sup>

Attributes can be:

- **read**: self-evident
- **written to**: self-evident
- **notified of**: server tells asynchronously that something has changed (no ack)
- (indicated): like notification, but with ack; can be considered as a write from server -> client	<!-- tbd. is that a fair description?  indications in practise? -->

	Indication seems to be used less frequently.

In addition to their value, attributes have:

- **handle**: "index of the attribute in the [attribute] table"


<!-- tbd.?
## Diagram

**tbd. Joku kuva, joka kuvaisi nuo kaikki?**
-->

## GAP

>***GAP*** *handles all of the BLE device discovery, connection, security, and advertising functions.  It is essential when establishing a connection between BLE devices.* <sub>[source](https://dronebotworkshop.com/esp32-bluetooth/)</sub>

## GATT

>*[...] determines the format of the data exchanged between BLE devices.* <sub>[source](https://dronebotworkshop.com/esp32-bluetooth/)</sub>

Our focus. Web Bluetooth API's are on this level.

## L2CAP

Transmission layer below the GATT protocol. 

You will face this in defining some buffer sizes, e.g. `L2CAP_MTU`.

|||
|---|---|
|SDU|Service Data Unit: a packet of data that L2CAP exchanges with the upper layer and transports transparently over an L2CAP channel [...].|
|PDU|Protocol Data Unit: a packet of data containing L2CAP protocol information fields, control information, and/or upper layer information data|
|MTU|Maximum Transmission Unit. The maximum size of payload data, in *octets* (aka bytes), that the upper layer entity can accept (that is, the MTU corresponds to the maximum SDU size).|
|MPU|Maximum PDU Payload Size. The maximum size of payload data in octets that the L2CAP layer entity can accept (that is, the MPS corresponds to the maximum PDU payload size).|

<small>[source](https://software-dl.ti.com/lprf/sdg-latest/html/ble-stack-3.x/l2cap.html)</small>

## Other picks (not terminology)

>*When using the LE Data Length Extension feature, the length of the LE packet can be up to 251 bytes.*

<p />
>*The maximum `ATT_MTU` size is always 4 bytes less than the value of the `MAX_PDU_SIZE`.*

<p />
>*The longest attribute that can be sent in a single packet is (ATT_MTU-1) bytes.*

<p />
>*[...] connection interval can range from a minimum value of 6 (7.5 ms) to a maximum of 3200 (4.0 s)*

## References

- [Generic Attribute Profile (GATT)](https://software-dl.ti.com/lprf/sdg-latest/html/ble-stack-3.x/gatt.html) (Texas Instruments, 2016) <sub>`[TI-GATT]`</sub>
- [Logical Link Control and Adaptation Layer Protocol (L2CAP)](https://software-dl.ti.com/lprf/sdg-latest/html/ble-stack-3.x/l2cap.html) (Texas Instruments; 2016)

<!-- we don't mention GAP
- [Generic Access Profile (GAP)](https://software-dl.ti.com/lprf/sdg-latest/html/ble-stack-3.x/gap.html) (Texas Instruments, 2016)
-->
