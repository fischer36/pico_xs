use crate::regs;
use crate::sio;

//cfg_global_asm! {
//    ".cfi_sections .debug_frame
//     .section .Reset, \"ax\"
//     .global Reset
//     .type Reset,%function
//     .thumb_func",
//    ".cfi_startproc
//     Reset:",
//
//    // If enabled, initialise the SP. This is normally initialised by the CPU itself or by a
//    // bootloader, but some debuggers fail to set it when resetting the target, leading to
//    // stack corruptions.
//    #[cfg(feature = "set-sp")]
//    "ldr r0, =_stack_start
//     msr msp, r0",
//
//    // If enabled, initialise VTOR to the start of the vector table. This is normally initialised
//    // by a bootloader when the non-reset value is required, but some bootloaders do not set it,
//    // leading to frustrating issues where everything seems to work but interrupts are never
//    // handled. The VTOR register is optional on ARMv6-M, but when not present is RAZ,WI and
//    // therefore safe to write to.
//    #[cfg(feature = "set-vtor")]
//    "ldr r0, =0xe000ed08
//     ldr r1, =__vector_table
//     str r1, [r0]",
//
//    // Run user pre-init code which must be executed immediately after startup, before the
//    // potentially time-consuming memory initialisation takes place.
//    // Example use cases include disabling default watchdogs or enabling RAM.
//    "bl __pre_init",
//
//    // If enabled, initialize RAM with zeros. This is not usually required, but might be necessary
//    // to properly initialize checksum-based memory integrity measures on safety-critical hardware.
//    #[cfg(feature = "zero-init-ram")]
//    "ldr r0, =_ram_start
//     ldr r1, =_ram_end
//     movs r2, #0
//     0:
//     cmp r1, r0
//     beq 1f
//     stm r0!, {{r2}}
//     b 0b
//     1:",
//
//    // Initialise .bss memory. `__sbss` and `__ebss` come from the linker script.
//    #[cfg(not(feature = "zero-init-ram"))]
//    "ldr r0, =__sbss
//     ldr r1, =__ebss
//     movs r2, #0
//     2:
//     cmp r1, r0
//     beq 3f
//     stm r0!, {{r2}}
//     b 2b
//     3:",
//
//    // Initialise .data memory. `__sdata`, `__sidata`, and `__edata` come from the linker script.
//    "ldr r0, =__sdata
//     ldr r1, =__edata
//     ldr r2, =__sidata
//     4:
//     cmp r1, r0
//     beq 5f
//     ldm r2!, {{r3}}
//     stm r0!, {{r3}}
//     b 4b
//     5:",
//
//    // Potentially enable an FPU.
//    // SCB.CPACR is 0xE000_ED88.
//    // We enable access to CP10 and CP11 from priviliged and unprivileged mode.
//    #[cfg(has_fpu)]
//    "ldr r0, =0xE000ED88
//     ldr r1, =(0b1111 << 20)
//     ldr r2, [r0]
//     orr r2, r2, r1
//     str r2, [r0]
//     dsb
//     isb",
//
//    // Jump to user main function.
//    // `bl` is used for the extended range, but the user main function should not return,
//    // so trap on any unexpected return.
//    "bl main
//     udf #0",
//
//    ".cfi_endproc
//     .size Reset, . - Reset",
//}
#[no_mangle]
unsafe extern "C" fn HardFault_() -> ! {
    loop {}
}
#[no_mangle]
extern "C" fn DefaultHandler_() -> ! {
    loop {}
}
//#[no_mangle]
//extern "C" fn Reset() -> ! {
//    loop {}
//}
#[no_mangle]
unsafe extern "C" fn timer_irq_handler() {
    sio::out_set(25);
    unsafe {
        // clear bit to disable interrupt lathced to timer
        let timer_intrrupt: *mut u32 = (regs::TIMER_BASE + 0x34) as *mut u32;
        let old = core::ptr::read_volatile(timer_intrrupt);
        core::ptr::write_volatile(timer_intrrupt, old & !(1 << 0));
    }
}

#[no_mangle]
unsafe extern "C" fn usb_irq_handler() {
    sio::out_set(25);
    unsafe {
        //// clear bit to disable interrupt lathced to timer
        //let timer_intrrupt: *mut u32 = (regs::TIMER_BASE + 0x34) as *mut u32;
        //let old = core::ptr::read_volatile(timer_intrrupt);
        //core::ptr::write_volatile(timer_intrrupt, old & !(1 << 0));
    }
}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
