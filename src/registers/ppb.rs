// PPB_BASE / MO_PLUS
const BASE: u32 = 0xe0000000;
// Interrupt set enable
pub const NVIC_ISER: *mut u32 = (BASE + 0x100) as *mut u32;
// Interrupt clear enable
pub const NVIC_ICER: *mut u32 = (BASE + 0x180) as *mut u32;
// Interrupt set pending
pub const NVIC_ISPR: *mut u32 = (BASE + 0x200) as *mut u32;
// Interrupt clear pending
pub const NVIC_ICPR: *mut u32 = (BASE + 0x280) as *mut u32;
