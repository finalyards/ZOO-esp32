# Terminology

*This gives you the basic BLE terminology, in order to follow the source code. For more info, see Training Material (in README).*

### Devices: Peripheral, central vs. Server, client

A **peripheral** is the more power-challenged device, often a battery-operated sensor. It can advertise its presence and features. 

A **central** has more power, and has a wider role e.g. in the connection process. This is normally your phone, tablet or computer.

A **server** provides the data. It stores e.g. the latest messages and provides them to the radio waves once a discussion opportunity presents itself with the client (every `X` ms).

A **client** knows what to do with the peripheral. It is closer to the humand or automation, and can e.g. provide a UI/API to steer the device, or pass the data for longer storage/analysis. Think home automation hub.

Usually, a peripheral is the server, and a central is the client. Which term is being used depends on which aspect of their existence is being focused upon.

A peripheral can connect to multiple centrals, and a central to multiple peripherals.


### Service

A BLE peripheral provides one or more services. These can be standardized (so "any" software knows to deal with the device) or custom (only your software knows the tricks).


### Characteristic

A service consists of one or more characteristics. These stand for certain data to be read, written or notified when it changes.

In turn, "characteristics [can be considered as] groups of information called attributes." <sup>`[TI-GATT]`</sup>

Attributes can be:

- **read**: self-evident
- **written to**: self-evident
- **notified of**: server tells asynchronously that something has changed (no ack)
- (indicated): like notification, but with ack; can be considered as a write from server -> client	<!-- tbd. is that a fair description?  indications in practise? -->

>Indication seems to be used less frequently.

In addition to their value, attributes have:

- **handle**: "index of the attribute in the [attribute] table"




## GATT

Our focus, since the Web Bluetooth API functions on this level.

## L2CAP

Transmission layer below the GATT protocol.

|||
|---|---|
|SDU|Service Data Unit: a packet of data that L2CAP exchanges with the upper layer and transports transparently over an L2CAP channel [...].|
|PDU|Protocol Data Unit: a packet of data containing L2CAP protocol information fields, control information, and/or upper layer information data|
|MTU|Maximum Transmission Unit. The maximum size of payload data, in octets (*aka bytes*), that the upper layer entity can accept (that is, the MTU corresponds to the maximum SDU size).|
|MPU|Maximum PDU Payload Size. The maximum size of payload data in octets that the L2CAP layer entity can accept (that is, the MPS corresponds to the maximum PDU payload size).|

<small>*Descriptions from `[TI-L2CAP]`*</small>

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
- [Logical Link Control and Adaptation Layer Protocol (L2CAP)](https://software-dl.ti.com/lprf/sdg-latest/html/ble-stack-3.x/l2cap.html) (Texas Instruments; 2016) <sub>`[TI-L2CAP]`</sub>

<!-- we don't mention GAP
- [Generic Access Profile (GAP)](https://software-dl.ti.com/lprf/sdg-latest/html/ble-stack-3.x/gap.html) (Texas Instruments, 2016)
-->
