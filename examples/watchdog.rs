//! # Watchdog Example
//!
//! Program initializes watchdog and then blinks LED in a loop. If wathdog doesn't get kicked
//! (xd_load_counter) it triggers a watchdog reset.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{
    gpio,
    registers::{self, watchdog},
    xs,
};
use xs::Bits;

#[no_mangle]
pub extern "C" fn main() -> ! {
    xs::sleep();
    xs::sleep();
    registers::resets::reset_wait(1 << 5);
    let mut gpio = gpio::Gpio::new(25);
    registers::watchdog::xd_init();
    registers::xosc::init();
    registers::watchdog::xd_tick(12);

    while !registers::watchdog::xs_is_running() {
        xs::sleep();
    }

    registers::watchdog::xd_load_counter(2000);
    gpio.oe.clr();
    gpio.out.clr();
    gpio.select_funcsel(5);

    registers::watchdog::xd_load_counter(1000);
    gpio.oe.set();
    gpio.out.set();

    loop {
        registers::watchdog::xd_load_counter(1000);
    }
}
