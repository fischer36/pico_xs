use crate::regs;

pub fn reset() {
    unsafe {
        // reset io bank (5) & timer (21)
        let xd_mask: u32 = (1 << 5) | (1 << 21) | (1 << 24);
        core::ptr::write_volatile(regs::RESETS_RESET_CLR as *mut u32, xd_mask);

        // Wait for reset to be finished.
        for _ in 0..50_000 {
            core::arch::asm!("nop");
        }
    }
}

pub fn reset_ctrl(bit: u32) {
    let bit_mask = 1 << bit;
    let reset_reset_ctrl: u32 = 0x0;
    unsafe {
        let old = core::ptr::read_volatile((regs::RESETS_BASE + reset_reset_ctrl) as *mut u32);
        core::ptr::write_volatile(
            (regs::RESETS_BASE + reset_reset_ctrl) as *mut u32,
            old | bit_mask,
        );
    }

    // unsafe {
    //     for _ in 0..50_000 {
    //         core::arch::asm!("nop");
    //     }
    // }
}

pub fn unreset_ctrl(bit: u32) {
    let bit_mask = !(1 << bit);
    let reset_reset_ctrl: u32 = 0x0;
    unsafe {
        core::ptr::write_volatile((regs::RESETS_BASE + reset_reset_ctrl) as *mut u32, bit_mask);
        for _ in 0..50_000 {
            core::arch::asm!("nop");
        }
    }
}
