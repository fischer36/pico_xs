use crate::interrupt;
use crate::regs::{USBCTRL_BASE, USBCTRL_DPRAM_BASE, USBCTRL_REGS_BASE, USB_DPRAM_SIZE};
use crate::{reset, sio};

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_device_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}
#[repr(C, packed)]
pub struct UsbSetupPacket {
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
}

pub extern "C" fn usb_irq() {
    unsafe {
        crate::FIRED = true;
    }
    // sio::out_clr(25);
    // let setup_packet: *mut UsbSetupPacket = 0x50100000 as *mut u32;
    // let direction: u8 = *setup_packet.bm_request_type;
    // let req: u8 = *setup_packet.b_request;
    // set pid to 1 in ep0 in
    //
}

pub fn init() {
    let usb_reset_bit: u32 = 24;
    // reset::reset_ctrl(usb_reset_bit);
    // reset::unreset_ctrl(usb_reset_bit);

    let usb_irq: u32 = 5;
    interrupt::irq_set_enable(usb_irq);
    mux_usb();
    pwr();
    main_ctrl(0);
    const SIE_CTRL_EP0_INT_1BUF_BITS: u32 = 0x20000000;
    sie_ctrl(SIE_CTRL_EP0_INT_1BUF_BITS);

    usb_interrupt_enable(0);

    // SETUP END POINTS

    let sie_ctrl_offset: u32 = 0x4c;
    let sie_ctrl: *mut u32 = (USBCTRL_REGS_BASE + sie_ctrl_offset) as *mut u32;
    unsafe {
        let old = core::ptr::read_volatile(sie_ctrl);

        core::ptr::write_volatile(sie_ctrl, old | (1 << 16));
    }
    // clear dpram memory
    //
}

// WORKS!
pub fn mux_usb() {
    let usb_muxing_offset: u32 = 0x74;

    let mask: u32 = 0x00000001 | 0x00000008;
    let usb_muxing: *mut u32 = (USBCTRL_REGS_BASE + usb_muxing_offset) as *mut u32;
    unsafe {
        // let old = core::ptr::read_volatile(usb_muxing);
        core::ptr::write_volatile(usb_muxing, mask);
    }
}
// WORKS!
pub fn pwr() {
    let usb_pwr_offset: u32 = 0x78;
    let mask: u32 = 0x00000004 | 0x00000008;
    let usb_pwr: *mut u32 = (USBCTRL_REGS_BASE + usb_pwr_offset) as *mut u32;
    unsafe {
        // let old = core::ptr::read_volatile(usb_pwr);
        core::ptr::write_volatile(usb_pwr, mask);
    }
}

// WORKS!
pub fn main_ctrl(bit: u32) {
    const CTRL_CONTROLLER_EN_BITS: u32 = 0x00000001;

    let usb_main_ctrl_offset: u32 = 0x40;
    let usb_main_ctrl: *mut u32 = (USBCTRL_REGS_BASE + usb_main_ctrl_offset) as *mut u32;
    unsafe {
        let old = core::ptr::read_volatile(usb_main_ctrl);
        core::ptr::write_volatile(usb_main_ctrl, 1 << 0);
    }
}

// WORKS!
pub fn sie_ctrl(mask: u32) {
    let sie_ctrl_offset: u32 = 0x4c;
    let sie_ctrl: *mut u32 = (USBCTRL_REGS_BASE + sie_ctrl_offset) as *mut u32;
    unsafe {
        let old = core::ptr::read_volatile(sie_ctrl);
        const EP0_INT_1BUF: u32 = 29;

        core::ptr::write_volatile(sie_ctrl, 1 << EP0_INT_1BUF);
    }
}

// WORKS!
pub fn usb_interrupt_enable(bit: u32) {
    let mask = (1 << 4) | (1 << 12) | (1 << 16); // buff status, buf reset, but setup req, respectively
    let usb_inte: *mut u32 = (USBCTRL_REGS_BASE + 0x90) as *mut u32;

    unsafe {
        // let old = core::ptr::read_volatile(usb_inte);
        core::ptr::write_volatile(usb_inte, mask);
    }
}
pub fn configure_endpoins() {
    let ep_enable_mask = 1 << 31;
    let ep_bulk_mask = 2 << 26;
    let ep_interrupt_mask = 3 << 26;
    let ep_type_clear_mask = 3 << 26;

    unsafe {
        let ep_in_ctrl_data_buffer_1: &mut [u8] = &mut *((USBCTRL_BASE + 180) as *mut [u8; 64]);
        let ep_in_ctrl_data_offset_1 = hw_data_offset(ep_in_ctrl_data_buffer_1.as_ptr());
        let ep_in_ctrl_1: *mut u32 = (USBCTRL_BASE + 0x8) as *mut u32;
        let ep_in_buffer_ctrl_1: *mut u32 = (USBCTRL_BASE + 0x88) as *mut u32;
        let mut old = ep_in_ctrl_1.read_volatile();
        old &= !ep_type_clear_mask;
        ep_in_ctrl_1
            .write_volatile(old | ep_enable_mask | ep_interrupt_mask | ep_in_ctrl_data_offset_1);
    }

    unsafe {
        let ep_out_ctrl_data_buffer_1: &mut [u8] = &mut *((USBCTRL_BASE + 244) as *mut [u8; 64]);
        let ep_out_ctrl_data_offset_1 = hw_data_offset(ep_out_ctrl_data_buffer_1.as_ptr());
        let ep_out_ctrl_1: *mut u32 = (USBCTRL_BASE + 0xC) as *mut u32;
        let ep_out_buffer_ctrl_1: *mut u32 = (USBCTRL_BASE + 0x8C) as *mut u32;
        let mut old = ep_out_ctrl_1.read_volatile();
        old &= !ep_type_clear_mask;
        ep_out_ctrl_1
            .write_volatile(old | ep_enable_mask | ep_bulk_mask | ep_out_ctrl_data_offset_1);
    }
    unsafe {
        let ep_in_ctrl_data_buffer_2: &mut [u8] = &mut *((USBCTRL_BASE + 328) as *mut [u8; 64]);
        let ep_in_ctrl_data_offset_2 = hw_data_offset(ep_in_ctrl_data_buffer_2.as_ptr());
        let ep_in_ctrl_2: *mut u32 = (USBCTRL_BASE + 0x10) as *mut u32;
        let ep_in_buffer_ctrl_2: *mut u32 = (USBCTRL_BASE + 0x90) as *mut u32;
        let mut old = ep_in_ctrl_2.read_volatile();
        old &= !ep_type_clear_mask;
        ep_in_ctrl_2.write_volatile(old | ep_enable_mask | ep_bulk_mask | ep_in_ctrl_data_offset_2);
    }
}

pub fn hw_data_offset(buff: *const u8) -> u32 {
    // Remove usb base from buffer pointer
    return (buff as u32) ^ (0x50100000 as u32);
}
