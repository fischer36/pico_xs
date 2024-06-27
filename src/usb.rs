use core::{marker::PhantomData, ops::Deref, slice};
use vcell::VolatileCell;
const USB_DRAM_ADDR: usize = 0x5010_0000;
const USB_DPRAM_SIZE: usize = 4096;

static mut CONFIGURED: bool = false;
static mut SHOULD_SET_ADDRESS: bool = false;
static mut DEV_ADDR: u8 = 0;

const USB_NUM_ENDPOINTS: usize = 16;
const EP0_BUFF: *mut u8 = (0x5010_0000 + 0x100) as *mut _;

const USB_REQUEST_SET_ADDRESS: u8 = 0x05;
const USB_REQUEST_GET_DESCRIPTION: u8 = 0x06;
const USB_REQUEST_SET_CONFIGURATION: u8 = 0x09;

const USB_DT_DEVICE: u16 = 0x01;
const USB_DT_CONFIG: u16 = 0x02;
const USB_DT_STRING: u16 = 0x03;

const USB_BUF_CTRL_AVAIL: u32 = 0x0400;
const USB_BUF_CTRL_FULL: u32 = 0x8000;
const USB_BUF_CTRL_DATA0_PID: u32 = 0x0000;
const USB_BUF_CTRL_DATA1_PID: u32 = 0x2000;
const USB_BUF_CTRL_LEN_MASK: u32 = 0x3ff;

// DPRAM content for a USB Device
#[repr(C)]
pub struct DpramContent {
    setup_packet: [VolatileCell<u8>; 8],
    ep_ctrl: [EpCtrl; USB_NUM_ENDPOINTS - 1],
    ep_buf_ctrl: [EpCtrl; USB_NUM_ENDPOINTS],

    ep0_buf_a: [VolatileCell<u8>; 0x40],
    _ep0_buf_b: [VolatileCell<u8>; 0x40],

    exp_data: [VolatileCell<u8>; USB_DPRAM_SIZE - 0x180],
}

pub struct UsbDpram {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UsbDpram {}
impl UsbDpram {
    #[inline(always)]
    pub const fn ptr() -> *const DpramContent {
        0x5010_0000 as *const _
    }
}
impl Deref for UsbDpram {
    type Target = DpramContent;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UsbDpram::ptr() }
    }
}

#[repr(C)]
struct EpCtrl {
    in_v: VolatileCell<u32>,
    out: VolatileCell<u32>,
}

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
struct DeviceDescriptor {
    bLength: u8,
    bDescriptorType: u8,
    bcdUSB: u16,
    bDeviceClass: u8,
    bDeviceSubClass: u8,
    bDeviceProtocol: u8,
    bMaxPacketSize0: u8,
    idVendor: u16,
    idProduct: u16,
    bcdDevice: u16,
    iManufacturer: u8,
    iProduct: u8,
    iSerialNumber: u8,
    bNumConfigurations: u8,
}

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
struct ConfigDescriptor {
    bLength: u8,
    bDescriptorType: u8,
    wTotalLength: u16,
    bNumInterfaces: u8,
    bConfigurationValue: u8,
    iConfiguration: u8,
    bmAttributes: u8,
    bMaxPower: u8,
}

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
struct InterfaceDescriptor {
    bLength: u8,
    bDescriptorType: u8,
    bInterfaceNumber: u8,
    bAlternateSetting: u8,
    bNumEndpoints: u8,
    bInterfaceClass: u8,
    bInterfaceSubClass: u8,
    bInterfaceProtocol: u8,
    iInterface: u8,
}

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
struct EndPointDescriptor {
    bLength: u8,
    bDescriptorType: u8,
    bEndpointAddress: u8,
    bmAttributes: u8,
    wMaxPacketSize: u16,
    bInterval: u8,
}

struct EndpointConfig {
    descriptor: &'static EndPointDescriptor,

    handler: fn(&mut UsbDevice, &[VolatileCell<u8>], u32),
    endpoint_control: Option<*const VolatileCell<u32>>,
    buffer_control: *const VolatileCell<u32>,
    data_buffer: &'static [VolatileCell<u8>],

    next_pid: u8,
}

impl EndpointConfig {
    fn is_tx(&self) -> bool {
        (self.descriptor.bEndpointAddress & USB_DIR_IN) == USB_DIR_IN
    }
}

struct UsbDeviceConfiguration {
    endpoints: [Option<EndpointConfig>; USB_NUM_ENDPOINTS],
}

// Descriptors
static USB_DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    bLength: core::mem::size_of::<DeviceDescriptor>() as u8,
    bDescriptorType: 1,    // USB_DT_DEVICE
    bcdUSB: 0x0110,        // USB 1.1 device
    bDeviceClass: 0,       // Specified in interface descriptor
    bDeviceSubClass: 0,    // No subclass
    bDeviceProtocol: 0,    // No protocol
    bMaxPacketSize0: 64,   // Max packet size for ep0
    idVendor: 0x0000,      // Your vendor id
    idProduct: 0x0001,     // Your product ID
    bcdDevice: 0,          // No device revision number
    iManufacturer: 1,      // Manufacturer string index
    iProduct: 2,           // Product string index
    iSerialNumber: 0,      // No serial number
    bNumConfigurations: 1, // One configuration
};

static USB_INTERFACE_DESCRIPTOR: InterfaceDescriptor = InterfaceDescriptor {
    bLength: core::mem::size_of::<InterfaceDescriptor>() as u8,
    bDescriptorType: 0x04, // USB_DT_INTERFACE
    bInterfaceNumber: 0,
    bAlternateSetting: 0,
    bNumEndpoints: 1,      // Interface has 2 endpoints
    bInterfaceClass: 0xff, // Vendor specific endpoint
    bInterfaceSubClass: 0,
    bInterfaceProtocol: 0,
    iInterface: 0,
};

static USB_CONFIG_DESCRIPTOR: ConfigDescriptor = ConfigDescriptor {
    bLength: core::mem::size_of::<ConfigDescriptor>() as u8,
    bDescriptorType: 2, // USB_DT_CONFIG,
    wTotalLength: (core::mem::size_of::<ConfigDescriptor>()
        + core::mem::size_of::<InterfaceDescriptor>()
        + core::mem::size_of::<EndPointDescriptor>()
        + core::mem::size_of::<EndPointDescriptor>()) as u16,
    bNumInterfaces: 1,
    bConfigurationValue: 1, // Configuration 1
    iConfiguration: 0,      // No string
    bmAttributes: 0xc0,     // attributes: self powered, no remote wakeup
    bMaxPower: 0x32,        // 100ma
};

// Intentionally empty
fn ep0_out_handler(_usb: &mut UsbDevice, _buff: &[VolatileCell<u8>], _len: u32) {}

const ADDR_ENDP: *mut u32 = (0x50110000) as *mut u32;

fn ep0_in_handler(usb: &mut UsbDevice, _buff: &[VolatileCell<u8>], _len: u32) {
    unsafe {
        if SHOULD_SET_ADDRESS {
            crate::modify_register(ADDR_ENDP, crate::create_bitmask(0, 6), DEV_ADDR as u32);
            SHOULD_SET_ADDRESS = false;
        } else {
            affirm(usb.ep0_out_ref);
        }
    }
}

const LANG_DESCRIPTOR: [u8; 4] = [4, 0x03, 0x09, 0x04];

const DESCRIPTOR_STRING: &[&[u8]] = &[
    "Pico Hal XS".as_bytes(),
    "Pico Test Device in Rust!".as_bytes(),
];

const USB_DIR_OUT: u8 = 0;
const USB_DIR_IN: u8 = 0x80;

const USB_TRANSFER_TYPE_CONTROL: u8 = 0;
const USB_TRANSFER_TYPE_BULK: u8 = 2;

const USB_DT_ENDPOINT: u8 = 5;

const EP0_OUT_ADDR: u8 = USB_DIR_OUT | 0;
const EP0_IN_ADDR: u8 = USB_DIR_IN | 0;

static EP0_OUT: EndPointDescriptor = EndPointDescriptor {
    bLength: core::mem::size_of::<EndPointDescriptor>() as u8,
    bDescriptorType: USB_DT_ENDPOINT,
    bEndpointAddress: EP0_OUT_ADDR, // EP number 1, OUT from host (rx to device)
    bmAttributes: USB_TRANSFER_TYPE_CONTROL,
    wMaxPacketSize: 64,
    bInterval: 0,
};

static EP0_IN: EndPointDescriptor = EndPointDescriptor {
    bLength: core::mem::size_of::<EndPointDescriptor>() as u8,
    bDescriptorType: USB_DT_ENDPOINT,
    bEndpointAddress: EP0_IN_ADDR, // EP number 1, OUT from host (rx to device)
    bmAttributes: USB_TRANSFER_TYPE_CONTROL,
    wMaxPacketSize: 64,
    bInterval: 0,
};

pub struct UsbDevice {
    config: UsbDeviceConfiguration,
    ep0_in_ref: *mut EndpointConfig,
    ep0_out_ref: *mut EndpointConfig,
}

impl UsbDevice {
    fn new(config: UsbDeviceConfiguration) -> Self {
        let a = core::ptr::from_ref(config.endpoints[0].as_ref().unwrap()) as *mut EndpointConfig;
        let b = core::ptr::from_ref(config.endpoints[1].as_ref().unwrap()) as *mut EndpointConfig;
        Self {
            config,
            ep0_in_ref: b,
            ep0_out_ref: a,
        }
    }

    // Run USB
    pub fn poll(&mut self) {
        const SIE_STATUS: *mut u32 = (0x50110000 + 0x50) as *mut u32;
        const BUFF_STATUS: *mut u32 = (0x50110000 + 0x58) as *mut u32;
        const USB_INTS: *mut u32 = (0x50110000 + 0x98) as *mut u32;
        const USBCTRL_REGS_BASE: u32 = 0x50110000;
        unsafe {
            let info: u32 = core::ptr::read_volatile(USB_INTS);
            if info & (1 << 16) != 0 {
                crate::set_bits(SIE_STATUS, 1 << 17);
                self.handle_setup_packet()
            } else if info & (1 << 12) != 0 {
                crate::set_bits(SIE_STATUS, 1 << 19);
                DEV_ADDR = 0;
                SHOULD_SET_ADDRESS = false;
                CONFIGURED = false;
                ADDR_ENDP.write_volatile(0);
            } else if info & (1 << 4) != 0 {
                let buffers = core::ptr::read_volatile(BUFF_STATUS);
                const EP0_OUT_BIT: u32 = 1 << 1; // bit position for EP0 OUT
                const EP0_IN_BIT: u32 = 1 << 0; // bit position for EP0 IN
                if buffers & EP0_OUT_BIT != 0 {
                    crate::clr_bits(BUFF_STATUS, 1 << 1);
                }
                if buffers & EP0_IN_BIT != 0 {
                    const BUFF_STATUS: *mut u32 = (0x50110000 + 0x58) as *mut u32;
                    crate::clr_bits(BUFF_STATUS, 1 << 0);
                    if SHOULD_SET_ADDRESS {
                        crate::modify_register(
                            ADDR_ENDP,
                            crate::create_bitmask(0, 6),
                            DEV_ADDR as u32,
                        );
                        SHOULD_SET_ADDRESS = false;
                    } else {
                        affirm(self.ep0_out_ref);
                    }
                }
            }
        }
    }
    fn get_endpoint_configuration_mut(
        &mut self,
        endpoint_address: u8,
    ) -> Option<&mut EndpointConfig> {
        self.config
            .endpoints
            .iter_mut()
            .filter_map(|c| c.as_mut())
            .find(|cfg| cfg.descriptor.bEndpointAddress == endpoint_address)
    }

    pub fn configured(&self) -> bool {
        unsafe { CONFIGURED }
    }

    fn handle_setup_packet(&mut self) {
        let setup_packet = unsafe { UsbSetupPacket::from_raw(0x50100000 as *mut u8) };
        let ep0_in_config = self.config.endpoints[1].as_mut().unwrap();
        unsafe {
            EP0_IN_NEXT_PID = 1;
        }

        let req_direction = setup_packet.bmRequestType;
        let req = setup_packet.bRequest;

        match req_direction {
            USB_DIR_OUT => match req {
                USB_REQUEST_SET_ADDRESS => unsafe {
                    DEV_ADDR = (setup_packet.wValue & 0xff) as u8;
                    SHOULD_SET_ADDRESS = true;
                    let mut val = USB_BUF_CTRL_AVAIL | USB_BUF_CTRL_FULL;
                    val |= if EP0_IN_NEXT_PID == 1 {
                        USB_BUF_CTRL_DATA1_PID
                    } else {
                        USB_BUF_CTRL_DATA0_PID
                    };

                    EP0_IN_NEXT_PID ^= 1;
                    EP0_IN_BUFFER_CTRL.write_volatile(val);
                },
                USB_REQUEST_SET_CONFIGURATION => unsafe {
                    let mut val = USB_BUF_CTRL_AVAIL | USB_BUF_CTRL_FULL;
                    val |= if EP0_IN_NEXT_PID == 1 {
                        USB_BUF_CTRL_DATA1_PID
                    } else {
                        USB_BUF_CTRL_DATA0_PID
                    };

                    EP0_IN_NEXT_PID ^= 1;
                    EP0_IN_BUFFER_CTRL.write_volatile(val);
                    CONFIGURED = true;
                },
                _ => (), // Other request
            },
            USB_DIR_IN => {
                if req == USB_REQUEST_GET_DESCRIPTION {
                    let descriptor_type = setup_packet.wValue >> 8;
                    match descriptor_type {
                        USB_DT_DEVICE => {
                            let device_descriptor =
                                &USB_DEVICE_DESCRIPTOR as *const DeviceDescriptor;
                            let ep = self.config.endpoints[1].as_mut().unwrap();
                            unsafe {
                                EP0_IN_NEXT_PID = 1;
                            }
                            let data = unsafe {
                                slice::from_raw_parts(
                                    (device_descriptor) as *const u8,
                                    core::mem::size_of::<DeviceDescriptor>(),
                                )
                            };
                            let len = data.len();
                            let data = Some(data);
                            //self.start_transfer(EP0_IN_ADDR, data.len() as u32, Some(data));
                            unsafe {
                                let mut val = len as u32 | USB_BUF_CTRL_AVAIL;

                                let ep_config =
                                    self.get_endpoint_configuration_mut(EP0_IN_ADDR).unwrap();
                                let buff_ptr: &mut [u8] = slice::from_raw_parts_mut(EP0_BUFF, 0x40);

                                if ep_config.is_tx() {
                                    if let Some(data) = data {
                                        // Copy data into USB memory

                                        for (index, byte) in data.iter().enumerate() {
                                            buff_ptr[index] = *byte;
                                            //ep_config.data_buffer[index].set(*byte);
                                        }
                                    } else {
                                        assert!(len == 0);
                                    }

                                    // Mark buffer as full
                                    val |= USB_BUF_CTRL_FULL;
                                }

                                val |= if EP0_IN_NEXT_PID == 1 {
                                    USB_BUF_CTRL_DATA1_PID
                                } else {
                                    USB_BUF_CTRL_DATA0_PID
                                };

                                EP0_IN_NEXT_PID ^= 1;
                                //ep_config.next_pid ^= 1;

                                EP0_IN_BUFFER_CTRL.write_volatile(val);
                            }
                        }
                        USB_DT_CONFIG => {
                            let config_descriptor =
                                &USB_CONFIG_DESCRIPTOR as *const ConfigDescriptor;
                            let mut buff = [0u8; 64];
                            let mut buf_index = 0;
                            let data = unsafe {
                                slice::from_raw_parts(
                                    (config_descriptor) as *const u8,
                                    core::mem::size_of::<ConfigDescriptor>(),
                                )
                            };
                            let data_len = data.len();
                            buff[..data_len].copy_from_slice(data);
                            buf_index += data_len;
                            unsafe {
                                if setup_packet.wLength >= (*config_descriptor).wTotalLength {
                                    let interface_descriptor =
                                        &USB_INTERFACE_DESCRIPTOR as *const InterfaceDescriptor;
                                    let data = unsafe {
                                        slice::from_raw_parts(
                                            (interface_descriptor) as *const u8,
                                            core::mem::size_of::<InterfaceDescriptor>(),
                                        )
                                    };
                                    let data_len = data.len();
                                    buff[buf_index..buf_index + data_len].copy_from_slice(data);
                                    buf_index += data_len;
                                    for i in 2..USB_NUM_ENDPOINTS {
                                        if let Some(endpoint) = &self.config.endpoints[i] {
                                            let endpoint_descriptor = endpoint.descriptor;
                                            let data = unsafe {
                                                slice::from_raw_parts(
                                                    (endpoint_descriptor
                                                        as *const EndPointDescriptor)
                                                        as *const u8,
                                                    core::mem::size_of::<EndPointDescriptor>(),
                                                )
                                            };
                                            let data_len = data.len();
                                            buff[buf_index..buf_index + data_len]
                                                .copy_from_slice(data);
                                            buf_index += data_len;
                                        }
                                    }
                                }
                            }

                            let len = buf_index;
                            let data = Some(&buff[..len]);
                            //start_transferxd(self.ep0_in_ref, data_len as u32, Some(&buff[..data_len]));
                            unsafe {
                                let mut val = len as u32 | USB_BUF_CTRL_AVAIL;

                                let ep_config =
                                    self.get_endpoint_configuration_mut(EP0_IN_ADDR).unwrap();
                                let buff_ptr: &mut [u8] = slice::from_raw_parts_mut(EP0_BUFF, 0x40);
                                if ep_config.is_tx() {
                                    if let Some(data) = data {
                                        // Copy data into USB memory

                                        for (index, byte) in data.iter().enumerate() {
                                            buff_ptr[index] = *byte;
                                            //ep_config.data_buffer[index].set(*byte);
                                        }
                                    } else {
                                        assert!(len == 0);
                                    }

                                    // Mark buffer as full
                                    val |= USB_BUF_CTRL_FULL;
                                }

                                val |= if EP0_IN_NEXT_PID == 1 {
                                    USB_BUF_CTRL_DATA1_PID
                                } else {
                                    USB_BUF_CTRL_DATA0_PID
                                };

                                EP0_IN_NEXT_PID ^= 1;
                                //ep_config.next_pid ^= 1;

                                EP0_IN_BUFFER_CTRL.write_volatile(val);
                            }
                        }
                        USB_DT_STRING => {
                            let i = setup_packet.wValue & 0xff;
                            let mut ep_buffer = [0u8; 64];
                            let mut len;
                            if i == 0 {
                                ep_buffer[..4].copy_from_slice(&LANG_DESCRIPTOR[..4]);
                                len = 4;
                            } else if i == 1 {
                                len = usb_prepare_string_descriptor(
                                    &mut ep_buffer,
                                    DESCRIPTOR_STRING[0],
                                )
                            } else {
                                len = usb_prepare_string_descriptor2(
                                    &mut ep_buffer,
                                    DESCRIPTOR_STRING[1],
                                )
                            };
                            unsafe {
                                let mut val = len as u32 | USB_BUF_CTRL_AVAIL;
                                let data = Some(&ep_buffer[..len]);
                                let ep_config =
                                    self.get_endpoint_configuration_mut(EP0_IN_ADDR).unwrap();
                                let buff_ptr: &mut [u8] = slice::from_raw_parts_mut(EP0_BUFF, 0x40);
                                if ep_config.is_tx() {
                                    if let Some(data) = data {
                                        // Copy data into USB memory

                                        for (index, byte) in data.iter().enumerate() {
                                            buff_ptr[index] = *byte;
                                            //ep_config.data_buffer[index].set(*byte);
                                        }
                                    } else {
                                        assert!(len == 0);
                                    }

                                    // Mark buffer as full
                                    val |= USB_BUF_CTRL_FULL;
                                }

                                val |= if EP0_IN_NEXT_PID == 1 {
                                    USB_BUF_CTRL_DATA1_PID
                                } else {
                                    USB_BUF_CTRL_DATA0_PID
                                };

                                EP0_IN_NEXT_PID ^= 1;
                                //ep_config.next_pid ^= 1;

                                EP0_IN_BUFFER_CTRL.write_volatile(val);
                            }
                        }
                        _ => (), // Ignore other descriptor types
                    }
                }
            }
            _ => (),
        }
    }
}
pub fn affirm(ep: *mut EndpointConfig) {
    let mut val = 0 | USB_BUF_CTRL_AVAIL;
    let endpoint_addr = EP0_OUT_ADDR;
    unsafe {
        val |= if EP0_OUT_NEXT_PID == 1 {
            USB_BUF_CTRL_DATA1_PID
        } else {
            USB_BUF_CTRL_DATA0_PID
        };

        EP0_OUT_NEXT_PID ^= 1;

        EP0_OUT_BUFFER_CTRL.write_volatile(val);
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct UsbSetupPacket {
    bmRequestType: u8,
    bRequest: u8,
    wValue: u16,
    _wIndex: u16,
    wLength: u16,
}

impl UsbSetupPacket {
    unsafe fn from_raw(ptr: *const u8) -> Self {
        use core::ptr;
        Self {
            bmRequestType: ptr::read_volatile(ptr),
            bRequest: ptr::read_volatile(ptr.add(1)),
            wValue: (ptr::read_volatile(ptr.add(2)) as u16)
                | ((ptr::read_volatile(ptr.add(3)) as u16) << 8),
            _wIndex: (ptr::read_volatile(ptr.add(4)) as u16)
                | ((ptr::read_volatile(ptr.add(5)) as u16) << 8),
            wLength: (ptr::read_volatile(ptr.add(6)) as u16)
                | ((ptr::read_volatile(ptr.add(7)) as u16) << 8),
        }
    }
}
static mut EP0_IN_REF: *mut Option<EndpointConfig> = core::ptr::null_mut();
static mut EP0_OUT_REF: *mut Option<EndpointConfig> = core::ptr::null_mut();
static mut EP0_IN_NEXT_PID: u8 = 0;
static mut EP0_OUT_NEXT_PID: u8 = 0;
use crate::regs::RESETS_BASE;
use crate::regs::USBCTRL_REGS_BASE;
pub fn usb_device_init() -> UsbDevice {
    const USBCTRL_DRAM_BASE: *mut u8 = 0x50100000 as *mut u8;
    unsafe {
        let old = core::ptr::read_volatile(RESETS_BASE as *mut u32);
        core::ptr::write_volatile(RESETS_BASE as *mut u32, old | (1 << 24));
        crate::sleep();
        core::ptr::write_volatile(RESETS_BASE as *mut u32, old & !(1 << 24));
    }

    let dpram = UsbDpram {
        _marker: PhantomData,
    };
    //
    //let dpram_start = UsbDpram::ptr() as *mut u8;
    //
    //for offset in 0..USB_DPRAM_SIZE {
    //    unsafe { dpram_start.add(offset).write_volatile(0) };
    //}

    const USB_MUXING: *mut u32 = (USBCTRL_REGS_BASE + 0x74) as *mut u32;
    crate::set_bits(USB_MUXING, (1 << 0) | (1 << 3));
    const USB_PWR: *mut u32 = (USBCTRL_REGS_BASE + 0x78) as *mut u32;
    crate::set_bits(USB_PWR, (1 << 2) | (1 << 3));

    const USB_MAIN_CTRL: *mut u32 = (USBCTRL_REGS_BASE + 0x40) as *mut u32;
    crate::clr_bits(USB_MAIN_CTRL, 1 << 31 | 1 << 1);
    crate::set_bits(USB_MAIN_CTRL, 1 << 0);

    const SIE_CTRL: *mut u32 = (USBCTRL_REGS_BASE + 0x4c) as *mut u32;
    crate::set_bits(SIE_CTRL, 1 << 29);

    const USB_INTE: *mut u32 = (USBCTRL_REGS_BASE + 0x90) as *mut u32;
    crate::set_bits(USB_INTE, 1 << 4 | 1 << 16 | 1 << 12);

    let endpoints = unsafe {
        [
            Some(EndpointConfig {
                descriptor: &EP0_OUT,
                handler: ep0_out_handler,
                endpoint_control: None,
                buffer_control: &dpram.ep_buf_ctrl[0].out as *const _,
                // Data buffer is shared with EP0_IN
                data_buffer: slice::from_raw_parts(dpram.ep0_buf_a.as_ptr(), 0x40),
                next_pid: 0,
            }),
            Some(EndpointConfig {
                descriptor: &EP0_IN,
                handler: ep0_in_handler,
                endpoint_control: None,
                buffer_control: &dpram.ep_buf_ctrl[0].in_v as *const _,
                // Data buffer is shared with EP0_OUT
                data_buffer: slice::from_raw_parts(dpram.ep0_buf_a.as_ptr(), 0x40),
                next_pid: 0,
            }),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ]
    };

    let usb_config: UsbDeviceConfiguration = UsbDeviceConfiguration { endpoints };
    crate::set_bits(SIE_CTRL, 1 << 16);
    UsbDevice::new(usb_config)
}

#[allow(non_snake_case)]
fn usb_prepare_string_descriptor(buffer: &mut [u8], xdescriptor: &[u8]) -> usize {
    let descriptor: &[u8] = "Pico Test Device in Rust!".as_bytes();
    let bLength = 2 + (descriptor.len() * 2);
    let bDescriptorType = 0x03;

    let mut buf_index = 0;

    buffer[buf_index] = bLength as u8;
    buf_index += 1;

    buffer[buf_index] = bDescriptorType;
    buf_index += 1;

    let mut descriptor_index = 0;

    while descriptor_index < descriptor.len() {
        let c = descriptor[descriptor_index];
        descriptor_index += 1;

        buffer[buf_index] = c;
        buf_index += 1;

        buffer[buf_index] = 0;
        buf_index += 1;
    }

    bLength
}

#[allow(non_snake_case)]
fn usb_prepare_string_descriptor2(buffer: &mut [u8], descriptor: &[u8]) -> usize {
    let descriptor: &[u8] = "Pico Hal XS".as_bytes();
    let bLength = 2 + (descriptor.len() * 2);
    let bDescriptorType = 0x03;

    let mut buf_index = 0;

    buffer[buf_index] = bLength as u8;
    buf_index += 1;

    buffer[buf_index] = bDescriptorType;
    buf_index += 1;

    let mut descriptor_index = 0;

    while descriptor_index < descriptor.len() {
        let c = descriptor[descriptor_index];
        descriptor_index += 1;

        buffer[buf_index] = c;
        buf_index += 1;

        buffer[buf_index] = 0;
        buf_index += 1;
    }

    bLength
}

const EP0_OUT_BUFFER_CTRL: *mut u32 = (0x50100000 + 0x84) as *mut u32;
const EP0_IN_BUFFER_CTRL: *mut u32 = (0x50100000 + 0x80) as *mut u32;
