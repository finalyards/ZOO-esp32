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


### Capabilities

A service consists of one or more capabilities. These stand for certain data to be read, written or notified when it changes.


## GAPP

*tbd.*

## GATT

*tbd.*

