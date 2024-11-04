# `v53l5cx`

Higher level abstraction for the ST.com [VL53L5CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html) sensors.

APIs for using either a single, or multiple such sensors at once.


## Requirements

- Follow the steps in the `../vl53l5cx_uld/README.md` 


## Running examples

### Single board

```
$ cargo build --release --features=single,distance_mm,defmt --example single-emb
```

### Multiple boards

```
$ EMBASSY_EXECUTOR_TASK_ARENA_SIZE=50000 \
  cargo build --release --features=flock,distance_mm,defmt --example many-emb
```


## References

- [Breakout Boards for VL53L5CX](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) (ST.com)
- `1`: [PCB4109A, version 12, variant 00B](https://www.st.com/resource/en/schematic_pack/pcb4109a-00b-sch012.pdf) (ST.com; 2021; PDF 2pp.)

	>*Interestingly, marked `CONFIDENTIAL` but behind a public web link.* `#st.com`
