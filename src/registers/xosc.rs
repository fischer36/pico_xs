/// XOSC - Crystal Oscillator Control for RP2040.
///
/// The XOSC is disabled by default and needs to be configured before use.
/// This module provides functionality to configure, enable, and check the status of the XOSC.
///
/// The XOSC configuration involves setting the frequency range, startup delay, and enabling the oscillator.
/// It also provides a method to check if the XOSC is enabled.
use crate::xs::Bits;

const BASE: u32 = 0x40024000;
const CTRL: *mut u32 = (BASE + 0x00) as *mut u32;
const STATUS: *mut u32 = (BASE + 0x04) as *mut u32;
const DOORMANT: *mut u32 = (BASE + 0x08) as *mut u32;
const STARTUP: *mut u32 = (BASE + 0x0c) as *mut u32;
const COUNT: *mut u32 = (BASE + 0x1c) as *mut u32;

/// Initializes the XOSC.
///
/// This function sets the frequency range, startup delay, and enables the XOSC. It will wait until the XOSC
/// becomes stable before returning.
///
/// # Example
/// ```
/// registers::xosc::init();
/// ```
pub fn init() {
    // Configures XOSC with default values.

    // Set Freq Range
    CTRL.modify(0xFFF, 0xaa0);

    // Set Start Up Delay
    STARTUP.set(0xc4);

    // Enable Xosc
    const XOSC_CTRL_DISABLE_BITS: u32 = 0xd1e;
    const XOSC_CTRL_ENABLE_BITS: u32 = 0xfab;
    CTRL.set(XOSC_CTRL_ENABLE_BITS << 12);

    const XOSC_STATUS_STABLE_BITS: u32 = 0x80000000;

    // Wait for XOSC to stabilize
    unsafe {
        loop {
            if STATUS.read_volatile() & (XOSC_STATUS_STABLE_BITS) != 0 {
                break;
            }
        }
    }
}

/// Checks if the XOSC is enabled.
///
/// This function returns `true` if the XOSC is currently enabled, `false` otherwise.
///
/// # Example
/// ```
/// if registers::xosc::is_enabled() {
///     // XOSC is enabled
/// }
/// ```
pub fn is_enabled() -> bool {
    unsafe {
        const XOSC_CTRL_ENABLE_BITS: u32 = 0xfab;
        if (STATUS.read_volatile() & (1 << 12)) != 0 {
            true
        } else {
            false
        }
    }
}
