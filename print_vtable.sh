
#!/bin/zsh

set -e

arm-none-eabi-objdump -s --section .vector_table target/thumbv6m-none-eabi/debug/pico-xs
arm-none-eabi-objdump -D -j .vector_table target/thumbv6m-none-eabi/debug/pico-xs


