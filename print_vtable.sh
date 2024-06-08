
#!/bin/zsh

set -e

arm-none-eabi-objdump -s --section .vector_table target/thumbv6m-none-eabi/debug/pico-hal-xs
arm-none-eabi-objdump -d -j .vector_table target/thumbv6m-none-eabi/debug/pico-hal-xs


