#!/bin/bash
clear;
rm ./sniffer-target.hex
rm ./sniffer-target.uf2

cargo build --release
if [ $? -eq 0 ]
then
    clear;
    echo "Successful built!"
    arm-none-eabi-objcopy -O ihex ../target/thumbv7em-none-eabihf/release/sniffer-target sniffer-target.hex
    uf2conv -f 0xADA52840 -c -o sniffer-target.uf2 sniffer-target.hex
    echo "**sniffer-target.uf2** generated!"
else
    echo "Unsuccessful built!! No uf2 updated. Check built messages"

fi