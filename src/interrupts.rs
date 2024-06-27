const PPB_BASE: u32 = 0xe0000000;
const PPB_NVIC_ISER: *mut u32 = (PPB_BASE + 0xe100) as *mut u32; // Set enable
const PPB_NVIC_ICER: *mut u32 = (PPB_BASE + 0xe180) as *mut u32; // Clr enable

const PPB_NVIC_ISPR: *mut u32 = (PPB_BASE + 0xe200) as *mut u32; // Set pending
const PPB_NVIC_ICPR: *mut u32 = (PPB_BASE + 0xe280) as *mut u32; // Clr pending
                                                                 //
pub fn interrupt_set_enable(bit: u32) {
    unsafe {
        let old = core::ptr::read_volatile(PPB_NVIC_ISER);
        core::ptr::write_volatile(PPB_NVIC_ISER, old | 1 << bit);
    }
}

pub fn interrupt_set_disable(bit: u32) {
    unsafe {
        let old = core::ptr::read_volatile(PPB_NVIC_ICER);
        core::ptr::write_volatile(PPB_NVIC_ICER, old | 1 << bit);
    }
}
#[allow(non_snake_case)]

pub mod M0PLUS {
    use crate::regs;
    const SYST_CSR: *mut u32 = (regs::PPB_BASE + 0xe010) as *mut u32;
    const SYST_RVR: *mut u32 = (regs::PPB_BASE + 0xe014) as *mut u32;
    const SYST_CVR: *mut u32 = (regs::PPB_BASE + 0xe018) as *mut u32;
}
