# API

>*The intention is that API documentation would come from the sources. For now, it's here.

## `vl53l5cx::VL`

### Single board

```
fn VL::new_and_setup(&i2c_shared, I2cAddr) -> Result<VL>
```

Creates a handle for a single sensor.


```
fn VL::start_ranging(self, &RangingConfig, Input<'static>) -> Ranging<DIM>
```

Starts a ranging session. This consumes the `VL` handle, and turns it into a `Ranging<DIM>`. This is because many operations are not allowed while the sensor is sensing. This way, you don't even have access to them.

If you choose to explicitly `.stop()` ranging, you'll get the `VL` (and `Input<'static>`, used for sensing fresh data) back.

### Multiple boards

The library calls this a "flock" of sensors.

```
fn VL::new_flock<_, const BOARDS: usize>(LPns: [Output;BOARDS], &i2c_shared, i2c_addr_gen: impl Fn(usize) -> I2cAddr) -> Result<[VL;BOARDS]>
```

This produces an array of `VL` handles. These handles are operated as a ..well.. "flock", so that you for example start their ranging together:

```
fn [VL;BOARDS]::start_ranging(self, &RangingConfig<DIM>, Input<'static>) -> Result<RangingFlock<N,DIM>>
```

Like with a single board, the "flock" is consumed, and turned into `RangingFlock<N,DIM>` which provides methods for listening to incoming measurements, and stopping the ranging.


## `Ranging<const DIM: usize>` (single board)

An active ranging session.

```
async fn Ranging<DIM>::get_data(&mut self) -> Result<(ResultsData<DIM>, TempC, Instant)>
```

Returns when there's new data available.

- `ResultsData<DIM>` has actual data (and metadata) in matrices. The particular fields depend on the `feature`s you've defined in `Cargo.toml`.
- `TempC` is the temperature of the sensor
- `Instant` is a time stamp approximating when the results were taken (after the measurement; tries to be as close after it as possible)

	The `Instant` can be used to compose multiple measurements - perhaps from different boards - together, relative to each other.


## `RangingFlock<N,DIM>` (multiple boards)

```
async fn RangingFlock<N,DIM>::get_data(&mut self) -> Result<(usize,ResultsData<DIM>,TempC,Instant)>
```

Similar to the single-board `.get_data()`, but provides one more field in the tuple:

- `usize`: board id (0..`{boards}`-1); used for understanding which sensor the measurement belongs to.

Note that results are provided one at a time. This resembles a stream of data, and once Rust is up to "async generators", that's likely how the `get_data()` will be re-implemented. Conseptually, it's already a stream of measurements.


## `RangingConfig::<const DIM: usize>`

Configuration for a ranging session. See the sources for the details.

Compared to ULD C API (which you don't need to know), setting of dimension happens as a generic `const` parameter. Valid values are either `<4>` (for 4x4 results) and `<8>` (for 8x8). Limitations on integration time, scanning frequencies are different, based on the resolution you choose.


## "Missing" features

The VL53L5CX sensor can do more than described above.

We are only going to be introducing features if there's an actual use case for them (i.e. someone willing to take the QA burden on such features). Having 100% parity with the ULD C API is not the aim - better UX is, and so is reliability of exposed features.

### Low power modes

Can be done. Would be interesting.

Needs someone who's willing to measure the power consumption, document stuff, and - frankly - own that side of the project.

Until the author has a need for low power ranging, ..it's a nah.

 🐽

### Built-in gesture recognition

The use case for those (as part of the sensor itself) is likely low power, simple applications.

The author is more interested in re-implementing such on top of these libraries, in Rust. This would allow us to have more access to the implementation of such features - while the ULD C API is publicly available by the vendor, the firmware source code is not.

