# Read the Fine Print!!!

There are some fine print in the specs that you'd be good to be aware of, early on.

## Multiple targets has a minimum distance limitation.

>"The **minimum distance between two targets** to be detected is **600 mm**."
>
>source: [UM3109 - Rev 11](https://www.st.com/resource/en/user_manual/um3109-a-guide-for-using-the-vl53l8cx-lowpower-highperformance-timeofflight-multizone-ranging-sensor-stmicroelectronics.pdf) (page 12)

This has been tucked into the spec as a side note, but matters **immensely**.

It's also good to be aware of this, when reading multiple-targets results, in development. They should never be less than 60cm apart.
