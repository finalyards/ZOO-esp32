# Troubleshooting

## ESP32-C3 dev board not responding

Occasionally, they need resetting to a "download state". Do this by:

- Press both `RESET` and `BOOT` buttons
- Release `RESET`
- Release `BOOT`
- reconnect using USB/IP: e.g. `sudo usbip attach -r 192.168.1.29 -b 3-1`
- try flashing and running

