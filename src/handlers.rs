use crate::regs;
use crate::sio;

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
