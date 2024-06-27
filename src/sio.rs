use core::ptr::null_mut;

use crate::regs;

pub fn oe_set(gpio: u32) {
    unsafe {
        let sio_gpio_regs = &mut *((regs::SIO_BASE + 0x014) as *mut SioGpioRegisters);
        sio_gpio_regs.gpio_oe_set |= 1 << gpio;
    }
}
pub fn oe_clr(gpio: u32) {
    unsafe {
        let sio_gpio_regs = &mut *((regs::SIO_BASE + 0x014) as *mut SioGpioRegisters);
        sio_gpio_regs.gpio_oe_clr |= 1 << gpio;
    }
}

#[repr(C)]
pub struct SioGpioRegisters {
    gpio_out_set: u32,
    gpio_out_clr: u32,
    gpio_out_xor: u32,
    gpio_oe: u32,
    gpio_oe_set: u32,
    gpio_oe_clr: u32,
}

pub fn out_set_asm(gpio: u32) {
    unsafe {
        let gpio_out_set_addr = regs::SIO_BASE + 0x14;
        let bit_mask = 1 << gpio;
        core::arch::asm!(
            "str {value}, [{addr}]", // Store value (bitmask) in addr (out_set)
            value = in(reg) bit_mask, // tells the compiler to pass rust variable into asm.
            addr = in(reg) gpio_out_set_addr, // here aswell
            options(nostack) // tells compiler that it is stack-less
        );
    }
}
pub fn out_set(gpio: u32) {
    unsafe {
        let sio_gpio_regs = &mut *((regs::SIO_BASE + 0x014) as *mut SioGpioRegisters);
        sio_gpio_regs.gpio_out_set |= 1 << gpio;
    }
}

pub fn out_clr(gpio: u32) {
    unsafe {
        let sio_gpio_regs = &mut *((regs::SIO_BASE + 0x014) as *mut SioGpioRegisters);
        sio_gpio_regs.gpio_out_clr |= 1 << gpio;
    }
}
