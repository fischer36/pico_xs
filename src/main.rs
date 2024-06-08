#![no_std]
#![no_main]

use cortex_m_rt::exception;
use rp_pico::entry;
pub mod io;
pub mod pads;
pub mod regs;
pub mod reset;
pub mod sio;
pub mod timer;
pub mod watchdog;

use watchdog::{did_reboot, update};

#[entry]
fn main() -> ! {
    reset::reset();
    timer::set_timer();
    sio::oe_clr(15);
    sio::out_clr(15);
    io::gpio_ctrl(15);
    pads::set_pulls(15, true, false);

    sio::oe_clr(25);
    sio::out_clr(25);
    io::gpio_ctrl(25);
    sio::oe_set(25);

    let mut on = false;

    // if did_reboot() == true {
    //     watchdog::enable(300, true);
    // } else {
    //     watchdog::enable(300, false);
    // }
    // let mut counter = 0;
    loop {
        // counter += 1;
        // if counter == 15 {
        //     loop {
        //         sleep();
        //     }
        // }

        // update();
        sleep();
        sio::out_set(25);
        // update();
        sleep();
        sio::out_clr(25);
        // update();
        // // if gpio_in(15) == true {
        // if on == true {
        //     sio::out_clr(25);
        //     on = false
        // } else {
        //     sio::out_set(25);
        //     on = true
        // }
        // sleep();
        // // }
        //
        // update();
    }
}
// #[exception]
// fn SysTick() {
//     todo!();
// }
// // This is the function we will use to replace TIMER_IRQ_0 in our RAM Vector Table
// extern "C" fn timer_irq0_replacement() {
//     critical_section::with(|cs| {
//         let ledalarm = unsafe { LED_AND_ALARM.borrow(cs).take() };
//         if let Some((mut led, mut alarm)) = ledalarm {
//             // Clear the alarm interrupt or this interrupt service routine will keep firing
//             alarm.clear_interrupt();
//             // Schedule a new alarm after FAST_BLINK_INTERVAL_US have passed (300 milliseconds)
//             let _ = alarm.schedule(FAST_BLINK_INTERVAL_US);
//             led.toggle().unwrap();
//             // Return LED_AND_ALARM into our static variable
//             unsafe {
//                 LED_AND_ALARM
//                     .borrow(cs)
//                     .replace_with(|_| Some((led, alarm)));
//             }
//         }
//     });
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
fn sleep_for_cycles(cycle_count: u32) {
    let count = if cycle_count < 10_000 || cycle_count > 1_000_000 {
        10_0000
    } else {
        cycle_count
    };
    unsafe {
        for _ in 0..cycle_count {
            core::arch::asm!("nop");
        }
    }
}
fn sleep() {
    unsafe {
        for _ in 0..500_000 {
            core::arch::asm!("nop");
        }
    }
}
