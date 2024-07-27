// UART0_BASE, RESET_BIT = 22
const BASE0: u32 = 0x40034000;

// UART1_BASE, RESET_BIT = 23
const BASE1: u32 = 0x40038000;
use crate::xs::Bits;
//fn set_baudrate()
//pub fn init() {
//    // Setup Clk Peri
//    // Reset Uart Peripheral
//    // Set baudrate
//    // Set format  e
//    super::resets::reset_wait(1 << 22);
//
//
//    // Reset UART0 and UART1
//    // Enable Clk Peri Ctrl
//    //super::clocks::PERI_CTRL.set(1 << 11);
//    let uart0 = Uart::new(BASE0);
//    // SET BUAD DIVINT
//    uart0.UARTIBRD.clear(0b_1111_1111_1111_1111);
//    uart0.UARTIBRD.set(78);
//    // SET BAUD DIVFRAC
//    uart0.UARTFBRD.clear(0b1_1111);
//    uart0.UARTFBRD.set(1 << 3);
//    // SET WLEN BITS TO 1 (3/11)
//    uart0.UARTLCR_H.set(3 << 5);
//    // SET BITS: UARTEN (0), TXE (8) and RXE (9)
//    uart0.UARTCR.set(1 << 0 | 1 << 8 | 1 << 9);
//
//    // Select funcsel 2 for gpio0 (uart0_tx)
//    let mut gpio0 = crate::gpio::Gpio::new(0);
//    gpio0.oe.clr();
//    gpio0.out.clr();
//    gpio0.select_funcsel(2);
//
//
//    // Select funcsel 2 for gpio1 (uart0_rx)
//    let mut gpio1 = crate::gpio::Gpio::new(1);
//    gpio1.select_funcsel(2);
//    // SET BITS: RXIM(4) and RTIM (6)
//    uart0.UARTIMSC.set(1 << 4 | 1 << 6);
//
//    // CLEAR RXIFLSEL BITS (5:3)
//    uart0.UARTIFLS.clear(0b111 << 3);
//}

// Implemented for Uart0 and Uart1, as they have separate memory locations but share functionality
pub struct Uart {
    pub UARTDR: *mut u32,        //0x000
    pub UARTRSR: *mut u32,       //0x004
    pub UARTFR: *mut u32,        // 0x018
    pub UARTILPR: *mut u32,      // 0x020
    pub UARTIBRD: *mut u32,      //0x024
    pub UARTFBRD: *mut u32,      //0x028
    pub UARTLCR_H: *mut u32,     //0x02c
    pub UARTCR: *mut u32,        //0x030
    pub UARTIFLS: *mut u32,      //0x034
    pub UARTIMSC: *mut u32,      //0x038
    pub UARTRIS: *mut u32,       //0x03c
    pub UARTMIS: *mut u32,       //0x040
    pub UARTICR: *mut u32,       //0x044
    pub UARTDMACR: *mut u32,     //0x048
    pub UARTPERIPHID0: *mut u32, //0xfe0
    pub UARTPERIPHID1: *mut u32, //0xfe4
    pub UARTPERIPHID2: *mut u32, //0xfe8
    pub UARTPERIPHID3: *mut u32, //0xfec
    pub UARTPCELLID0: *mut u32,  //0xff0
    pub UARTPCELLID1: *mut u32,  //0xff4
    pub UARTPCELLID2: *mut u32,  //0xff8
    pub UARTPCELLID3: *mut u32,  //0xffc
}

impl Uart {
    // BASE ADDRESS OF ETHIER UART0 OR UART1
    pub fn new(base: u32) -> Self {
        assert!(base == BASE0 || base == BASE1);
        Self {
            UARTDR: (base + 0x000) as *mut u32,
            UARTRSR: (base + 0x004) as *mut u32,
            UARTFR: (base + 0x018) as *mut u32,
            UARTILPR: (base + 0x020) as *mut u32,
            UARTIBRD: (base + 0x024) as *mut u32,
            UARTFBRD: (base + 0x028) as *mut u32,
            UARTLCR_H: (base + 0x02c) as *mut u32,
            UARTCR: (base + 0x030) as *mut u32,
            UARTIFLS: (base + 0x034) as *mut u32,
            UARTIMSC: (base + 0x038) as *mut u32,
            UARTRIS: (base + 0x03c) as *mut u32,
            UARTMIS: (base + 0x040) as *mut u32,
            UARTICR: (base + 0x044) as *mut u32,
            UARTDMACR: (base + 0x048) as *mut u32,
            UARTPERIPHID0: (base + 0xfe0) as *mut u32,
            UARTPERIPHID1: (base + 0xfe4) as *mut u32,
            UARTPERIPHID2: (base + 0xfe8) as *mut u32,
            UARTPERIPHID3: (base + 0xfec) as *mut u32,
            UARTPCELLID0: (base + 0xff0) as *mut u32,
            UARTPCELLID1: (base + 0xff4) as *mut u32,
            UARTPCELLID2: (base + 0xff8) as *mut u32,
            UARTPCELLID3: (base + 0xffc) as *mut u32,
        }
    }
}
