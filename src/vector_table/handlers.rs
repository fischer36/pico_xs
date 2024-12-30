#![no_std]
#![no_main]

extern "C" {
    // Declare the symbols from the linker script for memory boundaries
    fn entry() -> !;
    static mut __sidata: u32;
    static mut __sdata: u32;
    static mut __edata: u32;
    static mut __sbss: u32;
    static mut __ebss: u32;
}

#[no_mangle]
unsafe extern "C" fn RESET() -> ! {
    // Copy the data segment initializers from flash to SRAM
    let mut source = &__sidata as *const u32;
    let mut destination = &mut __sdata as *mut u32;

    while destination < &mut __edata as *mut u32 {
        core::ptr::write(destination, core::ptr::read(source));
        destination = destination.add(1);
        source = source.add(1);
    }

    // Zero fill the bss segment
    destination = &mut __sbss as *mut u32;
    while destination < &mut __ebss as *mut u32 {
        core::ptr::write(destination, 0);
        destination = destination.add(1);
    }

    // Call the main application
    entry();

    // In case main returns, ensure a safe hang
    loop {}
}

// Default Handler For Unfound Handler and Interrupts Symbols
#[no_mangle]
extern "C" fn DEFAULT_HANDLER() -> ! {
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
