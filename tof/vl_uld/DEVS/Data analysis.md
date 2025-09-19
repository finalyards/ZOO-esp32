# Data analysis

The author collects here some logged, actual sensor data, to understand its internal relations better.

---

## 12-Sep-25

|sensor|matrix|targets|
|---|---|---|
|L8|4x4|2|

### .A

```
# First target:
	nb_target_detected: [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
	distance_mm: [1712, 1748, 1716, 1712, 1826, 1813, 1797, 1792, 1815, 1813, 1794, 1786, 1819, 1814, 1793, 1777]
	target_status: [9, 5, 9, 9, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]

# 2nd target:
	nb_target_detected: [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
	distance_mm: [1746, 1633, 1737, 1712, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
	target_status: [4, 4, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

Observations:

**First target**

All measurements are valid, but three are noted with status `9`:

>"Range valid with large pulse (may be due to a merged target)"

Those should be trusted "50%", whereas most measurements are good.

**Second target**

Three zones has `.target_status` `4` ("target consistency failed"). They do still have meaningful-looking `.distance_mm`, but based on the specs, such numbers are not to be trusted (confidence 0%).

There is one result with status `5` (ok); for this one also the `.nb_target_detected` indicates there to be two targets detected.


**‼️NOTE‼️**

The distances are the same!

The VL53 specs say that there would be min. 60cm between consequetive targets. What's happening here?


### .B

A similat sample - slightly different layout.

**Common fields (and raw data)**

```
	nb_target_detected: [1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
	RAW distance_mm: [1683, 1758, 1653, 1741, 1654, 1758, 1699, 1727, 1828, 0, 1817, 0, 1803, 0, 1782, 0, 1820, 2874, 1820, 0, 1803, 2394, 1778, 2798, 1831, 0, 1809, 0, 1794, 0, 1784, 0]
	RAW target_status: [9, 5, 5, 5, 5, 4, 9, 5, 5, 0, 5, 0, 5, 0, 5, 0, 5, 4, 5, 0, 5, 4, 5, 4, 5, 0, 5, 0, 5, 0, 5, 0]
20.404966
```

**First target**

```
	distance_mm: [1683, 1653, 1654, 1699, 1828, 1817, 1803, 1782, 1820, 1820, 1803, 1778, 1831, 1809, 1794, 1784]
	target_status: [9, 5, 5, 9, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]
20.405126
```

**Second target**

```
	distance_mm: [1758, 1741, 1758, 1727, 0, 0, 0, 0, 2874, 0, 2394, 2798, 0, 0, 0, 0]
	target_status: [5, 5, 4, 5, 0, 0, 0, 0, 4, 0, 4, 4, 0, 0, 0, 0]
```

Picking the interesting ones:

||||||||||||
|---|---|---|---|---|---|---|---|---|---|---|
|`nb_target_detected`|1|**2**|**2**|1|...|1|...|1|1|...|
|`distance_mm`|1758|**1741**|**1758**|1727|...|2874|...|2394|2798|...|
|`target_status`|5|**5**|**4**|5|...|4|...|4|4|...|

**Observations**

`target-status` goes together with having a `distance_mm` measurement value - but why is `nb_target_detected` only valid for two of these?

In particular, the valid ones have both `target-status` 5 **and** 4 ("target consistency failed"). 

How is this supposed to be interpreted??

---

### Conclusion

The author doesn't want the end user to be concerned with data inaccuracies.

And it should remain in the role of `vl_uld` to "correct" (filter) such data - not the `vl_api` which is an abstraction layer for other things.

I don't want to (at this stage) consider other fields than the `nb_target_detected`, `distance_mm` and `target_status` mentioned above. Later, perhaps...

To see what actually got implemented, see `src/results_data.rs`.


