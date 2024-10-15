use crate::xs::Bits;

// PADS_BANK0_BASE
pub const BASE: u32 = 0x4001c000;

/// # Behavior of Pad Ctrl Bits (0..7)
///
/// - 0: SLEWFAST - Slew rate control. 1 = Fast, 0 = Slow
/// - 1: SCHMITT - Enable Schmitt trigger
/// - 2: PDE - Pull down enable
/// - 3: PUE - Pull up enable
/// - 4: DRIVE_LOW - Drive strength low bit (2mA when combined with DRIVE_HIGH = 0)
/// - 5: DRIVE_HIGH - Drive strength high bit (12mA when combined with DRIVE_LOW = 1)
/// - 6: IE - Input enable
/// - 7: OD - Output disable. Overrides output enable from peripherals

/// Enum to represent the pull-up, pull-down, or no-pull resistor
pub enum Pull {
    None = 0, // No pull resistor
    Down = 1, // Pull-down resistor
    Up = 2,   // Pull-up resistor
}

/// Clears both pull directions, then sets pull-up or pull-down for the given GPIO.
///
/// # Parameters
/// - `pin`: The GPIO pin number to configure.
/// - `dir`: The pull direction to set, either `Pull::Down`, `Pull::Up`, or `Pull::None`.
///
/// # Example:
/// ```rust
/// gpio_pull(5, Pull::Up); // Configures GPIO pin 5 with a pull-up resistor
/// gpio_pull(3, Pull::Down); // Configures GPIO pin 3 with a pull-down resistor
/// ```
///
pub fn gpio_pull(pin: u32, dir: Pull) {
    let pad_ctrl = (BASE + 0x4 + 0x4 * pin) as *mut u32;

    // Clear both PUE and PDE bits
    pad_ctrl.clear(1 << 3 | 1 << 2);

    match dir {
        Pull::Down => {
            // Set PDE (pull down enable)
            pad_ctrl.set(1 << 2);
        }
        Pull::Up => {
            // Set PUE bit (pull up enable)
            pad_ctrl.set(1 << 3);
        }
        Pull::None => {
            // Do nothing because pulls have already been cleared
        }
    }
}
