# Brushless motors

<!-- tbd. MORE PICTURES! :) -->

Brushless motors are a whole lot different than brushed motors.

**Benefits:**

- More quiet
- More energy efficient

**Cons:**

- Needs a controller
- More costly


## My approach /using VESC

Ordered these from [Trampa boards](https://trampaboards.com/) and [Maytech](https://maytech.cn/products/). Will also attach some notes about the purchasing experience since.. well, it matters.

### Trampa

|Product|What is it?|What can it do?|What did it cost?|
|---|---|---|---|---|
|[VESC 6 Mk VI](https://trampaboards.com/vesc-6-mkvi--trampa-gives-you-maximum-power-p-34848.html)|VESC controller, v6|Be powered by 3..12S batteries<br />Drive a BLDC motor<br />power external electronics<br />Interface to similar Trampa VESC's, via CANbus|EUR 230 (+ VAT + customs)|
|[XT90s Anti Spark Connector](https://trampaboards.com/xt90s-anti-spark-connector--female-p-24171.html)|Female connector from the battery|Adopt the battery I have available to be initially used with this.|EUR 3 (+ VAT + customs)|

#### Customer experience

The web shop is almost okay, but its search feature is pretty useless (often providing "no results" if there's a slight typo or something; Trampa could have a look at that - you should *always* recommend something). e.g. "X90" or "ANTISPARK" don't give anything.

Ordering via credit card. Very smooth!

Trampa shipping costs are reasonable, ~18 EUR (UK to Finland).

### Maytech

|Product|What is it?|What can it do?|What did it cost?|
|---|---|---|---|---|
|[5055 Unsealed Motor; 6mm shaft; Sensored; 220KV](https://maytech.cn/products/brushless-hall-sensor-motor-mto5055-220-ha)|small(ish) motor|Likely enough to move a pizza-box sized robot.|EUR 62 (+ VAT + customs)|
|[6355 Motor; Unsealed; 8mm shaft; Sensored; 170KV](https://maytech.cn/products/brushless-hall-sensor-motor-mto6355-170-ha)|medium-sized motor|More than enough!|EUR 70 (+ VAT + customs)|
|[MTSPF50A controller](https://maytech.cn/products/maytech-superesc-50a-compatible-with-vesc-software)|VESC controller, v4|Be powered by 3..12S batteries<br />|EUR 77 (+ VAT + customs)|


#### Customer experience

The web shop is confusing. There's loads of information, including manuals, embedded in the pages. I find this kind of unnecessary - the shop and the technical documentation is traditionally separate. I guess it works if you already know what you want. I had an email discussion with one of their staff - which they recommend to do anyways to check "availability".

Payment via PayPal. Ok.. I guess.  (I think Western buyers would prefer credit card directly, but on second though PayPal might actually be just the right mediator.)  Ordering went smooth.

Maytech shipping was EUR 55.


## Customs

On Maytech's delivery: 

- I received an SMS from UPS, stating customs details are "missing" and I need to fill them in. No problem!

<!-- tbd. -->



## Appendices

### Terminology!

Above tables already expose some terms that are good to be aware of:

<details><summary>**VESC v4** vs. **v6**</summary>
These are hardware design versions. "v6" is more modern, but not necessarily a replacement. They are just different.
</details>

<details><summary>**Unsealed** vs. **Sealed**</summary>
Unsealed motors are prone to take in dust, water, what-not (obviously); a video states metallic dust to be the worst. But they cool better than sealed, are cheaper, and... can be taken apart (more easily).
</details>

<details><summary>**Sensored** vs. **Sensorless**</summary>
Sensors tell the orientation (and direction) of the shaft. They can make startup (after the controller wakes up) more quiet - I've heard - but many board users prefer sensorless, since.. fewer parts to break.
	
Also sensored motors can be run as sensorless, by the VESC controllers (so for learning purposes, it doesn't hurt).
</details>

<details><summary>**3..12S** batteries</summary>
The `S` means Series - how many cells are in series (i.e. adds to the Voltage). More voltage, more speed!

The `P` (e.g. `3SP1`; `10SP3`) is how many such batteries are in parallel. More endurance; more amps out (= higher torque). But bigger.
</details>

<details><summary>VAT + customs</summary>
Neither shop being within the EU, I need to handle the EU VAT and customs duties before getting the goods.
</details>


### Why these products?

Trampa because it's enforced by the official [VESC Project](https://vesc-project.com/Hardware) web site.

MayTech because it seems to have a wide selection of motors, many of which are suited to my eventual application (we're talking sealed motors there).

I selected motors and controllers kind of side-by-side. Battery I'll get later, separately. I was most hesitant about the MayTech VescTool v4 -compatible controller, but want to see how much difference there is to an official controller. We'll see!


## References

<!-- tbd. videos! -->

- [VESC project](https://vesc-project.com/node/3839)

