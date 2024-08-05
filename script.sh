
#!/bin/zsh
set -e
arm-none-eabi-objdump -D -S target/thumbv6m-none-eabi/debug/examples/watchdog> asm
