use crate::xs::Bits;
// PADS_BANK0_BASE
//
pub const BASE: u32 = 0x4001c000;

// Temporary prototype functions for gpio pad ctrl
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
pub fn input_enable(gpio: u32) {
    let pad_ctrl = (BASE + 0x4 + 0x4 * gpio) as *mut u32;
    // clear PDE bit
    pad_ctrl.clear(1 << 2);

    // set PUE bit
    pad_ctrl.set(1 << 3);
}
