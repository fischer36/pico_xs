#[no_mangle]
pub extern "C" fn NON_MASKABLE_INT() -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn HARD_FAULT() -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn SV_CALL() -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn PEND_SV() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn SYS_TICK() -> ! {
    loop {}
}
