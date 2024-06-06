pub mod resets {
    pub fn reset() {
        const RESETS_BASE: u32 = 0x4000C000;
        const RESETS_RESET_CLR: u32 = RESETS_BASE + 0x0 + 0x3000;
        unsafe {
            let xd_mask: u32 = 1 << 5;
            core::ptr::write_volatile(RESETS_RESET_CLR as *mut u32, xd_mask);

            for _ in 0..50_000 {
                core::arch::asm!("nop");
            }
        }
    }
}
pub mod pads {
    const PADS_BANK0_BASE: u32 = 0x4001c000;
    pub fn set_pulls(gpio: u32, up: bool, down: bool) {
        let pads_ctrl_offset = match gpio {
            15 => 0x40,
            _ => 0x40,
        };
        const PADS_BANK0_GPIO0_PUE_LSB: u32 = 3;
        const PADS_BANK0_GPIO0_PDE_LSB: u32 = 2;
        let up_bit = (up as u32) << PADS_BANK0_GPIO0_PUE_LSB;
        let down_bit = (down as u32) << PADS_BANK0_GPIO0_PDE_LSB;
        let mask = (1 << PADS_BANK0_GPIO0_PUE_LSB) | (1 << PADS_BANK0_GPIO0_PDE_LSB);
        let pads_gpio_reg = (PADS_BANK0_BASE + pads_ctrl_offset) as *mut u32;
        unsafe {
            let current_value = core::ptr::read_volatile(pads_gpio_reg);
            let new_value = (current_value & !mask) | up_bit | down_bit;

            core::ptr::write_volatile(pads_gpio_reg, new_value);
        }
    }
}

pub mod io {
    const IO_BANK0_BASE: u32 = 0x40014000;
    pub fn gpio_ctrl(gpio: u32) {
        let ctrl_offset = match gpio {
            15 => 0x07c,
            25 => 0x0cc,
            _ => 0x07c,
        };
        let gpio_ctrl: *mut u32 = (IO_BANK0_BASE + ctrl_offset) as *mut u32;

        unsafe {
            let value = gpio_ctrl.read_volatile();
            gpio_ctrl.write_volatile((value & !0b11111) | 5);
        }
    }
}

pub mod sio {
    const SIO_BASE: u32 = 0xd000_0000;
    pub fn gpio_in(gpio: u32) -> bool {
        let gpio_in: *mut u32 = (SIO_BASE + 0x004) as *mut u32;
        unsafe {
            let value = core::ptr::read_volatile(gpio_in);
            let pin_state = (value & (1 << gpio)) == 0;
            return pin_state;
        }
    }
    pub fn oe_set(gpio: u32) {
        unsafe {
            const SIO_OE_SET: *mut u32 = (SIO_BASE + 0x024) as *mut u32;
            let mask = 1 << gpio;
            core::ptr::write_volatile(SIO_OE_SET, mask);
        }
    }
    pub fn oe_clr(gpio: u32) {
        unsafe {
            const SIO_OE_CLR: *mut u32 = (SIO_BASE + 0x028) as *mut u32;
            let mask = 1 << gpio;
            core::ptr::write_volatile(SIO_OE_CLR, mask);
        }
    }
    pub fn out_set(gpio: u32) {
        unsafe {
            const SIO_OUT_SET: *mut u32 = (SIO_BASE + 0x014) as *mut u32;
            let mask = 1 << gpio;
            let value = core::ptr::read_volatile(SIO_OUT_SET);
            core::ptr::write_volatile(SIO_OUT_SET, value | mask);
        }
    }
    pub fn out_clear(gpio: u32) {
        unsafe {
            const SIO_OUT_CLR: *mut u32 = (SIO_BASE + 0x018) as *mut u32;
            let mask = 1 << gpio;
            let value = core::ptr::read_volatile(SIO_OUT_CLR);
            core::ptr::write_volatile(SIO_OUT_CLR, value | mask);
        }
    }
}
