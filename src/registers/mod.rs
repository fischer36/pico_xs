pub mod io_bank;
pub mod pads_bank;
pub mod ppb_base;
pub mod resets;
pub mod rom;
pub mod sio;
//pub mod spi;
//pub mod clocks;
pub mod watchdog;
pub mod xosc;

pub const THIRTY_TWO_BITS: u32 = 0b_1001_0101_0101_0101_0101_0101_0101_0101;
pub const ROM_BASE: u32 = 0x00000000;
pub const XIP_BASE: u32 = 0x10000000;
pub const XIP_MAIN_BASE: u32 = 0x10000000;
pub const XIP_NOALLOC_BASE: u32 = 0x11000000;
pub const XIP_NOCACHE_BASE: u32 = 0x12000000;
pub const XIP_NOCACHE_NOALLOC_BASE: u32 = 0x13000000;
pub const XIP_CTRL_BASE: u32 = 0x14000000;
pub const XIP_SRAM_BASE: u32 = 0x15000000;
pub const XIP_SRAM_END: u32 = 0x15004000;
pub const XIP_SSI_BASE: u32 = 0x18000000;
pub const SRAM_BASE: u32 = 0x20000000;
pub const SRAM_STRIPED_BASE: u32 = 0x20000000;
pub const SRAM_STRIPED_END: u32 = 0x20040000;
pub const SRAM4_BASE: u32 = 0x20040000;
pub const SRAM5_BASE: u32 = 0x20041000;
pub const SRAM_END: u32 = 0x20042000;
pub const SRAM0_BASE: u32 = 0x21000000;
pub const SRAM1_BASE: u32 = 0x21010000;
pub const SRAM2_BASE: u32 = 0x21020000;
pub const SRAM3_BASE: u32 = 0x21030000;
pub const SYSINFO_BASE: u32 = 0x40000000;
pub const SYSCFG_BASE: u32 = 0x40004000;
pub const CLOCKS_BASE: u32 = 0x40008000;
pub const RESETS_BASE: u32 = 0x4000c000;
pub const PSM_BASE: u32 = 0x40010000;
pub const IO_BANK0_BASE: u32 = 0x40014000;
pub const IO_QSPI_BASE: u32 = 0x40018000;
pub const PADS_QSPI_BASE: u32 = 0x40020000;
pub const XOSC_BASE: u32 = 0x40024000;
pub const PLL_SYS_BASE: u32 = 0x40028000;
pub const PLL_USB_BASE: u32 = 0x4002c000;
pub const BUSCTRL_BASE: u32 = 0x40030000;
pub const UART0_BASE: u32 = 0x40034000;
pub const UART1_BASE: u32 = 0x40038000;
pub const SPI0_BASE: u32 = 0x4003c000;
pub const SPI1_BASE: u32 = 0x40040000;
pub const I2C0_BASE: u32 = 0x40044000;
pub const I2C1_BASE: u32 = 0x40048000;
pub const ADC_BASE: u32 = 0x4004c000;
pub const PWM_BASE: u32 = 0x40050000;
pub const TIMER_BASE: u32 = 0x40054000;
pub const WATCHDOG_BASE: u32 = 0x40058000;
pub const RTC_BASE: u32 = 0x4005c000;
pub const ROSC_BASE: u32 = 0x40060000;
pub const VREG_AND_CHIP_RESET_BASE: u32 = 0x40064000;
pub const TBMAN_BASE: u32 = 0x4006c000;
pub const DMA_BASE: u32 = 0x50000000;
pub const USBCTRL_DPRAM_BASE: u32 = 0x50100000;
pub const USBCTRL_BASE: u32 = 0x50100000;
pub const USBCTRL_REGS_BASE: u32 = 0x50110000;
pub const PIO0_BASE: u32 = 0x50200000;
pub const PIO1_BASE: u32 = 0x50300000;
pub const XIP_AUX_BASE: u32 = 0x50400000;
pub const SIO_BASE: u32 = 0xd0000000;
pub const PPB_BASE: u32 = 0xe0000000;
