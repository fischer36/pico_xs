# Pico XS
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg) [![Crates.io](https://img.shields.io/crates/v/pico_xs.svg)](https://crates.io/crates/pico_xs)

Pico XS is an independent, bare-metal, all-in-one SDK for embedded Rust development on the ARM Cortex-M0+ Raspberry Pi Pico microcontroller. This project offers minimal, low-level access to the hardware features of the microcontroller, providing developers with precise control and the ability to work directly with the hardware.

## Table of Contents
- [🛠️ Getting Started](#getting-started)
- [📝 Documentation](#documentation)
- [🗺️ Roadmap](#roadmap)
- [📜 License](#license)

## 🛠️ Getting Started

### Prerequisites
```bash
# Add the ARM Cortex-M0+ compilation target required for the Pico
rustup target add thumbv6m-none-eabi

# Install elf2uf2-rs to convert ELF binaries to UF2 format
cargo install elf2uf2-rs --locked
```

### Using Pico XS SDK
You can either build from the [pico-xs-project-template](https://github.com/fischer36/pico-xs-project-template) or manually configure your project:
- You need to copy the [link.ld](./link.ld) and [build.rs](./build.rs) files to the root of your project. These files are necessary for compilation.
- Include the pico_xs SDK in your project:
```bash
cargo add pico_xs
```
- Compile the Project for the Raspberry Pi Pico using the ARM target:
```bash
cargo build --release --target thumbv6m-none-eabi
```
- Convert the binary to UF2 format:
```bash
elf2uf2-rs target/thumbv6m-none-eabi/release/<project_name>.elf <project_name>.uf2
```
- Flash the UF2 file to the Pico by simply moving the UF2 file into the Pico's USB mass storage device.

### Running Pico XS SDK Examples
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
- **Linker Script** - The provided linker script [link.ld](./link.ld) is required to match the Pico's memory layout and provide essential boilerplate specifics to the RP2040 chip like bootloader2 and system handlers.
- **Target Configuration** - The Raspberry Pi Pico uses the thumbv6m-none-eabi architecture. The Rust target needs to be set appropriately:
```bash
rustup target add thumbv6m-none-eabi
```
- **Compiling the Project** - Compile the project using Cargo with the specified target:
```bash
cargo build --release --target thumbv6m-none-eabi
```
- **Convert and Flash** - Use elf2uf2-rs to convert the ELF binary to UF2 format and then simply move the resulting UF2 file to the Pico (The Pico needs to be in USB Bootloader Mode).

## 🗺️ Roadmap
- [x] Crates.io Release
- [x] Sample Project
- [ ] Develop Custom Stage 2 Bootloader Firmware

## 📜 License
The contents of this repository (excluding *boot2.bin*) are licensed under the [Apache License 2.0](LICENSE). See the `LICENSE` file for details. The `boot2.bin` file is licensed under the [BSD-3-Clause License](LICENSE-BSD3). See the `LICENSE-BSD3` file for details.


