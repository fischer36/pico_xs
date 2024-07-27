/// SIO_BASE ADDRESS
use crate::xs::Bits;
pub const BASE: u32 = 0xd000_0000;

/// # Raw addresses and methods to interact with SIO gpio registers.
///
/// ## Adresses:
/// GPIO OUTPUT VALUE
pub const GPIO_OUT: *mut u32 = (BASE + 0x10) as *mut u32;
/// GPIO OUTPUT VALUE SET
pub const GPIO_OUT_SET: *mut u32 = (BASE + 0x14) as *mut u32;
/// GPIO OUTPUT VALUE CLEAR
pub const GPIO_OUT_CLR: *mut u32 = (BASE + 0x18) as *mut u32;
/// GPIO OUTPUT VALUE XOR
pub const GPIO_OUT_XOR: *mut u32 = (BASE + 0x1C) as *mut u32;
/// GPIO OUTPUT ENABLE
pub const GPIO_OE: *mut u32 = (BASE + 0x20) as *mut u32;
/// GPIO OUTPUT ENABLE SET
pub const GPIO_OE_SET: *mut u32 = (BASE + 0x24) as *mut u32;
/// GPIO OUTPUT ENABLE CLEAR
pub const GPIO_OE_CLR: *mut u32 = (BASE + 0x28) as *mut u32;
/// GPIO OUTPUT ENABLE XOR
pub const GPIO_OE_XOR: *mut u32 = (BASE + 0x2C) as *mut u32;
/// Methods
///
pub fn gpio_oe_set(gpio: u32) {
    GPIO_OE_SET.set(1 << gpio);
}
pub fn gpio_oe_clr(gpio: u32) {
    GPIO_OE_CLR.clear(1 << gpio);
}
pub fn gpio_oe_xor(gpio: u32) {
    GPIO_OE_XOR.xor(1 << gpio);
}
pub fn gpio_out_set(gpio: u32) {
    GPIO_OUT_SET.set(1 << gpio);
}
pub fn gpio_out_clr(gpio: u32) {
    GPIO_OUT_CLR.clear(1 << gpio);
}
pub fn gpio_out_xor(gpio: u32) {
    GPIO_OUT_XOR.xor(1 << gpio);
}

/// Struct for GPIO_OUT and GPIO_OE, because they share register-memory structure and methods.
pub struct OutputSet {
    gpio: u32,
    base_address: *mut u32,
    set: *mut u32,
    clear: *mut u32,
    xor: *mut u32,
}

impl OutputSet {
    pub fn new(base_address: u32, gpio_pin: u32) -> Self {
        Self {
            gpio: gpio_pin,
            base_address: base_address as *mut u32,
            set: (base_address + 0x04) as *mut u32,
            clear: (base_address + 0x08) as *mut u32,
            xor: (base_address + 0x0c) as *mut u32,
        }
    }

    pub fn set(&mut self) {
        self.set.set(1 << self.gpio);
    }

    pub fn clr(&mut self) {
        self.clear.set(1 << self.gpio);
    }

    pub fn xor(&mut self) {
        self.xor.xor(1 << self.gpio);
    }
}
// Temp function
pub fn gpio_input_value() -> u32 {
    // GPIO_IN Register 29:0 where each bit represents the input value of the corresponding GPIO pin
    //
    const GPIO_IN: *mut u32 = (BASE + 0x004) as *mut u32;
    let value: u32;
    unsafe {
        value = GPIO_IN.read_volatile();
    }
    value
}
