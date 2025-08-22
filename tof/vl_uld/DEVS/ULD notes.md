# ULD notes

The vendor seems to treat the ULD libaries as separate entities, based on the chip. This is quite unnecessary - most of the interfaces are quite the same. 

Here are some notes, looking at the internals of the libraries.

## Version not updated

While the download packages are marked 2.0.1, the actual source code (`_api.h`) still reports the code as `2.0.0`.

## Comparison between `L8CX` and `L5CX`

Since there is no list of differences provided by the vendor, here's some details (disclaimer: mosty for internal use, helping to make the Rust interface be compatible with either):

||`L8CX`|`L5CX`|implications|
|---|---|---|---|
|`..._POWER_MODE_DEEP_SLEEP`|Introduced. Pretty much a "dead mode", since also the firmware disappears. Shouldn't be much different to power down & reinitialize the whole chip.|n/a|skip|
|`..._STATUS_...` (value 3)|`STATUS_LASER_SAFETY` |`STATUS_CRC_CSUM_FAILED` (unused)|For the first, `VL53L5CX` didn't use the code 3, only defined it. Second, we don't act on individual error codes, since the C code anyways merges them to an unrecognizable mess. skip|
|`..._STATUS_...` (value 5)|`STATUS_FW_CHECKSUM_FAIL`|n/a|
|`GLARE_FILTER`|n/a|exists|Not part of our API. skip|
|Charge pump functions: `{enable|disable}_internal_cp()`|n/a|exists|Not part of our API. skip|
|Sync pin: `{get|set}_external_sync_pin_enable()`|new function|n/a|Useful. Support under `vl53l8cx` feature.|

>Verdict: The sync pin interface seems useful, reflecting a hardware feature that `L8CX` has. The rest seems like minor things (that can be ignored in the interfacing). The overlapping changes give the gut feel that change management between products isn't very great. Thus, it's good we keep checking the differences!

In addition, the `[...]_api.c` has some - what seem like minor - changes, perhaps even bug fixes that made it to one version but not the other.

### Revision id's

Both chips share the product id (`0xF0`), but differ in their revision id (`0x02` for `L5CX`; `0x0C` for `L8CX`); this further emphasises them really - from the point of view of software - being revisions of the same product, not separate products.

