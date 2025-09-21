# Design

## GATT vs. L2CAP

While you don't need to master the BLE protocol in order to make applications for it, some high level knowledge is beneficial to know *why* we do things certain way.

**GATT** is the server / service(s) / characteristic(s) -based API. You can read, write, or listen to each characteristic exposed by a BLE peripheral. The main reason why the project works on this level is that **Bluetooth Web API supports it** (and not L2CAP).

---

There are references to **L2CAP** even in the source code. This is a layer below GATT.

L2CAP allows two-directional piping between the central and peripheral devices, so one could craft their own protocol and bypass the GATT hierarchies, altogether. This sounds tempting, if you control both ends, which is the case in our example. However:

- BLE sniffers might only work on the GATT level (=> harder debugging)
- no Bluetooth Web API support
- even native support may be lacking (depends on the platform)

Thus the choice is clear: GATT for the win!!
