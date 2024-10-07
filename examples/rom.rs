//! # ROM function usage (reset_to_usb_boot) Example
//!
//! Program immediatel resets back into USB bootloader mode from software.

#![no_std]
#![no_main]

extern crate pico_xs as hal;
use hal::{registers, xs};

#[no_mangle]
pub extern "C" fn main() -> ! {
    let fn_ptr: *mut u32 = registers::rom::get_fn('U', 'B');

    let reset_to_usb_boot: registers::rom::ResetToUsbBootFn =
        unsafe { core::mem::transmute(fn_ptr) };
    unsafe {
        reset_to_usb_boot(0, 0);
    }
    loop {
        xs::sleep();
    }
}
