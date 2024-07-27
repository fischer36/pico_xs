#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{gpio, registers, xs};
use registers::Watchdog;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut watchdog: watcdog = registers::Watchdog::new();
    watcdog.tick.cycles(1 << 2 | 1 << 3);
    watchdog.tick.enable();

    let mut xocs = registers::xosc::Xosc::new();

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
