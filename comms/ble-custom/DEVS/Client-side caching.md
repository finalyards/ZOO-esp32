# Client-side caching

Here's a comment I picked up from a [TrouBLE GitHub Issue](https://github.com/embassy-rs/trouble/issues/195#issuecomment-2666769648):

>*A random observation about the Address from a newbie to both trouble and BLE: it appears that **the client devices you connect to will 'remember' your peripheral's services & characteristics** (this is at least the case when testing with an iOS device). This can be problematic during development if you're frequently adding or changing services or characteristics, because the client won't re-attempt discovery for a device it already knows about. BUT: if you update the Address value on these changes, the client will treat the device as unknown/unbonded, and service/characteristic discovery works as expected. Lost almost a day figuring this one out the hard way :')*

The discussion continues:

>*@chrissuozzo Another way to fix that is by adding the **"Service Changed" characteristic (see [#180](https://github.com/embassy-rs/trouble/issues/180) for a discussion about it)** to the 0x1801 service. Relevant part in spec: [link](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/generic-attribute-profile--gatt-.html section 2.5.2).

