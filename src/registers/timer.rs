use super::resets;
use crate::xs::Bits;
pub const BASE: u32 = 0x40054000;

pub const TIMER_INTR: *mut u32 = (BASE + 0x34) as *mut u32;
pub const TIMER_INTE: *mut u32 = (BASE + 0x38) as *mut u32;
pub const TIMER_ARMED: *mut u32 = (BASE + 0x20) as *mut u32;
pub const TIMER_INTF: *mut u32 = (BASE + 0x3C) as *mut u32;

/// Represents a single Timer Alarm.
pub struct Alarm {
    /// The alarm number (0-3).
    pub alarm_number: u8,
}

impl Alarm {
    /// Creates a new `Alarm` instance.
    ///
    /// # Arguments
    ///
    /// * `alarm_number` - The alarm number (0-3).
    ///
    /// # Panics
    ///
    /// Panics if `alarm_number` is greater than 3.
    fn new(alarm_number: u8) -> Self {
        assert!(
            alarm_number < 4,
            "Invalid alarm number. Must be between 0 and 3."
        );
        Self { alarm_number }
    }

    /// Enables the alarm by setting the corresponding bit in the `TIMER_INTE` register.
    pub fn enable(&self) {
        let alarm_bit = self.alarm_number as u32;
        unsafe {
            // Read current TIMER_INTE value
            let current = core::ptr::read_volatile(TIMER_INTE);
            // Set the specific alarm bit without altering other bits
            core::ptr::write_volatile(TIMER_INTE, current | (1 << alarm_bit));
        }
    }

    /// Sets the alarm to trigger at a specific absolute time.
    ///
    /// # Arguments
    ///
    /// * `time` - The absolute time in microseconds when the alarm should trigger.
    ///
    /// # Safety
    ///
    /// This function performs a raw pointer write to hardware registers.
    /// Ensure that `time` does not cause overflow based on hardware specifications.
    pub fn set_time(&self, time: u64) {
        let alarm_offset = 0x10 + 0x4 * self.alarm_number as u32;
        let alarm_ptr = (BASE + alarm_offset) as *mut u32;
        unsafe {
            // Assuming the TIMER_ALARM registers are 32-bit and accept the lower 32 bits of time
            core::ptr::write_volatile(alarm_ptr, time as u32);
        }
    }

    /// Checks if the alarm is currently armed by reading the `TIMER_ARMED` register.
    ///
    /// # Returns
    ///
    /// `true` if the alarm is armed, `false` otherwise.
    pub fn is_armed(&self) -> bool {
        let alarm_bit = self.alarm_number as u32;
        unsafe { (*TIMER_ARMED & (1 << alarm_bit)) != 0 }
    }

    /// Manually triggers the alarm interrupt.
    pub fn trigger(&self) {
        let alarm_bit = self.alarm_number as u32;
        unsafe {
            // Typically, writing '1' to the interrupt flag register sets the interrupt
            let current = core::ptr::read_volatile(TIMER_INTF);
            core::ptr::write_volatile(TIMER_INTF, current | (1 << alarm_bit));
        }
    }

    /// Clears the interrupt flag for the alarm.
    ///
    /// Should be called after handling an interrupt to acknowledge it.
    pub fn clear_interrupt(&self) {
        let alarm_bit = self.alarm_number as u32;
        unsafe {
            // Writing '1' clears the interrupt flag
            TIMER_INTF.set(1 << alarm_bit);
        }
    }
}

/// Represents the Timer Peripheral with multiple alarms.
pub struct TimerPeripheral {
    /// Alarm 0
    pub alarm_0: Alarm,
    /// Alarm 1
    pub alarm_1: Alarm,
    /// Alarm 2
    pub alarm_2: Alarm,
    /// Alarm 3
    pub alarm_3: Alarm,
}

impl TimerPeripheral {
    /// Creates a new `TimerPeripheral` instance with all alarms initialized.
    ///
    /// # Returns
    ///
    /// A `TimerPeripheral` instance with alarms 0-3.
    pub fn new() -> Self {
        Self {
            alarm_0: Alarm::new(0),
            alarm_1: Alarm::new(1),
            alarm_2: Alarm::new(2),
            alarm_3: Alarm::new(3),
        }
    }

    /// Retrieves the current timer count.
    ///
    /// # Returns
    ///
    /// The current time as a `u64` value, combining high and low registers.
    ///
    /// # Safety
    ///
    /// This function performs raw pointer reads to hardware registers.
    /// Ensure that the timer peripheral is correctly initialized before calling.
    pub fn get_time(&self) -> u64 {
        const BASE: u32 = 0x40054000; // Replace with actual TIMER base address
        let time_lr: *const u32 = (BASE + 0x0c) as *const u32;
        let time_hr: *const u32 = (BASE + 0x08) as *const u32;
        unsafe {
            let lo = core::ptr::read_volatile(time_lr);
            let hi = core::ptr::read_volatile(time_hr);
            ((hi as u64) << 32) | (lo as u64)
        }
    }
}
