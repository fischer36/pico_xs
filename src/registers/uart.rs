// UART0_BASE, RESET_BIT = 22
use crate::xs::Bits;

use super::resets::{self, RESET_BIT_UART0, RESET_BIT_UART1};
const BASE0: u32 = 0x40034000;
// UART1_BASE, RESET_BIT = 23
const BASE1: u32 = 0x40038000;

pub struct Uart {
    base: u32,
}

impl Uart {
    pub fn init(uart_number: u8, clk_peri_hz: u32) -> Self {
        // Get base address of UARt 0 or 1: the following offsets remain the same from the base
        let base: u32;
        let reset: u32;
        if uart_number == 0 {
            // UART 0
            base = BASE0;
            reset = RESET_BIT_UART0;
        } else {
            // UART 1
            base = BASE1;
            reset = RESET_BIT_UART1;
        }
        // Reset Corresponding UART Sub System
        super::resets::reset_wait(reset);
        let uart: Uart = Self { base };
        // Set BaudRate with clk_peri_hz for UART synchronization
        uart.set_baudrate(115200, clk_peri_hz);

        // Set format
        uart.set_format(1);

        let uartlcr_h: *mut u32 = (uart.base + 0x02c) as *mut u32; //0x4003402c.set(1 << 4); //0x02c
        uartlcr_h.set(1 << 4);

        // Enable (0), Transmit (8), Recieve (9)
        let uartcr: *mut u32 = (uart.base + 0x030) as *mut u32;
        uartcr.set(1 << 8 | 1 << 9 | 1 << 0);

        // Set Dma transmit (1) and recieve (0)
        let uartdmacr: *mut u32 = (uart.base + 0x048) as *mut u32;
        uartdmacr.set(1 << 0 | 1 << 1);

        return uart;
    }

    fn set_baudrate(&self, baudrate: u32, clk_peri_hz: u32) {
        let baudrate_div = 8 * clk_peri_hz / baudrate;
        let baudrate_ibrd = baudrate_div >> 7;
        let baudrate_fbrd = (baudrate_ibrd & 0x7f) + 1 / 2;

        let uartibrd: *mut u32 = (self.base + 0x024) as *mut u32; // 15:0
        let uartfbrd: *mut u32 = (self.base + 0x028) as *mut u32; // 5:0

        uartibrd.set(baudrate_ibrd);
        uartfbrd.set(baudrate_fbrd);

        // // PL011 needs a (dummy) LCR_H write to latch in the divisors.
        //  // We don't want to actually change LCR_H contents here.
        //  uart_write_lcr_bits_masked(uart, 0, 0);
        //
        //  // See datasheet
        //  return (4 * clock_get_hz(clk_peri)) / (64 * baud_ibrd + baud_fbrd);
    }

    fn set_format(&self, bits: u32) {
        // Bits 7:0 defines format
        const UART_UARTLCR_H_SPS_MASK: u32 = 0x80;
        const UART_UARTLCR_H_WLEN_MASK: u32 = 0x60;
        const UART_UARTLCR_H_FEN_MASK: u32 = 0x10;
        const UART_UARTLCR_H_STP2_MASK: u32 = 0x08;
        const UART_UARTLCR_H_EPS_MASK: u32 = 0x04;
        const UART_UARTLCR_H_PEN_MASK: u32 = 0x02;
        const UART_UARTLCR_H_BRK_MASK: u32 = 0x01;
        const UART_UARTLCR_MASK: u32 = 0b_11111111;
        let uartlcr_h: *mut u32 = (self.base + 0x02c) as *mut u32;
        unsafe {
            uartlcr_h.write_volatile(bits & UART_UARTLCR_MASK);
        }
    }
}

//
// use crate::xs::Bits;
//fn set_baudrate()
//pub fn init() {

// Implemented for Uart0 and Uart1, as they have separate memory locations but share functionality
// pub struct Uart {
//     pub UARTDR: *mut u32,        //0x000
//     pub UARTRSR: *mut u32,       //0x004
//     pub UARTFR: *mut u32,        // 0x018
//     pub UARTILPR: *mut u32,      // 0x020
//     pub UARTIBRD: *mut u32,      //0x024
//     pub UARTFBRD: *mut u32,      //0x028
//     pub UARTLCR_H: *mut u32,     //0x02c
//     pub UARTCR: *mut u32,        //0x030
//     pub UARTIFLS: *mut u32,      //0x034
//     pub UARTIMSC: *mut u32,      //0x038
//     pub UARTRIS: *mut u32,       //0x03c
//     pub UARTMIS: *mut u32,       //0x040
//     pub UARTICR: *mut u32,       //0x044
//     pub UARTDMACR: *mut u32,     //0x048
//     pub UARTPERIPHID0: *mut u32, //0xfe0
//     pub UARTPERIPHID1: *mut u32, //0xfe4
//     pub UARTPERIPHID2: *mut u32, //0xfe8
//     pub UARTPERIPHID3: *mut u32, //0xfec
//     pub UARTPCELLID0: *mut u32,  //0xff0
//     pub UARTPCELLID1: *mut u32,  //0xff4
//     pub UARTPCELLID2: *mut u32,  //0xff8
//     pub UARTPCELLID3: *mut u32,  //0xffc
// }
//
// impl Uart {
//     // BASE ADDRESS OF ETHIER UART0 OR UART1
//     pub fn new(base: u32) -> Self {
//         assert!(base == BASE0 || base == BASE1);
//         Self {
//             UARTDR: (base + 0x000) as *mut u32,
//             UARTRSR: (base + 0x004) as *mut u32,
//             UARTFR: (base + 0x018) as *mut u32,
//             UARTILPR: (base + 0x020) as *mut u32,
//             UARTIBRD: (base + 0x024) as *mut u32,
//             UARTFBRD: (base + 0x028) as *mut u32,
//             UARTLCR_H: (base + 0x02c) as *mut u32,
//             UARTCR: (base + 0x030) as *mut u32,
//             UARTIFLS: (base + 0x034) as *mut u32,
//             UARTIMSC: (base + 0x038) as *mut u32,
//             UARTRIS: (base + 0x03c) as *mut u32,
//             UARTMIS: (base + 0x040) as *mut u32,
//             UARTICR: (base + 0x044) as *mut u32,
//             UARTDMACR: (base + 0x048) as *mut u32,
//             UARTPERIPHID0: (base + 0xfe0) as *mut u32,
//             UARTPERIPHID1: (base + 0xfe4) as *mut u32,
//             UARTPERIPHID2: (base + 0xfe8) as *mut u32,
//             UARTPERIPHID3: (base + 0xfec) as *mut u32,
//             UARTPCELLID0: (base + 0xff0) as *mut u32,
//             UARTPCELLID1: (base + 0xff4) as *mut u32,
//             UARTPCELLID2: (base + 0xff8) as *mut u32,
//             UARTPCELLID3: (base + 0xffc) as *mut u32,
//         }
//     }
// }
