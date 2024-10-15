//! # GPIO Input (Button) Example
//!
//! Program blinks LED when GPIO 15 is low - button press causing GROUND.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{gpio, registers, xs};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Reset IO_BANK0 (bit 5)
    registers::resets::reset_wait(1 << 5);

    // Initialize GPIO pin 25 (LED)
    let gpio_led = gpio::Gpio::new(25);
    // Select funcsel
    gpio_led.select_funcsel(5);
    // Enable output
    gpio_led.output_enable(true);

    // Initialize GPIO pin 15 (Button)
    let gpio_button = gpio::Gpio::new(15);
    // Select funcsel
    gpio_button.select_funcsel(5);
    // Set pull up resistor for button GPIO
    gpio_button.set_pull(registers::pads_bank::Pull::Up);

    loop {
        if gpio_button.input_high() {
            // Button is not pressed; keep LED on
            gpio_led.output_set(true);
        } else {
            // Button is pressed; turn LED off
            gpio_led.output_set(false);
        }
        xs::sleep();
    }
}
