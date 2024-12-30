//! # Timer Interrupt (Alarm) Example
//!
//! Program sets a timer interrupt for alarm 0. After 5 seconds the interrupt is triggered and Pico is put back into
//! USB bootloader mode.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{gpio, registers, xs};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Reset IO_BANK0 (bit 5) and TIMER (bit 21)
    registers::resets::reset_wait(1 << 5 | 1 << 21);

    // Initialize GPIO pin 25 (LED)
    let gpio = gpio::Gpio::new(25);

    // Select funcsel
    gpio.select_funcsel(5);

    // Enable output
    gpio.output_enable(true);

    // select funcsel
    gpio.select_funcsel(5);

    // enable output
    gpio.output_enable(true);

    // turn on the led
    gpio.output_set(true);

    // Enable TIMER0 interrupt
    registers::ppb_base::clear_pending(registers::ppb_base::Interrupt::TIMER_IRQ_0);
    registers::ppb_base::enable_interrupt(registers::ppb_base::Interrupt::TIMER_IRQ_0);

    // Get timer peripheral
    let mut timer = registers::timer::TimerPeripheral::new();

    // Enable alarm 0
    timer.alarm_0.enable();

    // Set alarm 0 to fire in 10 seconds
    timer.alarm_0.set_time(timer.get_time() + 5000000);

    loop {
        xs::sleep();
    }
}

// Custom interrupt handler for timer 0, it will simply reset the Pico back to USB bootloader mode.
#[no_mangle]
extern "C" fn TIMER_IRQ_0() {
    unsafe {
        let fn_ptr: *mut u32 = registers::rom::get_fn('U', 'B');

        let reset_to_usb_boot: registers::rom::ResetToUsbBootFn =
            unsafe { core::mem::transmute(fn_ptr) };
        unsafe {
            reset_to_usb_boot(0, 0);
        }
    }
}
