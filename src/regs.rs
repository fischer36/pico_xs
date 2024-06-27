pub const SIO_BASE: u32 = 0xd000_0000;
pub const SIO_GPIO_OUT_RW: u32 = SIO_BASE + 0x10;
pub const SIO_GPIO_OUT_SET: u32 = SIO_BASE + 0x14;
pub const SIO_GPIO_OUT_CLR: u32 = SIO_BASE + 0x18;
pub const SIO_GPIO_OUT_XOR: u32 = SIO_BASE + 0x1C;
pub const PPB_BASE: u32 = 0xe0000000;

const CLOCKS_BASE: u32 = 0x40008000;
const CLK_GPOUT0_CTRL: *mut u32 = (CLOCKS_BASE + 0x00) as *mut u32;
const CLK_GPOUT0_DIV: *mut u32 = (CLOCKS_BASE + 0x04) as *mut u32;
const CLK_GPOUT0_SELECTED: *mut u32 = (CLOCKS_BASE + 0x08) as *mut u32;
const CLK_GPOUT1_CTRL: *mut u32 = (CLOCKS_BASE + 0x0c) as *mut u32;
const CLK_GPOUT1_DIV: *mut u32 = (CLOCKS_BASE + 0x10) as *mut u32;
const CLK_GPOUT1_SELECTED: *mut u32 = (CLOCKS_BASE + 0x14) as *mut u32;
const CLK_GPOUT2_CTRL: *mut u32 = (CLOCKS_BASE + 0x18) as *mut u32;
const CLK_GPOUT2_DIV: *mut u32 = (CLOCKS_BASE + 0x1c) as *mut u32;
const CLK_GPOUT2_SELECTED: *mut u32 = (CLOCKS_BASE + 0x20) as *mut u32;
const CLK_GPOUT3_CTRL: *mut u32 = (CLOCKS_BASE + 0x24) as *mut u32;
const CLK_GPOUT3_DIV: *mut u32 = (CLOCKS_BASE + 0x28) as *mut u32;
const CLK_GPOUT3_SELECTED: *mut u32 = (CLOCKS_BASE + 0x2c) as *mut u32;
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
const CLK_SYS_RESUS_STATUS: *mut u32 = (CLOCKS_BASE + 0x7c) as *mut u32;
const FC0_REF_KHZ: *mut u32 = (CLOCKS_BASE + 0x80) as *mut u32;
const FC0_MIN_KHZ: *mut u32 = (CLOCKS_BASE + 0x84) as *mut u32;
const FC0_MAX_KHZ: *mut u32 = (CLOCKS_BASE + 0x88) as *mut u32;
const FC0_DELAY: *mut u32 = (CLOCKS_BASE + 0x8c) as *mut u32;
const FC0_INTERVAL: *mut u32 = (CLOCKS_BASE + 0x90) as *mut u32;
const FC0_SRC: *mut u32 = (CLOCKS_BASE + 0x94) as *mut u32;
const FC0_STATUS: *mut u32 = (CLOCKS_BASE + 0x98) as *mut u32;
const FC0_RESULT: *mut u32 = (CLOCKS_BASE + 0x9c) as *mut u32;
const WAKE_EN0: *mut u32 = (CLOCKS_BASE + 0xa0) as *mut u32;
const WAKE_EN1: *mut u32 = (CLOCKS_BASE + 0xa4) as *mut u32;
const SLEEP_EN0: *mut u32 = (CLOCKS_BASE + 0xa8) as *mut u32;
const SLEEP_EN1: *mut u32 = (CLOCKS_BASE + 0xac) as *mut u32;
const ENABLED0: *mut u32 = (CLOCKS_BASE + 0xb0) as *mut u32;
const ENABLED1: *mut u32 = (CLOCKS_BASE + 0xb4) as *mut u32;
const INTR: *mut u32 = (CLOCKS_BASE + 0xb8) as *mut u32;
const INTE: *mut u32 = (CLOCKS_BASE + 0xbc) as *mut u32;
const INTF: *mut u32 = (CLOCKS_BASE + 0xc0) as *mut u32;
const INTS: *mut u32 = (CLOCKS_BASE + 0xc4) as *mut u32;

pub const RESETS_BASE: u32 = 0x4000C000;
pub const RESETS_RESET_DONE: *mut u32 = (0x4000C000 + 0x8) as *mut u32;
pub const RESETS_RESET_CLR: u32 = RESETS_BASE + 0x0 + 0x3000;

pub const TIMER_BASE: u32 = 0x40054000;

pub const RESETS_RESET_RW: u32 = RESETS_BASE + 0x0 + 0x0000;
pub const RESETS_RESET_XOR: u32 = RESETS_BASE + 0x0 + 0x1000;
pub const RESETS_RESET_SET: u32 = RESETS_BASE + 0x0 + 0x2000;
pub const RESETS_RESET_DONE_RW: u32 = RESETS_BASE + 0x8 + 0x0000;
pub const RESETS_RESET_DONE_XOR: u32 = RESETS_BASE + 0x8 + 0x1000;
pub const RESETS_RESET_DONE_SET: u32 = RESETS_BASE + 0x8 + 0x2000;
pub const RESETS_RESET_DONE_CLR: u32 = RESETS_BASE + 0x8 + 0x3000;

pub const USB_DPRAM_SIZE: u32 = 4096;
pub const USBCTRL_DPRAM_BASE: u32 = 0x50100000;
pub const USBCTRL_BASE: u32 = 0x50100000;
pub const USBCTRL_REGS_BASE: u32 = 0x50110000;

const USB_INTS_SETUP_REQ_BITS: u32 = 0x00010000;

pub const IO_BANK0_BASE: u32 = 0x40014000;
pub const IO_BANK0_GPIO25_STATUS_RW: u32 = IO_BANK0_BASE + 0x0C8 + 0x0000;
pub const IO_BANK0_GPIO25_STATUS_XOR: u32 = IO_BANK0_BASE + 0x0C8 + 0x1000;
pub const IO_BANK0_GPIO25_STATUS_SET: u32 = IO_BANK0_BASE + 0x0C8 + 0x2000;
pub const IO_BANK0_GPIO25_STATUS_CLR: u32 = IO_BANK0_BASE + 0x0C8 + 0x3000;
pub const IO_BANK0_GPIO25_CTRL_RW: u32 = IO_BANK0_BASE + 0x0CC + 0x0000;
pub const IO_BANK0_GPIO25_CTRL_XOR: u32 = IO_BANK0_BASE + 0x0CC + 0x1000;
pub const IO_BANK0_GPIO25_CTRL_SET: u32 = IO_BANK0_BASE + 0x0CC + 0x2000;
pub const IO_BANK0_GPIO25_CTRL_CLR: u32 = IO_BANK0_BASE + 0x0CC + 0x3000;

pub const PADS_BANK0_GPIO0_PUE_RESET: u32 = 0x0;
pub const PADS_BANK0_GPIO0_PUE_BITS: u32 = 0x00000008;
pub const PADS_BANK0_GPIO0_PUE_MSB: u32 = 3;
pub const PADS_BANK0_GPIO0_PUE_ACCESS: &str = "RW";

pub const PADS_BANK0_BASE: u32 = 0x4001c000;
pub const PADS_BANK0_GPIO0_PUE_LSB: u32 = 3;
pub const PADS_BANK0_GPIO0_PDE_LSB: u32 = 2;

pub const PSM_BASE: u32 = 0x40010000;
pub const PSM_WDSEL: *mut u32 = (PSM_BASE + 0x8) as *mut u32;
pub const PSM_WDSEL_BITS: u32 = 0x0001ffff;
pub const WATCHDOG_BASE: u32 = 0x40058000;
