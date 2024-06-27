use core::cell::UnsafeCell;

pub trait RegisterBlock {
    fn new() -> &'static mut Self;
}

#[repr(C, align(4))]
pub struct SIO_REGISTER_BLOCK {
    pub CPUID: UnsafeCell<u32>,
    pub GPIO_IN: UnsafeCell<u32>,
    pub GPIO_HI_IN: UnsafeCell<u32>,
    pub GPIO_OUT: UnsafeCell<u32>,
    pub GPIO_OUT_SET: UnsafeCell<u32>,
    pub GPIO_OUT_CLR: UnsafeCell<u32>,
    pub GPIO_OUT_XOR: UnsafeCell<u32>,
    pub GPIO_OE: UnsafeCell<u32>,
    pub GPIO_OE_SET: UnsafeCell<u32>,
    pub GPIO_OE_CLR: UnsafeCell<u32>,
    pub GPIO_OE_XOR: UnsafeCell<u32>,
}

impl SIO_REGISTER_BLOCK {
    pub fn new(base_address: u32) -> &'static mut SIO_REGISTER_BLOCK {
        unsafe { &mut *(base_address as *mut SIO_REGISTER_BLOCK) }
    }

    pub fn GPIO_OUT_SET(&self) -> *mut u32 {
        unsafe { &mut *self.GPIO_OUT_SET.get() }
    }

    pub fn GPIO_OUT_CLR(&self) -> *mut u32 {
        unsafe { &mut *self.GPIO_OUT_CLR.get() }
    }
    pub fn GPIO_OE(&self) -> *mut u32 {
        unsafe { &mut *self.GPIO_OE.get() }
    }
    pub fn GPIO_OE_SET(&self) -> *mut u32 {
        unsafe { &mut *self.GPIO_OE_SET.get() }
    }
    pub fn GPIO_OE_CLR(&self) -> *mut u32 {
        unsafe { &mut *self.GPIO_OE_CLR.get() }
    }
}
