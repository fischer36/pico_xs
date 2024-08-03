use crate::{
    registers::resets::{RESET_BIT_PLL_SYS, RESET_BIT_PLL_USB},
    xs::Bits,
};

use super::resets::{self, reset_wait};

pub const BASE: u32 = 0x40008000;

pub const SYS_REUS_CTRL: *mut u32 = (BASE + 0x78) as *mut u32;
pub const SYS_REUS_STATUS: *mut u32 = (BASE + 0x7c) as *mut u32;

pub const PERI_CTRL: *mut u32 = (BASE + 0x48) as *mut u32;
pub const PERI_SELECTED: *mut u32 = (BASE + 0x50) as *mut u32;

pub const REF_CTRL: *mut u32 = (BASE + 0x30) as *mut u32;
pub const REF_DIV: *mut u32 = (BASE + 0x34) as *mut u32;
pub const REF_SELECTED: *mut u32 = (BASE + 0x38) as *mut u32;

pub const SYS_CTRL: *mut u32 = (BASE + 0x3c) as *mut u32;
pub const SYS_DIV: *mut u32 = (BASE + 0x40) as *mut u32;
pub const SYS_SELECTED: *mut u32 = (BASE + 0x44) as *mut u32;

pub const USB_CTRL: *mut u32 = (BASE + 0x54) as *mut u32;
pub const USB_DIV: *mut u32 = (BASE + 0x58) as *mut u32;
pub const USB_SELECTED: *mut u32 = (BASE + 0x5c) as *mut u32;

pub const ADC_CTRL: *mut u32 = (BASE + 0x60) as *mut u32;
pub const ADC_DIV: *mut u32 = (BASE + 0x64) as *mut u32;
pub const ADC_SELECTED: *mut u32 = (BASE + 0x68) as *mut u32;

pub const RTC_CTRL: *mut u32 = (BASE + 0x6c) as *mut u32;
pub const RTC_DIV: *mut u32 = (BASE + 0x70) as *mut u32;
pub const RTC_SELECTED: *mut u32 = (BASE + 0x74) as *mut u32;

pub fn ref_clock() {
    // Clear bit 1:0
    REF_CTRL.clear(1 << 0 | 1 << 1);
    // 0x0 → rosc_clksrc_ph
    // 0x1 → clksrc_clk_ref_aux
    // 0x2 → xosc_clksrc
    // Set bit 1 (XOSC_CLKSRC)
    REF_CTRL.clear(1 << 1);
}
pub fn init() {
    ref_clock();
    setup_ppls(false);
    setup_ppls(true);
}

// To configure a clock you need:
// - The frequency of the clock-source
// - The mux / aux mux position of the clock-source
// - The desired output frequency
//
// # Clock Sources:
// - clk_src_pll_sys
// - clk_src_gpin0
// - clk_src_gpin1
// - clk_src_pll_usb
// - rosc_clksrc
// - xosc_clksrc
// - clk_src_pll_sys
// - clk_src_pll_sys
// - etc.
//
/// # USB and SYS PLLS
// Are used to provide fast clocks from XOSC etc. by factorizing the source, XOSC
pub const PLL_SYS_BASE: *mut u32 = 0x40028000 as *mut u32;
pub const PLL_USB_BASE: *mut u32 = 0x4002c000 as *mut u32;
// Default PLL configuration:
// REF FBDIV VCO POSTDIV
// PLL SYS: 12 / 1 = 12MHz * 125 = 1500MHz / 6 / 2 = 125MHz
// PLL USB: 12 / 1 = 12MHz * 100 = 1200MHz / 5 / 5 = 48MHz
fn setup_ppls(pll_usb: bool) {
    unsafe {
        let pll_base: *mut u32;
        let ref_div = 1;
        let post_div1;
        let post_div2;
        let fbdiv;
        let vco_freq;
        let pdiv;
        let reset;

        const XOSC_KHZ: u32 = 12000;
        const REF_FREQ: u32 = XOSC_KHZ * 1000 / 1;
        const PLL_PRIM_POSTDIV1_LSB: u32 = 16;
        const PLL_PRIM_POSTDIV2_LSB: u32 = 12;

        if pll_usb {
            pll_base = PLL_USB_BASE;
            vco_freq = 1200 * 1000;
            post_div1 = 5;
            post_div2 = 5;
            fbdiv = vco_freq / REF_FREQ;
            pdiv = post_div1 << PLL_PRIM_POSTDIV1_LSB | post_div2 << PLL_PRIM_POSTDIV2_LSB;
            reset = RESET_BIT_PLL_USB;
        } else {
            pll_base = PLL_SYS_BASE;
            vco_freq = 1500 * 1000;
            post_div1 = 6;
            post_div2 = 2;
            fbdiv = vco_freq / REF_FREQ;
            pdiv = post_div1 << PLL_PRIM_POSTDIV1_LSB | post_div2 << PLL_PRIM_POSTDIV2_LSB;
            reset = RESET_BIT_PLL_SYS;
        }

        let pll_cs: *mut u32 = (pll_base as u32 + 0x0) as *mut u32; // Control and Status
        let pll_pwr: *mut u32 = (pll_base as u32 + 0x4) as *mut u32; // Controls the PLL power modes
        let pll_fbdiv_int: *mut u32 = (pll_base as u32 + 0x8) as *mut u32; // Feedback divisor
        let pll_prim: *mut u32 = (pll_base as u32 + 0xc) as *mut u32; // Controls the PLL post dividers for the primary output

        // Bring PLL_SYS and PLL_USB out of reset
        super::resets::reset_wait(reset);

        // pll->cs = refdiv;
        pll_cs.write_volatile(ref_div);

        // pll->fbdiv_int = fbdiv;
        pll_fbdiv_int.write_volatile(fbdiv);

        // Turn on PLL
        let power: u32 = 0x00000001| // Main power
                     0x00000020; // VCO Power
                                 // hw_clear_bits(&pll->pwr, power);
        pll_pwr.clear(power);

        // Wait for PLL to lock
        // while (!(pll->cs & PLL_CS_LOCK_BITS)) tight_loop_contents();
        loop {
            if pll_cs.read_volatile() & 0x80000000 != 0 {
                break;
            }
        }

        // Set up post dividers
        pll_prim.write_volatile(pdiv);

        // Turn on post divider
        pll_pwr.clear(0x00000008);
    }
}
