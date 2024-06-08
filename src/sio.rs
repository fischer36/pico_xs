use crate::regs;

pub fn oe_set(gpio: u32) {
    unsafe {
        const SIO_OE_SET: *mut u32 = (regs::SIO_BASE + 0x024) as *mut u32;
        let mask = 1 << gpio;
        core::ptr::write_volatile(SIO_OE_SET, mask);
    }
}
pub fn oe_clr(gpio: u32) {
    unsafe {
        const SIO_OE_CLR: *mut u32 = (regs::SIO_BASE + 0x028) as *mut u32;
        let mask = 1 << gpio;
        core::ptr::write_volatile(SIO_OE_CLR, mask);
    }
}
pub fn out_set(gpio: u32) {
    unsafe {
        const SIO_OUT_SET: *mut u32 = (regs::SIO_BASE + 0x014) as *mut u32;
        let mask = 1 << gpio;
        let value = core::ptr::read_volatile(SIO_OUT_SET);
        core::ptr::write_volatile(SIO_OUT_SET, value | mask);
    }
}
pub fn out_clr(gpio: u32) {
    unsafe {
        const SIO_OUT_CLR: *mut u32 = (regs::SIO_BASE + 0x018) as *mut u32;
        let mask = 1 << gpio;
        let value = core::ptr::read_volatile(SIO_OUT_CLR);
        core::ptr::write_volatile(SIO_OUT_CLR, value | mask);
    }
}
