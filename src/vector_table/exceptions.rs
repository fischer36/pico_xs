#[no_mangle]
unsafe extern "C" fn HardFault_() -> ! {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn SVCall() -> ! {
    loop {}
}
#[no_mangle]
unsafe extern "C" fn PendSv() -> ! {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn SysTick() -> ! {
    loop {}
}
