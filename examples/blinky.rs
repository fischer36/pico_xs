//! # GPIO Output (LED) Example
//!
//! Program blinks LED in a loop.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{gpio, registers, xs};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Reset IO_BANK0 (bit 5)
    registers::resets::reset_wait(1 << 5);

    // Initialize GPIO pin 25 (LED)
    let gpio = gpio::Gpio::new(25);

    // Select funcsel
    gpio.select_funcsel(5);

    // Enable output
    gpio.output_enable(true);
    loop {
        // Toggle output - LED on and off
        gpio.output_set(true);
        xs::sleep();
        gpio.output_set(false);
        xs::sleep();
    }
}
