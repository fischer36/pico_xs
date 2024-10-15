//! # Xosc (12MHz) Example
//!
//! Program uses the crystal oscillator clock.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{
    gpio,
    registers::{self, xosc},
    xs,
};

#[no_mangle]
pub extern "C" fn main() -> ! {
    registers::xosc::init();
    registers::resets::reset_wait(1 << 5);
    let mut gpio = gpio::Gpio::new(25);
    gpio.oe.clr();
    gpio.out.clr();
    gpio.select_funcsel(5);
    gpio.oe.set();
    gpio.out.set();
    loop {
        xs::sleep();
        gpio.out.clr();
        xs::sleep();
        gpio.out.set();
    }
}
