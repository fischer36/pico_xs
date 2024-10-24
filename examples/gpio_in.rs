//! # GPIO Input (Button) Example
//!
//! This program blinks an LED when GPIO pin 15 (connected to a button) is low (button press).

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
    // Set function select to SIO
    gpio_led.select_funcsel(5);
    // Enable output for the LED
    gpio_led.output_enable(true);

    // Initialize GPIO pin 15 (Button)
    let gpio_button = gpio::Gpio::new(15);
    // Set function select to SIO
    gpio_button.select_funcsel(5);
    // Enable pull-up resistor for the button input (active low)
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
