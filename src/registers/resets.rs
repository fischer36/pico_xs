use crate::xs::Bits;
const BASE: u32 = 0x4000C000;

const RESET_DONE: *mut u32 = (BASE + 0x8) as *mut u32;
const RESET_CLR: *mut u32 = (BASE + 0x3000) as *mut u32;

const RESET_MASK: u32 = 0b_00000001111111111111111111111111;

/// Resets peripherals specified in the mask and waits for them to be out of reset.
///
/// This function writes to the RESET_CLR register to clear the reset state of the
/// peripherals defined by the mask. It then continuously checks the RESET_DONE
/// register until all specified peripherals are reported to be out of reset.
///
/// # Parameters
///
/// - `mask`: A mask of bits corresponding to the peripherals to reset. Each bit
///   represents a different peripheral, with `1` indicating that the peripheral
///   should be reset.
///
/// # Behavior of Bits (0..24)
///
/// - `0`: ADC - Analog-to-digital converter
/// - `1`: BUSCTRL - Bus control
/// - `2`: DMA - Direct Memory Access controller
/// - `3`: I2C0 - I2C bus interface 0
/// - `4`: I2C1 - I2C bus interface 1
/// - `5`: IO_BANK0 - IO bank 0
/// - `6`: IO_QSPI - IO quality SPI
/// - `7`: JTAG - JTAG interface
/// - `8`: PADS_BANK0 - Pad control for bank 0
/// - `9`: PADS_QSPI - Pad control for QSPI
/// - `10`: PIO0 - Programmable I/O 0
/// - `11`: PIO1 - Programmable I/O 1
/// - `12`: PLL_SYS - System PLL
/// - `13`: PLL_USB - USB PLL
/// - `14`: PWM - Pulse Width Modulation controller
/// - `15`: RTC - Real-Time Clock
/// - `16`: SPI0 - SPI interface 0
/// - `17`: SPI1 - SPI interface 1
/// - `18`: SYSCFG - System configuration controller
/// - `19`: SYSINFO - System information controller
/// - `20`: TBMAN - Traceback manager
/// - `21`: TIMER - Timer peripheral
/// - `22`: UART0 - UART interface 0
/// - `23`: UART1 - UART interface 1
/// - `24`: USBCTRL - USB control interface
///
/// Bits 25 to 31 are reserved and should not be used for reset control.
///
/// # Example
///
/// ```
/// // Reset and wait for IO_BANK0 (5) and PADS_BANK0 (8) to be out of reset
/// reset_wait(1<<5|1<<8);
/// ```
pub fn reset_wait(mask: u32) {
    unsafe {
        // Modify bits in RESET_CLR to reset
        RESET_CLR.modify(RESET_MASK, mask);
        // Wait until RESET_DONE indicates all specified peripherals are out of reset
        while RESET_DONE.bits(RESET_MASK) & mask == 0 {
            core::arch::asm!("nop");
        }
    }
}
