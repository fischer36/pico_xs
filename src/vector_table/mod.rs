mod exceptions;
mod interrupts;

// Vector Entry Union
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: u32,
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[link_section = ".vector_table.exceptions"]
pub static __EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector { handler: HardFault },
    // Reserved 4-10
    // Exception 4:
    Vector { reserved: 0 },
    // Exception 5:
    Vector { reserved: 0 },
    // Exception 6:
    Vector { reserved: 0 },
    // Exception 7
    Vector { reserved: 0 },
    // Exception 8
    Vector { reserved: 0 },
    // Exception 9
    Vector { reserved: 0 },
    // Exception 10
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12:
    Vector { reserved: 0 },
    // Exception 13:
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector {
        handler: TIMER_IRQ_0,
    },
    Vector {
        handler: TIMER_IRQ_1,
    },
    Vector {
        handler: TIMER_IRQ_2,
    },
    Vector {
        handler: TIMER_IRQ_3,
    },
    Vector {
        handler: PWM_IRQ_WRAP,
    },
    Vector {
        handler: USBCTRL_IRQ,
    },
    Vector { handler: XIP_IRQ },
    Vector {
        handler: PIO0_IRQ_0,
    },
    Vector {
        handler: PIO0_IRQ_1,
    },
    Vector {
        handler: PIO1_IRQ_0,
    },
    Vector {
        handler: PIO1_IRQ_1,
    },
    Vector { handler: DMA_IRQ_0 },
    Vector { handler: DMA_IRQ_1 },
    Vector {
        handler: IO_IRQ_BANK0,
    },
    Vector {
        handler: IO_IRQ_QSPI,
    },
    Vector {
        handler: SIO_IRQ_PROC0,
    },
    Vector {
        handler: SIO_IRQ_PROC1,
    },
    Vector {
        handler: CLOCKS_IRQ,
    },
    Vector { handler: SPI0_IRQ },
    Vector { handler: SPI1_IRQ },
    Vector { handler: UART0_IRQ },
    Vector { handler: UART1_IRQ },
    Vector {
        handler: ADC_IRQ_FIFO,
    },
    Vector { handler: I2C0_IRQ },
    Vector { handler: I2C1_IRQ },
    Vector { handler: RTC_IRQ },
    Vector { handler: SWI_IRQ_0 },
    Vector { handler: SWI_IRQ_1 },
    Vector { handler: SWI_IRQ_2 },
    Vector { handler: SWI_IRQ_3 },
    Vector { handler: SWI_IRQ_4 },
    Vector { handler: SWI_IRQ_5 },
];

// Default Handler For Unfound Handler Symbols
#[no_mangle]
extern "C" fn DefaultHandler() -> ! {
    loop {}
}

// Rust Panic Handler
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

// Extern Declarations
extern "C" {
    // RESET HANDLER (RESET_HANDLER.c)
    fn Reset() -> !;
    // EXCEPTION HANDLERS (EXCEPTIONS.rs)
    fn NonMaskableInt();
    fn HardFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
    // INTERRUPT HANDLERS (INTERRUPTS.rs)
    fn TIMER_IRQ_0();
    fn TIMER_IRQ_1();
    fn TIMER_IRQ_2();
    fn TIMER_IRQ_3();
    fn PWM_IRQ_WRAP();
    fn USBCTRL_IRQ();
    fn XIP_IRQ();
    fn PIO0_IRQ_0();
    fn PIO0_IRQ_1();
    fn PIO1_IRQ_0();
    fn PIO1_IRQ_1();
    fn DMA_IRQ_0();
    fn DMA_IRQ_1();
    fn IO_IRQ_BANK0();
    fn IO_IRQ_QSPI();
    fn SIO_IRQ_PROC0();
    fn SIO_IRQ_PROC1();
    fn CLOCKS_IRQ();
    fn SPI0_IRQ();
    fn SPI1_IRQ();
    fn UART0_IRQ();
    fn UART1_IRQ();
    fn ADC_IRQ_FIFO();
    fn I2C0_IRQ();
    fn I2C1_IRQ();
    fn RTC_IRQ();
    fn SWI_IRQ_0();
    fn SWI_IRQ_1();
    fn SWI_IRQ_2();
    fn SWI_IRQ_3();
    fn SWI_IRQ_4();
    fn SWI_IRQ_5();
}
