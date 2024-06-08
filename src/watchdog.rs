use crate::regs::{PSM_BASE, WATCHDOG_BASE};

const PSM_WDSEL: *mut u32 = (PSM_BASE + 0x8) as *mut u32;
const WATCHDOG_CTRL: *mut u32 = (WATCHDOG_BASE + 0x00) as *mut u32;
const WATCHDOG_CTRL_ENABLE_BITS: u32 = 0x40000000;
const WATCHDOG_LOAD: *mut u32 = (WATCHDOG_BASE + 0x04) as *mut u32;
const WATCHDOG_NON_REBOOT_MAGIC: u32 = 0x6ab73121;
const WATCHDOG_REBOOT_MAGIC: u32 = 0x6ab73120;
const WATCHDOG_SCRATCH_4: *mut u32 = (WATCHDOG_BASE + 0x1c) as *mut u32;
const WATCHDOG_TICK: *mut u32 = (WATCHDOG_BASE + 0x2c) as *mut u32;

static mut load_value: u32 = 0;

pub fn update() {
    unsafe {
        WATCHDOG_LOAD.write_volatile(load_value);
    }
}

pub fn start(cyles: u32) {
    const KHZ: u32 = 1000;
    const XOSC_KHZ: u32 = 12000;
    let cycles: u32 = XOSC_KHZ / KHZ;
    unsafe {
        WATCHDOG_TICK.write_volatile(cycles | WATCHDOG_CTRL_ENABLE_BITS);
    }
}

pub fn enable(delay_ms: u32, did_reboot: bool) {
    let mut flag = WATCHDOG_NON_REBOOT_MAGIC;
    if did_reboot {
        flag = WATCHDOG_REBOOT_MAGIC;
    } else {
        flag = WATCHDOG_NON_REBOOT_MAGIC;
    }
    unsafe {
        WATCHDOG_SCRATCH_4.write_volatile(flag);
    }
    _enable(delay_ms)
}
pub fn _enable(delay_ms: u32) {
    unsafe {
        let mut value = WATCHDOG_CTRL.read_volatile();
        value &= !WATCHDOG_CTRL_ENABLE_BITS;
        WATCHDOG_CTRL.write_volatile(value);

        const PSM_WDSEL_ROSC_BITS: u32 = 0x00000001;
        const PSM_WDSEL_XOSC_BITS: u32 = 0x00000002;

        // Reset everything apart from ROSC and XOSC
        let wdsel_val = core::ptr::read_volatile(PSM_WDSEL);
        core::ptr::write_volatile(
            PSM_WDSEL,
            wdsel_val & !(PSM_WDSEL_ROSC_BITS | PSM_WDSEL_XOSC_BITS),
        );
        const WATCHDOG_CTRL_ENABLE_BITS: u32 = 0x40000000;
        const WATCHDOG_CTRL_PAUSE_DBG1_BITS: u32 = 0x04000000;
        const WATCHDOG_CTRL_PAUSE_JTAG_BITS: u32 = 0x01000000;
        const WATCHDOG_CTRL_PAUSE_DBG0_BITS: u32 = 0x02000000;
        // Configure debug pause settings based on the `pause_on_debug` flag
        let dbg_bits = WATCHDOG_CTRL_PAUSE_DBG0_BITS
            | WATCHDOG_CTRL_PAUSE_DBG1_BITS
            | WATCHDOG_CTRL_PAUSE_JTAG_BITS;

        let pause_on_debug = false;
        value = WATCHDOG_CTRL.read_volatile();
        if pause_on_debug == true {
            core::ptr::write_volatile(WATCHDOG_CTRL, value | dbg_bits);
        } else {
            core::ptr::write_volatile(WATCHDOG_CTRL, value & !dbg_bits);
        }

        load_value = delay_ms * 1000 * 2;
        if load_value > 0xffffff {
            load_value = 0xffffff;
        }
        update();

        value = WATCHDOG_CTRL.read_volatile();
        WATCHDOG_CTRL.write_volatile(value | WATCHDOG_CTRL_ENABLE_BITS);
    }
}

pub fn did_reboot() -> bool {
    unsafe { WATCHDOG_SCRATCH_4.read_volatile() == WATCHDOG_NON_REBOOT_MAGIC }
}

pub fn trigger() {
    unsafe {
        WATCHDOG_TICK.write_volatile(0);
    }
}
