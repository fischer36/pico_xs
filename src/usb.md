use crate::regs;
use crate::reset;

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
    // 1. Setup chip
    {
        let mask = (1 << 6) | (1 << 9) | (1 << 13) | (1 << 12);
        unsafe {
            clr_bits(RESETS_RESET_CLR as *mut u32, mask);
            sleep();
            let new_mask =
                (1 << 24) | (1 << 23) | (1 << 22) | (1 << 17) | (1 << 16) | (1 << 15) | (1 << 0);
            core::ptr::write_volatile(RESETS_BASE as *mut u32, new_mask);
        }
    }

    sleep();
    sleep();
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

        pll_initxd(true);
        pll_initxd(false);

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

fn setup_chip() {
    // 9 pads_qsip
    // 6 io qspi
    // 13 pll sys
    // 12 pll usb
    let mask = (1 << 6) | (1 << 9) | (1 << 13) | (1 << 12);

    reset::reset_mask(!mask);

    let new_mask = (1 << 24) | (1 << 23) | (1 << 22) | (1 << 17) | (1 << 16) | (1 << 15) | (1 << 0);

    unsafe {
        core::ptr::write_volatile(regs::RESETS_BASE as *mut u32, new_mask);
    }

    crate::sleep();
    crate::sleep();
}

pub fn setup() {
    setup_chip();
    setup_clocks();
    unsafe {
        core::ptr::write_volatile(EP0_IN_BUFFERCTRL, 0);
        let mask = 1 << 6; // 64 bits in size

        core::ptr::write_volatile(EP0_OUT_BUFFERCTRL, 0);
    }
}

const EP0_IN_ADDRESS: *mut u32 = (regs::USBCTRL_BASE + 0x80) as *mut u32;
const EP0_OUT_ADDRESS: *mut u32 = (regs::USBCTRL_BASE + 0x0) as *mut u32;

const EP0_IN_BUFFERCTRL: *mut u32 = (regs::USBCTRL_BASE + 0x80) as *mut u32;
const EP0_OUT_BUFFERCTRL: *mut u32 = (regs::USBCTRL_BASE + 0x84) as *mut u32;
const EP0_BUFFER: *mut u8 = (regs::USBCTRL_BASE + 0x100) as *mut u8; // goes on for 0x40 bytes

pub fn control_registers() {}
/*
 * EP_0_OUT:
 * */
