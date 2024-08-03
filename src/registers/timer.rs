use super::resets;
use crate::xs::Bits;

pub const BASE: u32 = 0x40054000;
pub const TIMER0: u32 = BASE + 0x420;
pub const TIMER1: u32 = BASE + 0x424;
pub const TIMER2: u32 = BASE + 0x428;
pub const TIMER3: u32 = BASE + 0x42c;

fn get_time() -> u64 {
    let time_lr: *const u32 = (BASE + 0x0c) as *const u32;
    let time_hr: *const u32 = (BASE + 0x08) as *const u32;
    unsafe {
        let lo = core::ptr::read_volatile(time_lr);
        let hi = core::ptr::read_volatile(time_hr);
        ((hi as u64) << 32) | (lo as u64)
    }
}

/* All timers work with all alarms
 * timer - PPB_BASE: TIMER_IRQ_0, TIMER_IRQ_1, TIMER_IRQ_2, TIMER_IRQ_3
 * alarm - TIMER_BASE:ALARM0, ALARM1, ALARM2, ALARM3
 * */
pub fn set_timer(timer: u32, alarm: u32) {
    // Reset Timer
    super::resets::reset_wait(resets::RESET_BIT_TIMER);
    if alarm > 3 {
        panic!("Invalid alarm number");
        // There is only alarm_0, alarm_1, alarm_2, and alarm_3.
    }

    if timer > 3 {
        panic!("Invalid timer number");
        // There is only irq: timer_0, timer_1, timer_2, and timer_3.
    }
    unsafe {
        // Enable the interrupt in inte register by writing to our specific alarm bit
        let timer_inte: *mut u32 = (BASE + 0x38) as *mut u32;
        let old = core::ptr::read_volatile(timer_inte);
        core::ptr::write_volatile(timer_inte, old | (1 << alarm));

        // Enable interrupt handler for TIMER
        super::ppb_base::enable_interrupt(super::ppb_base::Interrupt::TIMER_IRQ_0);

        // get current time
        let current_time_micro: u64 = get_time();
        let half_second: u64 = 500_0000;

        // gets out specific alarm register
        let alarm: *mut u32 = (BASE + 0x10 + 0x4 * alarm) as *mut u32;
        core::ptr::write_volatile(alarm, (current_time_micro + half_second) as u32);
    }
}
