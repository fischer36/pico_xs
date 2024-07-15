# pico_xs
A completely dependency-free, bare-metal hardware abstraction layer for the Raspberry Pi Pico microcontroller.

## Getting Started
### Step 1: Clone the repository
Clone the `pico_xs` repository to your system using Git. Open your terminal and run:
```bash
git clone https://github.com/fischer36/pico_xs
cd pico_xs
```
### Step 2: Install elf2uf2-rs
Install *elf2uf2-rs*, a tool to convert the compiled program into the UF2 format required by the Raspberry Pi Pico. Run the following command in your terminal:
```bash
cargo install elf2uf2-rs --locked
```
### Step 3: Build and run the example
Build and run the blinky example, which demonstrates basic functionality by blinking the LED on the Raspberry Pi Pico. Execute this command in the terminal:
```bash
cargo run --example blinky
```
