/// Disabled by default
use crate::xs::Bits;
//
//pub struct Tick(u32);
//
//impl Tick {
//    pub fn enable(&mut self) {
//        let enable_bit: u32 = 1 << 9;
//        self.0 |= enable_bit; // Use bitwise OR to set the bit
//    }
//
//    pub fn disable(&mut self) {
//        let enable_bit: u32 = 1 << 9;
//        self.0 &= !enable_bit; // Use bitwise AND with NOT to clear the bit
//    }
//
//    pub fn set_cycles(&mut self, value: u32) {
//        const TICK_CYCLES_MASK: u32 = 0x1FF; // 511 is 0b111111111 in binary
//        self.0 = (self.0 & !TICK_CYCLES_MASK) | (value & TICK_CYCLES_MASK); // Clear the mask bits and set to `value`
//    }
//}
//
//#[repr(C)]
//pub struct Watchdog {
//    ctrl: u32, // Assuming lower case for fields
//    load: u32,
//    reason: u32,
//    scratch_0: u32,
//    scratch_1: u32,
//    scratch_2: u32,
//    scratch_3: u32,
//    scratch_4: u32,
//    scratch_5: u32,
//    scratch_6: u32,
//    scratch_7: u32,
//    pub tick: Tick,
//}
//
//impl Watchdog {
//    pub fn new() -> &'static mut Self {
//        unsafe { &mut *(BASE as *mut Watchdog) }
//    }
//}
//
// BASE
const BASE: u32 = 0x40058000;
// Watchdog Control Register
const CTRL: *mut u32 = BASE as *mut u32;
// Watchdog Load Register
const LOAD: *mut u32 = (BASE + 0x04) as *mut u32;
// Watchdog Reason Register
const REASON: *mut u32 = (BASE + 0x08) as *mut u32;
// Watchdog Scratch Registers
const SCRATCH0: *mut u32 = (BASE + 0x0c) as *mut u32;
const SCRATCH1: *mut u32 = (BASE + 0x10) as *mut u32;
const SCRATCH2: *mut u32 = (BASE + 0x14) as *mut u32;
const SCRATCH3: *mut u32 = (BASE + 0x18) as *mut u32;
const SCRATCH4: *mut u32 = (BASE + 0x1c) as *mut u32;
const SCRATCH5: *mut u32 = (BASE + 0x20) as *mut u32;
const SCRATCH6: *mut u32 = (BASE + 0x24) as *mut u32;
const SCRATCH7: *mut u32 = (BASE + 0x28) as *mut u32;
// Watchdog Tick Generator Control Register
const TICK: *mut u32 = (BASE + 0x2c) as *mut u32;

pub struct Watchdog {
    countdown: u32,
}

// impl Watchdog {
//     // Initialize Watchdog
//     pub fn new(countdown_ms: u32) -> Self {
//         // Temporarily disable self for setup-process
//         CTRL.clear(1 << 30);
//
//         // Configure what peripherals (bits 16:0) to reset on watchdog-fire.
//         // The following sets all peripherals to be reset, except XOSC (1) and ROSC (0):
//         const WDSEL: *mut u32 = (super::PSM_BASE + 0x8) as *mut u32;
//         const WDSEL_MASK: u32 = 0x1FFFF;
//         WDSEL.modify(WDSEL_MASK, !WDSEL_MASK | 1 << 0 | 1 << 1);
//
//         // Disable pause on debug for proccessor-0(25), proccessor-1(26), and JTAG(24) bus fabric access (?)
//         CTRL.clear(1 << 24 | 1 << 25 | 1 << 26);
//
//         // Store countdown in seconds and multiply by 2 because tick decrements twice
//         let countdown_seconds = countdown_ms * 1000 * 2;
//         Self {
//             countdown: countdown_seconds,
//         }
//     }
// Enable Watchdog
//     pub fn enable(&self) {0x6ab73121
//         // Load the countdown
//         self.kick();
//         // Enable Watchdog
//         CTRL.set(1 << 30);
//         // Start the countdown, with XOSC frequency MHz
//         self.start(12000 / 1000);
//     }
//     // Disable Watchdog
//     pub fn disable(&self) {
//         // Simpely clear the enable bit
//         CTRL.clear(1 << 30);
//         // Disable tick aswell
//         TICK.clear(1 << 9);
//     }ets the job done much more easily than my attempts at doing this in the config.toml file. It runs the command:
//     // Resets countdown tick, preventing resets.
//     pub fn kick(&self) {
//         const LOAD_MASK: u32 = 0xffffff; // Bits 23:0
//                                          //
//         unsafe {
//             LOAD.write_volatile(self.countdown);
//         }
//     }
//     // Start the countdown; should be called from clocks-module allowing customized tick references
//     fn start(&self, cycles: u32) {
//         // Set cycle-speed (8:0) and then start tick countdown (9)
//         const TICK_CYCLES_MASK: u32 = 0x1FF; // Bits 8:0
//         TICK.modify(TICK_CYCLES_MASK, cycles | 1 << 9);
//     }
// }
//
const PSM_WDSEL: *mut u32 = (super::PSM_BASE + 0x8) as *mut u32;
const WATCHDOG_CTRL: *mut u32 = (BASE + 0x00) as *mut u32;
const WATCHDOG_CTRL_ENABLE_BITS: u32 = 0x40000000;
const WATCHDOG_LOAD: *mut u32 = (BASE + 0x04) as *mut u32;
const WATCHDOG_REASON: *mut u32 = (BASE + 0x08) as *mut u32;
const WATCHDOG_NON_REBOOT_MAGIC: u32 = 0x6ab73121;
const WATCHDOG_REBOOT_MAGIC: u32 = 0x6ab73120;
const WATCHDOG_SCRATCH_4: *mut u32 = (BASE + 0x1c) as *mut u32;
const WATCHDOG_TICK: *mut u32 = (BASE + 0x2c) as *mut u32;

static mut load_value: u32 = 200000;

// Cycles is 12 for xosc
pub fn start(cycles: u32) {
    unsafe {
        WATCHDOG_TICK.set(17 | 1 << 9);
    }
}
pub fn enable(delay_ms: u32) {
    unsafe {
        //
        WATCHDOG_SCRATCH_4.write_volatile(WATCHDOG_NON_REBOOT_MAGIC);
        // Disable Watchdog
        WATCHDOG_CTRL.clear(1 << 30);
        // RESET EVERYTHING EXCEPT ROSC AND XOSC
        let wdsel_bits = 0b_11111111111111100;
        const PSM_WDSEL: *mut u32 = (super::PSM_BASE + 0x8) as *mut u32;
        PSM_WDSEL.set(wdsel_bits & !(1 << 0 | 1 << 1));

        // CLEAR DEBUG BITS
        CTRL.clear(1 << 24 | 1 << 25 | 1 << 26);

        // Load value
        kick();

        let time_value = (0b_111111111111) & 0x00FFFFFF; // TIME is 24 bits wid
                                                         // load_value = delay_ms * 1000 * 2;

        // Enable
        CTRL.set(time_value | 1 << 30);
    }
}
pub fn kick() {
    unsafe {
        WATCHDOG_LOAD.set(load_value);
    }
}
pub fn did_reboot() -> bool {
    unsafe {
        if WATCHDOG_SCRATCH_4.read_volatile() == WATCHDOG_NON_REBOOT_MAGIC {
            true
        } else {
            false
        }
    }
}

pub fn trigger() {
    unsafe {
        WATCHDOG_TICK.write_volatile(0);
    }
}

pub fn xd_pause() {
    unsafe {
        // Enable 30
        let ctrl: *mut u32 = (0x40058000 + 0x00) as *mut u32; // 0x2C
        let old: u32 = ctrl.read_volatile();

        ctrl.write_volatile(old & !(1 << 26 | 1 << 25 | 1 << 24) as u32);
    }
}
pub fn xd_load_counter(counter: u32) {
    unsafe {
        // COunter = delay_ms * 1000 * 2 -> in micro seconds
        // Reserved 31:24
        // Load 23:0
        let load: *mut u32 = (0x40058000 + 0x04) as *mut u32; // 0x2C
        let old: u32 = load.read_volatile();

        //load.write_volatile(old | counter as u32);

        let mask: u32 = 0b_1111_1111_1111_1111_1111_1111;
        load.write_volatile(old | 2000);
    }
}
pub fn xd_resets() {
    unsafe {
        // Enable 30
        let wdsel: *mut u32 = (0x4000c000 + 0x4) as *mut u32; // 0x2C
        let old: u32 = wdsel.read_volatile();
        let bits: u32 = 131071; // 16:0
        let mask = old | bits;
        wdsel.write_volatile(mask & !(1 << 1 | 1 << 0) as u32);
    }
}
pub fn xd_enable() {
    unsafe {
        // Enable 30
        let ctrl: *mut u32 = (0x40058000 + 0x00) as *mut u32; // 0x2C
        let old: u32 = ctrl.read_volatile();

        ctrl.write_volatile(old | (1 << 30) | 0xfff as u32);
    }
}
pub fn xd_disable() {
    unsafe {
        // Enable 30
        let ctrl: *mut u32 = (0x40058000 + 0x00) as *mut u32; // 0x2C
        let old: u32 = ctrl.read_volatile();

        ctrl.write_volatile(old & !(1 << 30) as u32);
    }
}
pub fn xd_tick(cycles: u8) {
    // Cyckes = 12_000_000 / 1_000_000 as u8
    //
    const WATCHDOG_TICK_ENABLE_BITS: u32 = 0x200;

    // Reserved 31:20
    // Count 19:11
    // Running 10
    // Enable 9
    // Cycles 8:0
    unsafe {
        const WATCHDOG_TICK_ENABLE_BITS: u32 = 1 << 9;
        let tick: *mut u32 = (0x40058000 + 0x2c) as *mut u32; // 0x2C
        let old: u32 = tick.read_volatile();

        tick.write_volatile(old | WATCHDOG_TICK_ENABLE_BITS | 12);
    }
    //self.watchdog
    //    .tick()
    //    .write(|w| unsafe { w.bits(WATCHDOG_TICK_ENABLE_BITS | cycles as u32) })
}
pub fn xs_is_running() -> bool {
    unsafe {
        let tick: *mut u32 = (0x40058000 + 0x2c) as *mut u32; // 0x2C
        if tick.read_volatile() & (1 << 10) != 0 {
            return true;
        } else {
            return false;
        }
    }
}
pub fn xs_did_reboot() -> bool {
    unsafe {
        if REASON.read_volatile() & (1 << 0 | 1 << 1) != 0
            && WATCHDOG_SCRATCH_4.read_volatile() == WATCHDOG_NON_REBOOT_MAGIC
        {
            return true;
        } else {
            return false;
        }
    }
}
pub fn xd_init() {
    //xd_disable();
    xd_pause();
    xd_resets();
    xd_load_counter(2000);
    xd_enable();
}
