#![no_std]
#![no_main]
use rp_pico::entry;
mod pins;
mod watchdog;
use pins::{
    io::*,
    pads::{self, set_pulls},
    resets::reset,
    sio::*,
};
use watchdog::{did_reboot, update};
#[entry]
fn main() -> ! {
    reset();
    oe_clr(15);
    out_clear(15);
    gpio_ctrl(15);
    set_pulls(15, true, false);

    oe_clr(25);
    out_clear(25);
    gpio_ctrl(25);
    oe_set(25);

    let mut on = false;

    if did_reboot() == true {
        watchdog::enable(300, true);
    } else {
        watchdog::enable(300, false);
    }
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 15 {
            loop {
                sleep();
            }
        }

        update();
        out_set(25);
        update();
        sleep();
        out_clear(25);
        update();
        sleep();
        // // if gpio_in(15) == true {
        // if on == true {
        //     out_clear(25);
        //     on = false
        // } else {
        //     out_set(25);
        //     on = true
        // }
        // sleep();
        // // }
        //
        // update();
    }
}

fn sleep() {
    unsafe {
        for _ in 0..10_000 {
            core::arch::asm!("nop");
        }
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
