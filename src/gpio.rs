use crate::registers::{io_bank, sio};

use crate::xs::Bits;

pub struct Gpio {
    pin: u32,
    status: *mut u32,
    ctrl: *mut u32,
    pub out: sio::OutputSet,
    pub oe: sio::OutputSet,
}

impl Gpio {
    pub fn new(gpio: u32) -> Self {
        assert!(gpio < 30);
        Self {
            pin: gpio,
            status: (io_bank::BASE + 0x0 + 0x8 * gpio) as *mut u32,
            ctrl: (io_bank::BASE + 0x4 + 0x8 * gpio) as *mut u32,
            out: sio::OutputSet::new(sio::BASE + 0x010, gpio),
            oe: sio::OutputSet::new(sio::BASE + 0x020, gpio),
        }
    }
    pub fn select_funcsel(&self, funcsel: u32) {
        const FUNCSEL_MASK: u32 = 0b11111;
        assert!(funcsel > 0 && funcsel < 10);
        self.ctrl.modify(FUNCSEL_MASK, funcsel);
    }
}
