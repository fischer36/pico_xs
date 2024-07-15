#[no_mangle]
unsafe extern "C" fn HardFault_() -> ! {
    loop {}
}
#[no_mangle]
extern "C" fn DefaultHandler_() -> ! {
    loop {}
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
