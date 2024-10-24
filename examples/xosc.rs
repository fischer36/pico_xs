//! # XOSC (12MHz) Example
//!
//! This program sets up the external crystal oscillator (XOSC) at 12 MHz. It then verifies that
//! the XOSC is enabled. While the XOSC is enabled, the LED on GPIO pin 25 blinks. If the XOSC is
//! disabled for any reason, the program stops the blinking.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{gpio, registers, xs};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialize the external crystal oscillator (XOSC)
    registers::xosc::init();

    // Reset IO_BANK0 (bit 5) for GPIO operations
    registers::resets::reset_wait(1 << 5);

    // Initialize GPIO pin 25 (LED)
    let mut led_gpio = gpio::Gpio::new(25);

    // Configure GPIO pin 25 as output for the LED
    led_gpio.select_funcsel(5); // SIO funcsel
    led_gpio.output_enable(true); // Enable output

    // Blink the LED while XOSC is enabled
    loop {
        // Check if the XOSC is enabled
        if !registers::xosc::is_enabled() {
            // If XOSC is disabled, exit the loop and stop blinking
            break;
        }

        // Toggle the LED: ON -> Sleep -> OFF -> Sleep
        led_gpio.output_set(true); // Turn LED on
        xs::sleep(); // Sleep
        led_gpio.output_set(false); // Turn LED off
        xs::sleep(); // Sleep
    }

    // End program - keep the system in a sleep loop
    loop {
        xs::sleep();
    }
}
