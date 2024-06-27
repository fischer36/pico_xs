pub fn clear_bits(register: *mut u32, mask: u32) {
    unsafe {
        let old = register.read_volatile();
        register.write_volatile(old & !mask);
    }
}

pub fn set_bits(register: *mut u32, mask: u32) {
    unsafe {
        let old = register.read_volatile();
        register.write_volatile(old | mask);
    }
}

pub fn create_bitmask(start_bit: u32, end_bit: u32) -> u32 {
    if start_bit > end_bit || end_bit >= 32 {
        panic!("Invalid bit range or bit range out of bounds");
    }
    let length = end_bit - start_bit + 1;
    let mask = (1 << length) - 1;
    mask << start_bit
}
