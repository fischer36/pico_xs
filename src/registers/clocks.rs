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
// # PLLS:
// Are used to provide fast clocks from XOSC etc. by factorizing the source, XOSC
//
//
//  e e
