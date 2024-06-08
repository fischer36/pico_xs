use crate::regs;

pub fn set_pulls(gpio: u32, up: bool, down: bool) {
    let pads_ctrl_offset = match gpio {
        15 => 0x40,
        _ => 0x40,
    };

    let up_bit = (up as u32) << regs::PADS_BANK0_GPIO0_PUE_LSB;
    let down_bit = (down as u32) << regs::PADS_BANK0_GPIO0_PDE_LSB;
    let mask = (1 << regs::PADS_BANK0_GPIO0_PUE_LSB) | (1 << regs::PADS_BANK0_GPIO0_PDE_LSB);
    let pads_gpio_reg = (regs::PADS_BANK0_BASE + pads_ctrl_offset) as *mut u32;
    unsafe {
        let current_value = core::ptr::read_volatile(pads_gpio_reg);
        let new_value = (current_value & !mask) | up_bit | down_bit;

        core::ptr::write_volatile(pads_gpio_reg, new_value);
    }
}
