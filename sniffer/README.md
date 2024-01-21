# Sniffer

It listens for the ESB packets and sends them to the UART

To build the hex:

```bash
cargo build --release
arm-none-eabi-objcopy -O ihex ../target/thumbv7em-none-eabihf/release/sniffer sniffer.hex
```

Then upload it into the nrf52840-mdk USB dongle using nrf Connect.

Alternatively convert it to uf2 to upload it using [UF2 Bootloader](https://wiki.makerdiary.com/nrf52840-mdk-usb-dongle/programming/uf2boot/)

```bash
uf2conv -f 0xADA52840 -c -o sniffer.uf2 sniffer.hex
```
