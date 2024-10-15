# pico_xs
A completely dependency-free, bare-metal hardware abstraction layer for the Raspberry Pi Pico microcontroller.
## Disclaimer 
This project is **in development**, which means it is not ready for library use *yet*, however you can clone the repository and build it yourself.
## Getting Started
Step 1 - Clone the repository
```bash
git clone https://github.com/fischer36/pico_xs
cd pico_xs
```
Step 2 - Install elf2uf2-rs
```bash
cargo install elf2uf2-rs --locked
```
Step 3 - Build and run an example (with Pico in USB Bootloader mode)
```bash
cargo run --example blinky
```
## License
The contents of this repository (excluding *boot2.bin*) are licensed under the Apache License 2.0. See the LICENSE file for details.

The boot2.bin file is licensed under the BSD-3-Clause License. See the LICENSE-BSD3 file for details.
