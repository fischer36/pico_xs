# pico_xs
Pico XS is an independent, bare-metal, all-in-one SDK for embedded Rust development on the ARM Cortex-M0+ Raspberry Pi Pico microcontroller. This project offers minimal, low-level access to the hardware features of the microcontroller, providing developers with precise control and the ability to work directly with the hardware.
## 🚧 Disclaimer 
This project is **in development**, which means it is not ready for library use *yet*, however you can clone the repository and build it yourself.
## 🛠️ Getting Started
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
## 🗺️ Roadmap
- [ ] Develop Custom Stage 2 Bootloader Firmware.
- [ ] Publish on Crates.io.
## 📜 License
The contents of this repository (excluding *boot2.bin*) are licensed under the Apache License 2.0. See the LICENSE file for details.

The boot2.bin file is licensed under the BSD-3-Clause License. See the LICENSE-BSD3 file for details.
