# Pico XS 
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg) [![Crates.io](https://img.shields.io/crates/v/pico_xs.svg)](https://crates.io/crates/pico_xs)


Pico XS is an independent, bare-metal, all-in-one SDK for embedded Rust development on the ARM Cortex-M0+ Raspberry Pi Pico microcontroller. This project offers minimal, low-level access to the hardware features of the microcontroller, providing developers with precise control and the ability to work directly with the hardware.
## Table of Contents
1. [🛠️ Getting Started](#️-getting-started)
2. [📝 Documentation](#-documentation)
3. [🗺️ Roadmap](#️-roadmap)
4. [📜 License](#-license)

## 🛠️ Getting Started

### Prerequisites
```bash
# Add the ARM Cortex-M0+ compilation target required for the Pico
rustup target add thumbv6m-none-eabi

# Install elf2uf2-rs to convert ELF binaries to UF2 format
cargo install elf2uf2-rs --locked
```

### Using Pico-XS SDK
```bash
# Include the pico_xs SDK in Your Project
cargo add pico_xs

# Compile the Project for the Raspberry Pi Pico Using the ARM Target
cargo build --release --target thumbv6m-none-eabi

# Convert the Binary to UF2 Format
elf2uf2-rs target/thumbv6m-none-eabi/release/pico_xs.elf -o pico_xs.uf2 

# Flash the UF2 File to the Pico XS
# Simply move the resulting pico_xs.uf2 file to the Pico's USB mass storage device (appears as RPI-RP2 when in bootloader mode).
```

### Running Pico-XS SDK Examples
```bash
# Clone the Repository
git clone https://github.com/fischer36/pico_xs
cd pico_xs

# Install elf2uf2-rs
cargo install elf2uf2-rs --locked

# Compile and Run the blinky Example
# Ensure the Pico is in USB Bootloader Mode before running this command.
cargo run --example blinky
```

## 📝 Documentation

### Compilation Process
Pico XS is exclusively designed for embedded development on the Raspberry Pi Pico microcontroller, requiring a strict compilation process in accordance with the microcontroller's specifications. Pico XS provides all the necessary compilation steps by default, requiring only the user to set the appropriate target. Here's an overview of the compilation steps:
1. Linker Script - The provided linker script [link.ld](./link.ld) is required to match the Pico's memory layout and provide essential boilerplate specifics to the RP2040 chip.
2. Target Configuration - The Raspberry Pi Pico uses the thumbv6m-none-eabi architecture. The Rust target needs to be set appropriately: ``rustup target add thumbv6m-none-eabi``
3. Compiling the Project - Compile the project using Cargo with the specified target: ``cargo build --release --target thumbv6m-none-eabi``
4. Convert and Flash - Use elf2uf2-rs to convert the ELF binary to UF2 format and then simply move the resulting UF2 file to to the Pico (The Pico needs to be USB Bootloader Mode)
## 🗺️ Roadmap
- [x] Crates.io Release
- [ ] Develop Custom Stage 2 Bootloader Firmware.
- [ ] Sample Project

## 📜 License
The contents of this repository (excluding *boot3.bin*) are licensed under the [Apache License 2.0](LICENSE). See the `LICENSE` file for details. The `boot3.bin` file is licensed under the [BSD-3-Clause License](LICENSE-BSD3). See the `LICENSE-BSD3` file for details.
