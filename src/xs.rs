//! # XS (Extra-Small)
//!
//! `xs.rs`: collection of generic embedded utilities.

use core::ptr::{read_volatile, write_volatile};
/// Trait for bit-level manipulation of registers.
///
/// This trait should be implemented by types that represent mutable
/// memory addresses, such as hardware registers. It provides methods
/// to clear, set, modify, and read bits.
///
/// # Type Parameter
///
/// - `T`: The integer type for masks and values, typically `u8`, `u16`, `u32`, or `u64`.
pub trait Bits<T> {
    /// Clears specified bits.
    ///
    /// # Parameters
    ///
    /// - `bits`: Mask of bits to clear.
    fn clear(self, bits: T);

    /// Sets specified bits.
    ///
    /// # Parameters
    ///
    /// - `bits`: Mask of bits to set.
    fn set(self, bits: T);

    /// Modifies bits by clearing and setting based on the mask.
    ///
    /// # Parameters
    ///
    /// - `mask`: Mask of bits to clear.
    /// - `bits`: Bits to set within the cleared mask.
    fn modify(self, mask: T, bits: T);

    /// Xor bits
    ///
    /// # Parameters
    ///
    /// - `bits`: Bits to xor.
    fn xor(self, bits: T);

    /// Reads specified bits.
    ///
    /// # Parameters
    ///
    /// - `mask`: A mask that specifies which bits to read. Bits set to `1` in the mask
    ///   will be included in the returned value.
    ///
    /// # Returns
    ///
    /// - Returns the value of the bits that are set in the mask. The returned value
    ///   has the bits in their original positions.
    fn bits(self, mask: T) -> T;
}

impl Bits<u32> for *mut u32 {
    fn clear(self, bits: u32) {
        unsafe {
            write_volatile(self, read_volatile(self) & !bits);
        }
    }
    fn set(self, bits: u32) {
        unsafe {
            write_volatile(self, read_volatile(self) | bits);
        }
    }
    fn modify(self, mask: u32, bits: u32) {
        unsafe {
            write_volatile(self, (read_volatile(self) & !mask) | (mask & bits));
        }
    }
    fn xor(self, bits: u32) {
        unsafe {
            write_volatile(self, read_volatile(self) ^ bits);
        }
    }
    fn bits(self, mask: u32) -> u32 {
        unsafe { read_volatile(self) & mask }
    }
}

pub fn sleep() {
    unsafe {
        for _ in 0..50_000 {
            core::arch::asm!("nop");
        }
    }
}
