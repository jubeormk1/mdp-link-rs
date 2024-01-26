# Sniffer

It listens for the ESB packets and sends them to the **UART**

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

## UART Connections

If you wish to undertand better the BSP I suggest reading the file `nrf52840-mdk/board.rs` where details of the available peripheral exposed by board is described. If you do not have time for that here goes the piece of information all you need to know for now is that you need to connect a 3.3V USB to serial to the next pins:

- **Pin19**: Is the MDK TX, so connect the **Serial3V3.RX**
- **GND**: connect the **Serial3V3.GND** here
- Pin20: Is the MDK RX, no connection required