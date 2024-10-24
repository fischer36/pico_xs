//! # Watchdog Example
//!
//! This program demonstrates the functionality of the watchdog timer. The program initializes the
//! watchdog and turns on the LED. As long as the watchdog is kicked (refreshed) in the main loop,
//! the LED will remain on. If the watchdog kick line is commented out, the system will reset,
//! turning the LED off and on repeatedly, illustrating how the watchdog works.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{gpio, registers, xs};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Reset IO_BANK0 (bit 5) for GPIO operations
    registers::resets::reset_wait(1 << 5);

    // Create Watchdog instance
    let watchdog = registers::watchdog::Watchdog::new(1_200_000);

    // Initialize XOSC (external oscillator)
    registers::xosc::init();

    // Start Watchdog tick timer
    watchdog.tick();

    // Initialize GPIO pin 25 (LED)
    let led_gpio = gpio::Gpio::new(25);

    // Configure GPIO pin 25 as output for the LED
    led_gpio.select_funcsel(5); // SIO funcsel
    led_gpio.output_enable(true); // Enable output

    // Turn on the LED - it stays on as long as the system doesn't reset
    led_gpio.output_set(true);

    // Start the watchdog timer
    watchdog.start();

    // Main loop: kick the watchdog and keep the LED on
    loop {
        // Kick (refresh) the watchdog timer to prevent a system reset.
        // If this line is commented out, the watchdog will trigger a reset after the timeout.
        watchdog.kick();

        // Sleep to simulate normal operation between kicks
        xs::sleep_small();
    }
}
