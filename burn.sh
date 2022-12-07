#!/bin/sh

arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/blinky-rust blinky-rust.bin
dfu-util -a1 -d 1eaf:0003 -D blinky-rust.bin -R
