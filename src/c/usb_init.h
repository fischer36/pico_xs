
#ifndef USB_INIT_H
#define USB_INIT_H

void usb_device_init();
// static const struct usb_device_descriptor device_descriptor = {
//     .bLength = sizeof(struct usb_device_descriptor),
//     .bDescriptorType = 0x01,
//     .bcdUSB = 0x0110,       // USB 1.1 device
//     .bDeviceClass = 0,      // Specified in interface descriptor
//     .bDeviceSubClass = 0,   // No subclass
//     .bDeviceProtocol = 0,   // No protocol
//     .bMaxPacketSize0 = 64,  // Max packet size for ep0
//     .idVendor = 0x0000,     // Your vendor id
//     .idProduct = 0x0001,    // Your product ID
//     .bcdDevice = 0,         // No device revision number
//     .iManufacturer = 1,     // Manufacturer string index
//     .iProduct = 2,          // Product string index
//     .iSerialNumber = 0,     // No serial number
//     .bNumConfigurations = 1 // One configuration
// };

#endif // USB_INIT_H
