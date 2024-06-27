#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    pub out_set: UnsafeCell<u32>, // Control register
    pub out_clr: UnsafeCell<u32>, // Status register
}

impl RegisterBlock {
    pub fn new(base_address: u32) -> &'static mut RegisterBlock {
        unsafe { &mut *(base_address as *mut RegisterBlock) }
    }

    pub fn control(&self) -> &mut u32 {
        unsafe { &mut *self.out_set.get() }
    }

    pub fn status(&self) -> &mut u32 {
        unsafe { &mut *self.out_clr.get() }
    }
}
