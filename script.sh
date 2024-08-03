
#!/bin/zsh

# Ensure any failure in the steps causes the script to exit
set -e

# Define paths

# Run elf2uf2-rs
elf2uf2-rs target/thumbv6m-none-eabi/debug/examples/blinky target/thumbv6m-none-eabi/debug/examples/blinky.uf2

# Generate assembly dump
arm-none-eabi-objdump -D -S target/thumbv6m-none-eabi/debug/examples/blinky > asm

echo "Assembly dump has been generated at $OUTPUT_PATH"
