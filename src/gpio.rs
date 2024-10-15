//! Provides GPIO (General Purpose Input/Output) functionalities for the RP2040 microcontroller.
//!
//! This module includes mechanisms for initializing and configuring GPIO pins using low-level hardware access.
//! It exposes controls for pin outputs and enables hardware-specific operations such as function selection.
//!
//! # Usage
//! To use a GPIO pin:
//! - Instantiate the `Gpio` struct with the desired GPIO pin number.
//! - Utilize the struct methods to configure and operate the pin.
//!
//! # Example
//! ```
//! let mut gpio_pin = Gpio::new(5); // Create a new GPIO instance for pin 5.
//! gpio_pin.select_funcsel(0b00100); // Set the function selection to some desired mode.
//! ```
//!
use crate::registers::{io_bank, pads_bank, sio};
use crate::xs::Bits;

/// Represents a single GPIO pin on the RP2040, encapsulating its control and status.
pub struct Gpio {
    pin: u32,
    status: *mut u32,
    ctrl: *mut u32,
    pub pad_ctrl: *mut u32,
    pub out: sio::OutputSet,
    pub oe: sio::OutputSet,
}

impl Gpio {
    /// Constructs a new `Gpio` instance for the specified pin number.
    ///
    /// # Arguments
    /// * `gpio` - The GPIO pin number (must be less than 30).
    ///
    /// # Panics
    /// Panics if the provided pin number is out of range.
    pub fn new(gpio: u32) -> Self {
        assert!(gpio < 30);

        // Reset clear output- enable and clear for new instance.
        sio::gpio_oe_clr(gpio);
        sio::gpio_out_clr(gpio);

        Self {
            pin: gpio,
            status: (io_bank::BASE + 0x8 * gpio) as *mut u32,
            ctrl: (io_bank::BASE + 0x4 + 0x8 * gpio) as *mut u32,
            pad_ctrl: (pads_bank::BASE + 0x4 + 0x4 * gpio) as *mut u32,
            out: sio::OutputSet::new(sio::BASE + 0x010, gpio),
            oe: sio::OutputSet::new(sio::BASE + 0x020, gpio),
        }
    }

    /// Configures the function selection for the GPIO pin.
    ///
    ///
    /// # Arguments
    /// * `funcsel` - The function selection code, which configures the pin's mode (e.g., as input, output, alt function).
    ///
    /// # Notes
    /// - The `funcsel` value is masked to ensure only the lowest 5 bits are used.
    pub fn select_funcsel(&self, funcsel: u32) {
        const FUNCSEL_MASK: u32 = 0b11111;
        //assert!(funcsel > 0 && funcsel < 10);
        self.ctrl.modify(FUNCSEL_MASK, funcsel);
    }

    /// Toggles output enable - OE for the specified pin.
    ///
    /// # Arguments
    ///
    /// * `toggle` - `true` to enable output, -`false` to disable output.
    pub fn output_enable(&self, toggle: bool) {
        if toggle {
            sio::gpio_oe_set(self.pin);
        } else {
            sio::gpio_oe_clr(self.pin);
        }
    }

    pub fn output_set(&self, toggle: bool) {
        if toggle {
            sio::gpio_out_set(self.pin);
        } else {
            sio::gpio_out_clr(self.pin);
        }
    }

    pub fn input_high(&self) -> bool {
        return sio::gpio_input_value() & (1 << self.pin) != 0;
    }

    pub fn set_pull(&self, pull: pads_bank::Pull) {
        pads_bank::gpio_pull(self.pin, pull);
    }
}
