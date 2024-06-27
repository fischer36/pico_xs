#![no_std]
#![no_main]

use regs::RESETS_RESET_CLR;
const WATCHDOG_BASE: u32 = 0x40058000;
const WATCHDOG_TICK: *mut u32 = (WATCHDOG_BASE + 0x2c) as *mut u32;
const XOSC_BASE: u32 = 0x40024000 as u32;
const XOCV_CTRL: *mut u32 = (XOSC_BASE + 0x0) as *mut u32;
const XOCV_STATUS: *mut u32 = (XOSC_BASE + 0x04) as *mut u32;
const XOCV_DOORMANT: *mut u32 = (XOSC_BASE + 0x08) as *mut u32;
const XOCV_STARTUP: *mut u32 = (XOSC_BASE + 0x0c) as *mut u32;
const XOCV_COUNT: *mut u32 = (XOSC_BASE + 0x1c) as *mut u32;
const CLOCKS_BASE: u32 = 0x40008000;
const CLK_REF_CTRL: *mut u32 = (CLOCKS_BASE + 0x30) as *mut u32;
const CLK_REF_DIV: *mut u32 = (CLOCKS_BASE + 0x34) as *mut u32;
const CLK_REF_SELECTED: *mut u32 = (CLOCKS_BASE + 0x38) as *mut u32;
const CLK_SYS_CTRL: *mut u32 = (CLOCKS_BASE + 0x3c) as *mut u32;
const CLK_SYS_DIV: *mut u32 = (CLOCKS_BASE + 0x40) as *mut u32;
const CLK_SYS_SELECTED: *mut u32 = (CLOCKS_BASE + 0x44) as *mut u32;
const CLK_PERI_CTRL: *mut u32 = (CLOCKS_BASE + 0x48) as *mut u32;
const CLK_PERI_SELECTED: *mut u32 = (CLOCKS_BASE + 0x50) as *mut u32;
const CLK_USB_CTRL: *mut u32 = (CLOCKS_BASE + 0x54) as *mut u32;
const CLK_USB_DIV: *mut u32 = (CLOCKS_BASE + 0x58) as *mut u32;
const CLK_USB_SELECTED: *mut u32 = (CLOCKS_BASE + 0x5c) as *mut u32;
const CLK_ADC_CTRL: *mut u32 = (CLOCKS_BASE + 0x60) as *mut u32;
const CLK_ADC_DIV: *mut u32 = (CLOCKS_BASE + 0x64) as *mut u32;
const CLK_ADC_SELECTED: *mut u32 = (CLOCKS_BASE + 0x68) as *mut u32;
const CLK_RTC_CTRL: *mut u32 = (CLOCKS_BASE + 0x6c) as *mut u32;
const CLK_RTC_DIV: *mut u32 = (CLOCKS_BASE + 0x70) as *mut u32;
const CLK_RTC_SELECTED: *mut u32 = (CLOCKS_BASE + 0x74) as *mut u32;
const CLK_SYS_RESUS_CTRL: *mut u32 = (CLOCKS_BASE + 0x78) as *mut u32;
const MHZ: u32 = 1_000_000;
//pub mod __vectors;
pub mod final_utils;
pub mod handlers;
pub mod hardware;
pub mod interrupts;
pub mod io;
pub mod pads;
pub mod registers;
pub mod regs;
pub mod reset;
pub mod sio;
pub mod timer;
pub mod usb;
use rp_pico::entry;
use usb::usb_device_init;

#[entry]
fn main() -> ! {
    // Setup Chip
    {
        let mask = (1 << 6) | (1 << 9) | (1 << 13) | (1 << 12);
        unsafe {
            clr_bits(regs::RESETS_RESET_CLR as *mut u32, mask);
            sleep();
            let new_mask =
                (1 << 24) | (1 << 23) | (1 << 22) | (1 << 17) | (1 << 16) | (1 << 15) | (1 << 0);
            core::ptr::write_volatile(regs::RESETS_BASE as *mut u32, new_mask);
        }
    }

    sleep();
    sleep();

    unsafe {
        clocks_init();
    }
    unsafe {
        core::ptr::write_volatile(regs::RESETS_BASE as *mut u32, 0);
    }
    sleep();

    pub const SIO_BASE: u32 = 0xd000_0000;
    pub const SIO_GPIO_OUT_RW: u32 = SIO_BASE + 0x10;
    pub const SIO_GPIO_OUT_SET: u32 = SIO_BASE + 0x14;
    pub const SIO_GPIO_OUT_CLR: u32 = SIO_BASE + 0x18;
    pub const SIO_GPIO_OUT_XOR: u32 = SIO_BASE + 0x1C;
    pub const SIO_GPIO_OE_SET: u32 = SIO_BASE + 0x24;
    pub const GPIO25_CTR: *mut u32 = (0x40014000 + 0x0cc) as *mut u32;
    let sio_gpio_oe_clr: u32 = SIO_BASE + 0x28;
    set_bits(sio_gpio_oe_clr as *mut u32, 1 << 25);

    let sio_gpio_out_clr: u32 = SIO_BASE + 0x18;
    set_bits(sio_gpio_out_clr as *mut u32, 1 << 25);

    let pads_bank0_base: *mut u32 = (0x4001c000 + 0x68) as *mut u32;
    clr_bits(pads_bank0_base, 1 << 7);
    set_bits(pads_bank0_base, 1 << 6);
    unsafe {
        modify_register(GPIO25_CTR, create_bitmask(0, 4), 5);
    }
    set_bits(SIO_GPIO_OE_SET as *mut u32, 1 << 25);

    unsafe {
        let mut usb_device = usb::usb_device_init();

        while !usb_device.configured() {
            usb_device.poll();
        }
        pub const SIO_BASE: u32 = 0xd000_0000;
        pub const SIO_GPIO_OUT_RW: u32 = SIO_BASE + 0x10;
        pub const SIO_GPIO_OUT_SET: u32 = SIO_BASE + 0x14;
        pub const SIO_GPIO_OUT_CLR: u32 = SIO_BASE + 0x18;
        pub const SIO_GPIO_OUT_XOR: u32 = SIO_BASE + 0x1C;
        pub const SIO_GPIO_OE_SET: u32 = SIO_BASE + 0x24;
        pub const GPIO25_CTR: *mut u32 = (0x40014000 + 0x0cc) as *mut u32;
        set_bits(SIO_GPIO_OUT_SET as *mut u32, 1 << 25);
        loop {
            sleep();
            usb_device.poll();
        }
    }
}

pub fn sleep() {
    unsafe {
        for _ in 0..50_000 {
            core::arch::asm!("nop");
        }
    }
}
fn create_bitmask(start_bit: u32, end_bit: u32) -> u32 {
    if start_bit > end_bit || end_bit >= 32 {
        panic!("Invalid bit range or bit range out of bounds");
    }
    let length = end_bit - start_bit + 1;
    let mask = (1 << length) - 1; // Creates a mask of `length` bits long (e.g., for 3 bits: `0b111`).
    mask << start_bit // Shifts the mask to start from `start_bit`.
}
unsafe fn modify_register(reg: *mut u32, mask: u32, value: u32) {
    let current_value = reg.read_volatile(); // Read the current value of the register
    let cleared_value = current_value & !mask; // Clear the bits specified by the mask
    let new_value = cleared_value | (value & mask); // Set new value to the specified bits
    reg.write_volatile(new_value); // Write the new value back to the register
}

fn clr_bits(reg: *mut u32, bit_mask: u32) {
    unsafe {
        let old = core::ptr::read_volatile(reg); // Read current value from the register
        core::ptr::write_volatile(reg, old & !bit_mask); // Clear specified bits and write back
    }
}
fn set_bits(reg: *mut u32, bit_mask: u32) {
    unsafe {
        let old = core::ptr::read_volatile(reg); // Read current value from the register
        core::ptr::write_volatile(reg, old | bit_mask); // Set specified bits and write back
    }
}

fn pll_init(is_usb: bool) {
    const PLL_SYS_BASE: u32 = 0x40028000;
    const PLL_USB_BASE: u32 = 0x4002c000;
    let ref_div = if is_usb { 1 } else { 1 };
    let post_div1 = if is_usb { 5 } else { 6 };
    let post_div2 = if is_usb { 2 } else { 2 };
    let pll_base = if is_usb { PLL_USB_BASE } else { PLL_SYS_BASE };
    let ppl_cs: *mut u32 = pll_base as *mut u32;
    let ppl_pwr: *mut u32 = (pll_base + 0x4) as *mut u32;
    let ppl_fdiv: *mut u32 = (pll_base + 0x8) as *mut u32;
    let ppl_prim: *mut u32 = (pll_base + 0xc) as *mut u32;

    let ref_mhz = 12;
    let vco_freq = if is_usb { 480 * MHZ } else { 1500 * MHZ };
    let fbdiv = vco_freq / (ref_mhz * 1_000_000); // 40 for usb and 125 for sys

    unsafe {
        core::ptr::write_volatile(ppl_pwr, 0);
        core::ptr::write_volatile(ppl_fdiv, 2048);

        modify_register(ppl_cs, 63, ref_div);

        clr_bits(ppl_fdiv, 2048);
        core::ptr::write_volatile(ppl_fdiv, fbdiv);

        clr_bits(ppl_pwr, 1 << 5 | 1 << 0);
        loop {
            if (ppl_cs.read_volatile() & (1 << 31)) != 0 {
                break;
            }
        }
        let mask = 1 << 12 | 1 << 13 | 1 << 14 | 1 << 16 | 1 << 17 | 1 << 18;
        clr_bits(ppl_prim, mask);
        let old = core::ptr::read_volatile(ppl_prim);
        core::ptr::write_volatile(ppl_prim, old | (post_div1 << 16) | (post_div2 << 12));
        clr_bits(ppl_pwr, 1 << 3);
    }
}
unsafe fn clocks_init() {
    // 1. Setup Watchdog
    // 2. Setup XOSV
    {
        modify_register(WATCHDOG_TICK, 511, 1 << 2 | 1 << 3 | 1 << 9);
        clr_bits(CLK_SYS_RESUS_CTRL, 1 << 8);
        set_bits(XOCV_STATUS, 1 << 24);
        set_bits(XOCV_CTRL, 0xFAB000 | 0xaa0);
        set_bits(XOCV_STARTUP, 47);

        loop {
            if XOCV_STATUS.read_volatile() & (1 << 31) != 0 {
                break;
            }
        }
    }
    // 1. Setup USB_PLL AND SYS_PLL
    // 2. Setup Clocks
    {
        sleep();
        clr_bits(CLK_SYS_CTRL, 1 << 0);
        sleep();
        clr_bits(CLK_REF_CTRL, 1 << 0);
        sleep();
        set_bits(RESETS_RESET_CLR as *mut u32, 1 << 13 | 1 << 12);
        sleep();
        clr_bits(RESETS_RESET_CLR as *mut u32, 1 << 13 | 1 << 12);
        sleep();

        pll_init(true);
        pll_init(false);

        let div = 256;
        if (CLK_REF_DIV.read_volatile() & create_bitmask(8, 9)) < div {
            modify_register(CLK_REF_DIV, create_bitmask(8, 9), div);
        }

        modify_register(CLK_REF_CTRL, create_bitmask(0, 1), 2);

        loop {
            if CLK_REF_SELECTED.read_volatile() & (1 << 2) != (1 << 2) {
                continue;
            } else {
                break;
            }
        }

        modify_register(CLK_REF_DIV, create_bitmask(8, 9), div);

        let div = 256;
        if CLK_SYS_DIV.read_volatile() <= div {
            modify_register(CLK_SYS_DIV, create_bitmask(0, 7), div);
        }
        modify_register(CLK_SYS_CTRL, create_bitmask(5, 7), 0);
        modify_register(CLK_SYS_CTRL, 1 << 0, 1);

        loop {
            if (CLK_SYS_SELECTED.read_volatile() & (1 << 1)) != (1 << 1) {
            } else {
                break;
            }
        }
        modify_register(CLK_SYS_DIV, create_bitmask(0, 7), div);

        let div = 256;
        if (CLK_USB_DIV.read_volatile() & create_bitmask(8, 9)) < div {
            let mask = create_bitmask(0, 7);
            modify_register(CLK_USB_DIV, mask, div);
        }

        clr_bits(CLK_USB_CTRL, 1 << 11);
        nop();
        nop();
        modify_register(CLK_USB_CTRL, create_bitmask(5, 7), 0);
        set_bits(CLK_USB_CTRL, 1 << 11);
        modify_register(CLK_USB_DIV, create_bitmask(0, 7), div);

        let div = 256;
        if (CLK_ADC_DIV.read_volatile() & create_bitmask(8, 9)) < div {
            modify_register(CLK_ADC_DIV, create_bitmask(0, 7), div);
        }
        clr_bits(CLK_ADC_CTRL, 1 << 11);
        nop();
        nop();
        modify_register(CLK_ADC_CTRL, create_bitmask(5, 7), 0);
        set_bits(CLK_ADC_CTRL, 1 << 11);
        modify_register(CLK_ADC_DIV, create_bitmask(0, 7), div);

        let div = 262144;

        if (CLK_RTC_DIV.read_volatile() & create_bitmask(0, 7)) < div {
            modify_register(CLK_RTC_DIV, create_bitmask(0, 7), div);
        }
        clr_bits(CLK_RTC_CTRL, 1 << 11);
        nop();
        nop();
        modify_register(CLK_RTC_CTRL, create_bitmask(5, 7), 0);
        set_bits(CLK_RTC_CTRL, 1 << 11);
        modify_register(CLK_RTC_DIV, create_bitmask(0, 7), div);

        clr_bits(CLK_PERI_CTRL, 1 << 11);
        nop();
        nop();
        nop();
        modify_register(CLK_PERI_CTRL, create_bitmask(5, 7), 0);
        set_bits(CLK_PERI_CTRL, 1 << 11);

        nop();
        nop();
        nop();
    }
}
pub fn nop() {
    unsafe {
        core::arch::asm!("nop");
    }
}
pub fn setup_chip() {
    let mask = (1 << 6) | (1 << 9) | (1 << 13) | (1 << 12);
    unsafe {
        clr_bits(RESETS_RESET_CLR as *mut u32, mask);
        sleep();
        let new_mask =
            (1 << 24) | (1 << 23) | (1 << 22) | (1 << 17) | (1 << 16) | (1 << 15) | (1 << 0);
        core::ptr::write_volatile(regs::RESETS_BASE as *mut u32, new_mask);
    }

    sleep();
    sleep();
}
