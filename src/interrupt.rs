pub enum IRQ {
    USBCTRL = 5,
}

use crate::regs;
pub fn irq_set_enable(bit: u32) {
    let nvic_iser: u32 = 0xe100;

    let interrupt_set_enable: *mut u32 = (regs::PPB_BASE + nvic_iser) as *mut u32;
    unsafe {
        let old = core::ptr::read_volatile(interrupt_set_enable);
        core::ptr::write_volatile(interrupt_set_enable, 1 << bit);
    }
}

pub fn irq_set_disable(bit: u32) {
    let nvic_iser: u32 = 0xe100;
    let interrupt_set_enable: *mut u32 = (regs::PPB_BASE + nvic_iser) as *mut u32;
    unsafe {
        let old = core::ptr::read_volatile(interrupt_set_enable);
        core::ptr::write_volatile(interrupt_set_enable, old & !(1 << bit));
    }
}
