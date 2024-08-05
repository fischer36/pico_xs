#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{
    gpio,
    registers::{self, watchdog},
    xs,
};

#[no_mangle]
pub extern "C" fn main() -> ! {
    registers::resets::reset_wait(1 << 5);
    registers::watchdog::enable(300);
    registers::clocks::init();

    let mut gpio = gpio::Gpio::new(25);
    gpio.oe.clr();
    gpio.out.clr();
    gpio.select_funcsel(5);
    gpio.oe.set();
    gpio.out.set();
    loop {
        watchdog::kick();
        xs::sleep();
        gpio.out.clr();
        xs::sleep();
        gpio.out.set();
    }
}
