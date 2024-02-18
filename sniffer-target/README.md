# Sniffer-Target

This is a simple application that will transmit a message over a given channel repetedly so we can validate the sniffer.

To build the hex:

```bash
cargo build --release
arm-none-eabi-objcopy -O ihex ../target/thumbv7em-none-eabihf/release/sniffer-target sniffer-target.hex
```

Then upload it into the nrf52840-mdk USB dongle using nrf Connect.

Alternatively convert it to uf2 to upload it using [UF2 Bootloader](https://wiki.makerdiary.com/nrf52840-mdk-usb-dongle/programming/uf2boot/)

```bash
uf2conv -f 0xADA52840 -c -o sniffer-target.uf2 sniffer-target.hex
```
