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
pub mod vector_table;
pub mod watchdog;
use sio::out_set;
use vector_table::VectorTable;
static mut RAM_VTABLE: VectorTable = VectorTable::new();
use watchdog::{did_reboot, update};
static mut FIRED: bool = false;

static mut twice: bool = false;
extern "C" fn test_timer_irq0() {
    sio::out_clr(25);
    sleep();
    sleep();
    sio::out_set(25);
    // unsafe {
    //     if twice == true {
    //         sio::out_set(25)
    //     } else {
    //         sio::out_clr(25);
    //     }
    //     twice = !twice
    // }
    // clear bit to disable interrupt lathced to timer
    let timer_intrrupt: *mut u32 = (regs::TIMER_BASE + 0x34) as *mut u32;
    unsafe {
        let old = core::ptr::read_volatile(timer_intrrupt);
        core::ptr::write_volatile(timer_intrrupt, old & !(1 << 0));
    }
    unsafe {
        FIRED = true;
    }
}

#[entry]
fn main() -> ! {
    reset::reset();
    // let mut pac = rp_pico::hal::pac::Peripherals::take().unwrap();

    let scb = unsafe { &*rp_pico::hal::pac::SCB::ptr() };

    // Read the current VTOR address
    // Copy the vector table from flash to RAM
    let mut pac = rp_pico::hal::pac::Peripherals::take().unwrap();
    let ppb = &mut pac.PPB;
    unsafe {
        RAM_VTABLE.init(ppb);
        RAM_VTABLE.register_handler(
            rp2040_hal::pac::Interrupt::TIMER_IRQ_0 as usize,
            test_timer_irq0,
        );
    }

    sio::oe_clr(15);
    sio::out_clr(15);
    io::gpio_ctrl(15);
    pads::set_pulls(15, true, false);

    sio::oe_clr(25);
    sio::out_clr(25);
    io::gpio_ctrl(25);
    sio::oe_set(25);

    sio::out_set(25);

    unsafe {
        scb.vtor.write(&mut RAM_VTABLE as *mut _ as u32);
    }
    timer::set_timer();

    //     on = false
    // } else {

    // if did_reboot() == true {
    //     watchdog::enable(300, true);
    // } else {
    //     watchdog::enable(300, false);
    // }
    // let mut counter = 0;
    loop {
        sleep();

        // sio::out_clr(25);

        sleep();
        // sio::out_set(25);
        unsafe {
            if FIRED == true {
                // sio::out_clr(25);
                loop {
                    sio::out_clr(25);
                    sleep();
                    sio::out_set(25);
                }
            }
        }

        // counter += 1;
        // if counter == 15 {
        //     loop {
        //         sleep();
        //     }
        // }

        // update();
        // update();
        // update();
        // // if gpio_in(15) == true {
        // if on == true {
        //     on = true
        // }
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
        for _ in 0..500_00 {
            core::arch::asm!("nop");
        }
    }
}
