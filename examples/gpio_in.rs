//! # GPIO Input (Button) Example
//!
//! Program blinks LED when GPIO 15 is high.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{gpio, registers, xs};

#[no_mangle]
pub extern "C" fn main() -> ! {
    registers::resets::reset_wait(1 << 5);
    let mut gpio_led = gpio::Gpio::new(25);

    let mut gpio_button = gpio::Gpio::new(15);
    gpio_button.oe.clr();
    gpio_button.out.clr();
    gpio_button.select_funcsel(5);
    registers::pads_bank::input_enable(15);

    gpio_led.oe.clr();
    gpio_led.out.clr();

    gpio_led.select_funcsel(5);
    gpio_led.oe.set();
    loop {
        if registers::sio::gpio_input_value() & (1 << 15) != 0 {
            gpio_led.out.set();
        } else {
            gpio_led.out.clr();
        }
        xs::sleep();
    }
}
