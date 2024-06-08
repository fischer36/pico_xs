use crate::regs;

pub fn gpio_ctrl(gpio: u32) {
    let ctrl_offset = match gpio {
        15 => 0x07c,
        25 => 0x0cc,
        _ => 0x07c,
    };
    let gpio_ctrl: *mut u32 = (regs::IO_BANK0_BASE + ctrl_offset) as *mut u32;
    unsafe {
        let value = gpio_ctrl.read_volatile();
        gpio_ctrl.write_volatile((value & !0b11111) | 5);
    }
}

pub fn gpio_in(gpio: u32) -> bool {
    let gpio_in: *mut u32 = (regs::SIO_BASE + 0x004) as *mut u32;
    unsafe {
        let value = core::ptr::read_volatile(gpio_in);
        let pin_state = (value & (1 << gpio)) == 0;
        return pin_state;
    }
}
