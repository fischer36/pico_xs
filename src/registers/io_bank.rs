use crate::xs::Bits;

/// Base address for IO_BANK0.
///
/// This base address is used to access the control and status registers of individual GPIOs
/// within IO_BANK0. Each GPIO has a pair of 32-bit registers: STATUS and CTRL.
///
/// # Registers Layout
///
/// - Each GPIO from 0 to 29 has two associated registers:
///   - `STATUS`: Used to read the current status of the GPIO.
///   - `CTRL`: Used to control various functionalities of the GPIO.
///
/// # Offsets
///
/// - `STATUS` for GPIO `n`: `0x0 + 0x8 * n`
///   - Calculated by starting at the base address and moving `n` positions each 8 bytes apart.
///
/// - `CTRL` for GPIO `n`: `0x4 + 0x8 * n`
///   - Positioned 4 bytes after the `STATUS` register of the same GPIO, reflecting the control settings.
///
/// ## Example Usage
///
/// To access the `STATUS` register for GPIO 5:
/// ```rust
/// let gpio_status_addr = BASE + 0x0 + 0x8 * 5;
/// ```
///
/// To access the `CTRL` register for GPIO 5:
/// ```rust
/// let gpio_ctrl_addr = BASE + 0x4 + 0x8 * 5;
/// ```
///
/// # Constants
///
/// - `BASE`: The base memory address for IO_BANK0, which is `0x40014000`.
///
pub const BASE: u32 = 0x40014000;
