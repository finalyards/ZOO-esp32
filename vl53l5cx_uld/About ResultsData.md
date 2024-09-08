# Observed `ResultsData`

A summary of numbers which we've seen in real data. This should give some additional insight to what the various `ResultData` fields are about, to guide one which are needed in a particular application of yours.

|field|observed values|Vendor docs say|We add|
|---|---|---|---|
|`target_status`|5,6,9,10,255|"Measurements validity"|
|`targets_detected`|↴|"Number of detected targets in the current zone. This value should be the first one to check to know a measurement validity."|The `targets_per_zone_{X}` feature gives the maximum.|
|*targets per zone=1*|1|
|*targets per zone=2*|...|
|*targets per zone=3*|...|
|*targets per zone=4*|...|
|`ambient_per_spad`|1,0,2 (mostly 1)|"ambient signal rate due to noise" (unit:&nbsp;Kcps/SPAD)|
|`spads_enabled`|14000..17000|"Number of SPADs enabled for the current measurement. A far or low reflective target activates more SPADs."|
|`signal_per_spad`||"signal returned to the sensor in kcps/spads"; "Quantity of photons measured during the VCSEL." (unit:&nbsp;Kcps/SPAD)|
|`range_sigma_mm`|5..15|"sigma of the current distance in mm"; "Sigma estimator for the noise in the reported target distance" (unit: mm)|Compilation warns if defined separately from the `.distance_mm` values; only makes sense in relation to distance measurements.|
|`.distance_mm`|800..1800 (anything)|"Distance measurement" (unit:&nbsp;mm)|
|`.reflectance`|40..90|"estimated reflectance in percent" (unit:&nbsp;%)|


## `.target_status`

Vendor docs state:

>"5 is considered 100% valid";<br />
>"6 or 9 can be considered [to be valid by a] 50% [probability]";<br />
>"All other values [fall] below the 50% level."

Values 0..13; 255 are documented in ST.com [UM2884 - Rev 5](https://www.st.com/resource/en/user_manual/um2884-a-guide-to-using-the-vl53l5cx-multizone-timeofflight-ranging-sensor-with-a-wide-field-of-view-ultra-lite-driver-uld-stmicroelectronics.pdf) (PDF Feb'24; 18pp).

## `.ambient_per_spad`

Example:

```
[[1, 1, 1, 1], [1, 2, 0, 1], [1, 1, 1, 2], [1, 1, 1, 1]]
```

## `.spads_enabled`

Example (indoor lighting):

```
[[15104, 16384, 15616, 15872], [14592, 15616, 15616, 15872], [16128, 15104, 15360, 15360], [15872, 15616, 14080, 15360]]
```

## `.signal_per_spad`

Example (indoor lighting):

```
[[10, 15, 15, 15], [17, 17, 16, 14], [19, 19, 16, 16], [5, 19, 16, 17]]
```

## `.range_sigma_mm`

Related to the `distance_mm` results; provides an accuracy estimate for the provided result.

>![INFO]
>Before using this for value validity, consider the `.target_status`, to disqualify non-good measurements.

Example (indoor lighting; `.distance_mm` within `1452`..`1853` mm range with one outlier at `832` mm; all target statuses `Valid` or `HalfValid`):

```
[[15, 10, 7, 9], [7, 7, 5, 7], [6, 6, 6, 9], [12, 6, 7, 8]]
```

## `.distance_mm`

Example:

```
[[1452, 1721, 1834, 1869], [1738, 1806, 1820, 1853], [1755, 1772, 1783, 1826], [832, 1754, 1789, 1808]]
```

<!-- tbd. mention about minimum / maximum experiences? -->

## `.reflectance`

Example (indoor; white target):

```
[[40, 59, 79, 75], [70, 71, 87, 62], [77, 76, 67, 68], [5, 70, 69, 69]]
```

<!--
tbd. something at the end?
-->
