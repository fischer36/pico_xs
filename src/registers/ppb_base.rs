// PPB_BASE / MO_PLUS
use crate::xs::Bits;
const BASE: u32 = 0xe0000000;
// Interrupt set enable
pub const NVIC_ISER: *mut u32 = (BASE + 0xE100) as *mut u32;
// Interrupt clear enable
pub const NVIC_ICER: *mut u32 = (BASE + 0xE180) as *mut u32;
// Interrupt set pending
pub const NVIC_ISPR: *mut u32 = (BASE + 0xE200) as *mut u32;
// Interrupt clear pending
pub const NVIC_ICPR: *mut u32 = (BASE + 0xE280) as *mut u32;

pub enum Interrupt {
    TIMER_IRQ_0 = 0,
    TIMER_IRQ_1 = 1,
    TIMER_IRQ_2 = 2,
    TIMER_IRQ_3 = 3,
    PWM_IRQ_WRAP = 4,
    USBCTRL_IRQ = 5,
    XIP_IRQ = 6,
    PIO0_IRQ_0 = 7,
    PIO0_IRQ_1 = 8,
    PIO1_IRQ_0 = 9,
    PIO1_IRQ_1 = 10,
    DMA_IRQ_0 = 11,
    DMA_IRQ_1 = 12,
    IO_IRQ_BANK0 = 13,
    IO_IRQ_QSPI = 14,
    SIO_IRQ_PROC0 = 15,
    SIO_IRQ_PROC1 = 16,
    CLOCKS_IRQ = 17,
    SPI0_IRQ = 18,
    SPI1_IRQ = 19,
    UART0_IRQ = 20,
    UART1_IRQ = 21,
    ADC_IRQ_FIFO = 22,
    I2C0_IRQ = 23,
    I2C1_IRQ = 24,
    RTC_IRQ = 25,
}

pub fn enable_interrupt(interrupt: Interrupt) {
    NVIC_ISER.set(1 << 0 as u32);
}

pub fn disable_interrupt(interrupt: Interrupt) {
    NVIC_ISER.clear(1 << 0 as u32);
}

pub fn set_pending(interrupt: Interrupt) {
    NVIC_ICPR.clear(1 << interrupt as u32);
}
pub fn clear_pending(interrupt: Interrupt) {
    NVIC_ICPR.set(1 << interrupt as u32);
}
