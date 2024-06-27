use crate::regs;
pub fn reset_wait(bit: u32) {
    unsafe {
        let value = core::ptr::read_volatile(regs::RESETS_RESET_CLR as *mut u32);
        core::ptr::write_volatile(regs::RESETS_RESET_CLR as *mut u32, 1 << bit);

        // while core::ptr::read_volatile(regs::RESETS_RESET_DONE as *mut u32) & !(1 << bit) == 0 {
        // TODO! Should loop to check when out of reset
        for _ in 0..1_000 {
            core::arch::asm!("nop");
        }
    }
}
pub fn reset() {
    unsafe {
        // reset io bank (5) & timer (21)
        let xd_mask: u32 = (1 << 5) | (1 << 21) | (1 << 24);
        core::ptr::write_volatile(regs::RESETS_RESET_CLR as *mut u32, xd_mask);

        // wait for reset to be finished.
        for _ in 0..50_000 {
            core::arch::asm!("nop");
        }
    }
}

pub fn reset_mask(mask: u32) {
    unsafe {
        core::ptr::write_volatile(regs::RESETS_BASE as *mut u32, mask);
    }
}

pub fn unreset_ctrl(bit: u32) {
    unsafe {
        let old = core::ptr::read_volatile((regs::RESETS_BASE) as *mut u32);
        core::ptr::write_volatile((regs::RESETS_BASE) as *mut u32, old & !(1 << bit));
    }
}

pub fn wait_reset(bit: u32) {
    let reset_done_offset: u32 = 0x8;
    let reset_done: *const u32 = (regs::RESETS_BASE + reset_done_offset) as *const u32;

    let mask = 1 << bit;

    unsafe {
        loop {
            let old: u32 = core::ptr::read_volatile(reset_done);
            if (old & mask) <= 0 {
                return;
            } else {
                crate::sleep();
            }
        }
    }
}
