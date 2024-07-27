/// THe Xosc is disabled by default.

const BASE: u32 = 0x40024000;
use crate::xs::Bits;
#[repr(C)]
pub struct Xosc {
    pub ctrl: *mut u32, // 0x00
    //   11:0 freq_range
    //   23:12 enable
    pub status: *mut u32, // 0x04
    //   12 enabled
    //   24 badwrite
    //   31 stable
    pub doormant: *mut u32, // 0x8
    //   31:0
    pub startup: *mut u32, // 0x0c
    //   13:0 delay
    //   20 x4
    pub count: *mut u32, // 0x1c
                         //   7:0
}

impl Xosc {
    pub fn new() -> Self {
        Self {
            ctrl: BASE as *mut u32,
            status: (BASE + 0x04) as *mut u32,
            doormant: (BASE + 0x08) as *mut u32,
            startup: (BASE + 0x0c) as *mut u32,
            count: (BASE + 0x1c) as *mut u32,
        }
    }
    pub fn ctrl_enable(&self) {
        self.ctrl.clear(0b_1111_1111_1111 << 12);
        self.ctrl.set(0xfab << 12);
        loop {
            unsafe {
                let value = self.status.read_volatile();
                if (value & (1 << 31)) != 0 {
                    break;
                }
            }
        }
    }

    pub fn ctrl_disable(&self) {
        self.ctrl.clear(0b_1111_1111_1111 << 12);
        self.ctrl.set(0xd1e << 12);
    }
    pub fn ctrl_freq_range(&self) {
        // 1_15mhz
        self.ctrl.set(0xaa0);
    }
    pub fn startup_delay(&self) {
        self.startup.set(0xc4);
    }
}
