# Gotchas

There are some fine print in the specs that you'd be good to be aware of, in the design phase.

## Multiple targets has a minimum distance limitation!!

>"The **minimum distance between two targets** to be detected is **600 mm**."
>
>source: UM3109 - Rev 11 (page 12)

This has been tucked into the spec as a side note, but matters immensely.

It's also good to be aware of this, when reading multiple-targets results, in development. They should never be less than 60cm apart.
