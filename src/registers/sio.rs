/// SIO_BASE ADDRESS
use crate::xs::Bits;
pub const BASE: u32 = 0xd000_0000;

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

#[repr(C)]
pub struct SioGpioRegisters {
    gpio_out_set: u32,
    gpio_out_clr: u32,
    gpio_out_xor: u32,
    gpio_oe: u32,
    gpio_oe_set: u32,
    gpio_oe_clr: u32,
}

static mut SIO_GPIO_REGISTERS: *mut SioGpioRegisters = core::ptr::null_mut();
pub fn out_set_asm(gpio: u32) {
    unsafe {
        let gpio_out_set_addr = BASE + 0x14;
        let bit_mask = 1 << gpio;
        core::arch::asm!(
            "str {value}, [{addr}]", // Store value (bitmask) in addr (out_set)
            value = in(reg) bit_mask, // tells the compiler to pass rust variable into asm.
            addr = in(reg) gpio_out_set_addr, // here aswell
            options(nostack) // tells compiler that it is stack-less
        );
    }
}
pub fn out_set(gpio: u32) {
    let gpio_out_set_addr: *mut u32 = (BASE + 0x14) as *mut u32;
    gpio_out_set_addr.set(1 << gpio);
    //unsafe {
    //    let sio_gpio_regs = &mut *((BASE + 0x014) as *mut SioGpioRegisters);
    //    sio_gpio_regs.gpio_out_set |= 1 << gpio;
    //}
}

pub fn out_clr(gpio: u32) {
    let gpio_out_clr_addr: *mut u32 = (BASE + 0x18) as *mut u32;
    gpio_out_clr_addr.set(1 << gpio);
}
