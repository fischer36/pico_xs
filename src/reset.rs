use crate::regs;

pub fn reset() {
    unsafe {
        // reset io bank (5) & timer (21)
        let xd_mask: u32 = (1 << 5) | (1 << 21);
        core::ptr::write_volatile(regs::RESETS_RESET_CLR as *mut u32, xd_mask);

        // Wait for reset to be finished.
        for _ in 0..50_000 {
            core::arch::asm!("nop");
        }
    }
}
