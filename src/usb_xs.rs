#[allow(non_snake_case)]
pub struct SetupPacket {
    pub bmRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub _wIndex: u16,
    pub wLength: u16,
}

impl SetupPacket {
    pub unsafe fn from_raw(ptr: *const u8) -> Self {
        use core::ptr::read_volatile;
        Self {
            bmRequestType: read_volatile(ptr),
            bRequest: read_volatile(ptr.add(1)),
            wValue: (read_volatile(ptr.add(2)) as u16) | ((read_volatile(ptr.add(3)) as u16) << 8),
            _wIndex: (read_volatile(ptr.add(4)) as u16) | ((read_volatile(ptr.add(5)) as u16) << 8),
            wLength: (read_volatile(ptr.add(6)) as u16) | ((read_volatile(ptr.add(7)) as u16) << 8),
        }
    }
}

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
pub struct DeviceDescriptor {
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

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
pub struct ConfigDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub bMaxPower: u8,
}

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
pub struct InterfaceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}

#[repr(C)]
#[repr(packed)]
#[allow(non_snake_case)]
pub struct EndPointDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}
/// Fills buffer with string descriptor
/// Byte 0x8 = total length of buffer (0x8*2+descriptor_length *2)
/// Byte 0x10 = type id (string descriptor = 0x03)
/// The rest of the buffer is filled with the descriptor separated by 0 every other char.
/// (UTF-16 LE encoding)
///                         
/// Example byte[length, type, H, 0, E, 0, L, 0, L, 0, O'] (HELLO)
///
/// Returns the length of the buffer
pub fn copy_string_descriptor(buffer: &mut [u8], descriptor_string: &str) -> usize {
    let descriptor = descriptor_string.as_bytes();
    let b_length = 2 + (descriptor.len() * 2);
    let b_descriptor_type = 0x03;

    let mut buf_index = 0;

    // Set the first byte to length
    buffer[buf_index] = b_length as u8;
    buf_index += 1;

    // Second byte to type 0x3 = string
    buffer[buf_index] = b_descriptor_type;
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

    b_length
}
