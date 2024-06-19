use crate::regs;
pub const TIMER0: u32 = regs::TIMER_BASE + 0x420;
pub const TIMER1: u32 = regs::TIMER_BASE + 0x424;
pub const TIMER2: u32 = regs::TIMER_BASE + 0x428;
pub const TIMER3: u32 = regs::TIMER_BASE + 0x42c;

fn get_time() -> u64 {
    let time_lr: *const u32 = (regs::TIMER_BASE + 0x0c) as *const u32;
    let time_hr: *const u32 = (regs::TIMER_BASE + 0x08) as *const u32;
    unsafe {
        let lo = core::ptr::read_volatile(time_lr);
        let hi = core::ptr::read_volatile(time_hr);
        ((hi as u64) << 32) | (lo as u64)
    }
}

pub fn set_timer() {
    unsafe {
        // [!] VERIFIED WORKS - Enable the interrupt set bit to 1 for our alarm (alarm 0)
        let timer_inte: *mut u32 = (regs::TIMER_BASE + 0x38) as *mut u32;
        let old = core::ptr::read_volatile(timer_inte);
        core::ptr::write_volatile(timer_inte, old | (1 << 0));

        //
        // Enable interrupt handler
        const PPB_BASE: u32 = 0xe0000000;
        let nvic_iser: u32 = 0xe100;
        let interrupt_set_enable: *mut u32 = (PPB_BASE + nvic_iser) as *mut u32;
        core::ptr::write_volatile(interrupt_set_enable, 1 << 0);

        // get current time
        let current_time_micro: u64 = get_time();
        let half_second: u64 = 500_0000;
        // write time for when alarm should trigger
        let alarm_zero: *mut u64 = (regs::TIMER_BASE + 0x10) as *mut u64;
        core::ptr::write_volatile(alarm_zero, current_time_micro + half_second);
    }
}
