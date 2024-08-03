#![no_std]
#![no_main]

// Neet to verify, doesn't work
extern crate pico_xs as hal;
use hal::{
    gpio,
    registers::{self, watchdog},
    xs,
};
use xs::Bits;

#[no_mangle]
pub extern "C" fn main() -> ! {
    registers::watchdog::enable(300);

    registers::clocks::init();
    registers::xosc::init();
    registers::watchdog::start(2);
    registers::timer::set_timer(0, 0);
    registers::resets::reset_wait(1 << 5);
    let mut gpio = gpio::Gpio::new(25);
    gpio.oe.clr();
    gpio.out.clr();
    gpio.select_funcsel(5);
    gpio.oe.set();
    gpio.out.set();
    loop {
        xs::sleep();
        // xs::sleep();
        // gpio.out.clr();
        // xs::sleep();
        // xs::sleep();
        // gpio.out.set();
        registers::watchdog::kick();
    }
}
