// Ensure the crate does not link the Rust standard library, as this is a bare-metal environment.
#![no_std]

//! # Pico XS - Hardware Abstraction Layer (HAL) for the RP2040
//!
//! Pico XS provides a low-level abstraction over the RP2040 microcontroller, facilitating
//! bare-metal programming with Rust. This HAL includes initial boot configurations,
//! setup and management of peripherals.
//!
//! ## Features
//! - **Completely Independent**: From the bootloader to peripherals and protocols,
//! everything is designed from the ground up to be self-contained, ensuring full control
//! without reliance on external software.
//! - **Minimalistic**: Simple and lightweight abstractions; facilitating direct and portable access to
//! the underlying hardware code.
//!
//! ## Example
//! cargo run --example blinky
//!
//! ## Usage
//! Include `pico_xs` in your Cargo.toml and refer to the example code for initial setup and
//! usage.
//!
/// Hardware register modules, providing low-level access to system registers.
pub mod registers;
/// Interrupt vector handling for the RP2040.
pub mod vector_table;

/// GPIO module for general-purpose input/output functionalities.
pub mod gpio;
/// Extended support and utility functions.
pub mod xs;

// Holds the embedded stage-2 bootloader, required for initializing the RP2040 microcontroller
// (borrowed from rp2040-boot2 for now).
#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = *include_bytes!("../boot2.bin");

/// System entry point, called on by the reset handler on microcontroller reset.
/// Initializes hardware by clearing spinlocks and jumping to the user-defined `main` function.
#[no_mangle]
pub extern "C" fn entry() -> ! {
    clear_spinlocks();
    unsafe {
        main(); // User-defined main, must never return.
    }
}

/// External declaration of the main function that is implemented by the user.
/// This function should never return and is the entry point for application logic.
extern "Rust" {
    fn main() -> !;
}

/// Clears all hardware spinlocks at a predefined base address, ensuring that
/// all peripherals are accessible and not locked by default.
pub fn clear_spinlocks() {
    let spinlocks_base: *mut u32 = (0xd0000000_u32 + 0x100) as *mut u32;

    const SPINLOCK_COUNT: usize = 32;
    unsafe {
        for i in 0..SPINLOCK_COUNT {
            spinlocks_base.wrapping_add(i).write_volatile(1); // Unlock each spinlock.
        }
    }
}
