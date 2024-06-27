#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    control: UnsafeCell<u32>, // Control register
    status: UnsafeCell<u32>,  // Status register
}

impl RegisterBlock {
    pub fn new(base_address: u32) -> &'static mut RegisterBlock {
        unsafe { &mut *(base_address as *mut RegisterBlock) }
    }

    fn control(&self) -> &mut u32 {
        unsafe { &mut *self.control.get() }
    }

    fn status(&self) -> &mut u32 {
        unsafe { &mut *self.control.get() }
    }
}
