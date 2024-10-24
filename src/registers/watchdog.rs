/// Watchdog Timer for RP2040.
///
/// The watchdog is disabled by default and must be enabled explicitly. This module provides functionality
/// to configure, start, kick (reload), and check the status of the watchdog timer. Additionally, it can
/// be used to trigger resets and handle watchdog-related operations.
///
/// The watchdog timer helps to ensure the system remains functional and can reset the system if it does not
/// receive a kick within a specified timeout period.
use crate::xs::Bits;

const BASE: u32 = 0x40058000;
const CTRL: *mut u32 = BASE as *mut u32;
const LOAD: *mut u32 = (BASE + 0x04) as *mut u32;
const REASON: *mut u32 = (BASE + 0x08) as *mut u32;
const WATCHDOG_NON_REBOOT_MAGIC: u32 = 0x6ab73121;
const WATCHDOG_REBOOT_MAGIC: u32 = 0x6ab73120;

/// Structure representing the Watchdog Timer.
pub struct Watchdog {
    load_counter: u32,
}

impl Watchdog {
    /// Creates a new `Watchdog` instance.
    ///
    /// The watchdog is initialized with a default load counter of 1,200,000 ticks.
    ///
    ///
    /// # Arguments
    /// * `load_value` - value used to update the Watchdog timer in self.tick().
    ///
    /// # Example
    /// ```
    /// let watchdog = Watchdog::new(1_200_000);
    /// ```
    pub fn new(load_value: u32) -> Watchdog {
        Watchdog {
            load_counter: load_value,
        }
    }

    /// Configures and enables the watchdog tick mechanism.
    ///
    /// This function calculates the number of cycles required for the desired tick frequency (1 MHz example) and
    /// enables the tick mechanism.
    pub fn tick(&self) {
        const WATCHDOG_TICK_ENABLE_BITS: u32 = 1 << 9; // Enable bit
        let tick: *mut u32 = (BASE + 0x2c) as *mut u32;
        let cycles = (12_000_000 / 1_000_000) as u32; // Assuming 12 MHz clock
        unsafe {
            tick.write_volatile(WATCHDOG_TICK_ENABLE_BITS | cycles);
        }
    }

    /// Starts the watchdog timer.
    ///
    /// This function resets all peripherals, clears reset bits, and starts the watchdog timer.
    pub fn start(&self) {
        let wdsel = (super::PSM_BASE + 0x8) as *mut u32;
        unsafe {
            wdsel.write_volatile(0x1FFFF); // Reset all peripherals
            wdsel.write_volatile(wdsel.read_volatile() & !3); // Clear XOSC and ROSC reset bits
        }
        unsafe {
            CTRL.write_volatile(0x40000000 | 0xFFF);
        }
    }

    /// Kicks (reloads) the watchdog timer.
    ///
    /// This function resets the watchdog timer to prevent a reset.
    pub fn kick(&self) {
        LOAD.set(self.load_counter);
    }

    /// Checks if the watchdog is running.
    ///
    /// Returns `true` if the watchdog is currently running, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// if watchdog.running() {
    ///     // Watchdog is active
    /// }
    /// ```
    pub fn running(&self) -> bool {
        let tick: *mut u32 = (BASE + 0x2c) as *mut u32;
        unsafe { tick.read_volatile() & (1 << 10) != 0 }
    }

    /// Manually triggers a watchdog reset.
    ///
    /// This function forces the watchdog to trigger a system reset.
    pub fn trigger(&self) {
        CTRL.set(1 << 31);
    }

    /// Checks if the system was reset by the watchdog.
    ///
    /// Returns `true` if the last reset was caused by the watchdog, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// if watchdog.was_reset() {
    ///     // System was reset by the watchdog
    /// }
    /// ```
    // TODO: Check if this even works
    pub fn was_reset(&self) -> bool {
        let reason: *mut u32 = (BASE + 0x08) as *mut u32;
        unsafe {
            let bits = core::ptr::read_volatile(reason);
            bits & ((1 << 0) | (1 << 1)) != 0
        }
    }

    /// Enables or disables the watchdog timer.
    ///
    /// # Arguments
    /// * `enable` - `true` to enable the watchdog, `false` to disable it.
    pub fn enable(&self, enable: bool) {
        unsafe {
            if enable {
                CTRL.write_volatile(CTRL.read_volatile() | 0x40000000);
            } else {
                CTRL.write_volatile(CTRL.read_volatile() & !0x40000000);
            }
        }
    }
}
