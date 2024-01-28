#!/bin/bash
clear;
rm ./sniffer.hex
rm ./sniffer.uf2

cargo build --release
if [ $? -eq 0 ]
then
    clear;
    echo "Successful built!"
    arm-none-eabi-objcopy -O ihex ../target/thumbv7em-none-eabihf/release/sniffer sniffer.hex
    uf2conv -f 0xADA52840 -c -o sniffer.uf2 sniffer.hex
    echo "**sniffer.uf2** generated!"
else
    echo "Unsuccessful built!! No uf2 updated. Check built messages"

fi