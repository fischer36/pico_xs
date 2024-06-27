#include <stdint.h>
#include <string.h>

#define BASE_ADDRESS 0x50110000
#define OFFSET 0x74
#define USB_BUF_CTRL_AVAIL 0x00000400u

// void usb_handle_buff_status() {
//     uint32_t buffers = usb_hw->buf_status;
//     uint32_t remaining_buffers = buffers;
//
//     uint bit = 1u;
//     for (uint i = 0; remaining_buffers && i < USB_NUM_ENDPOINTS * 2; i++) {
//         if (remaining_buffers & bit) {
//             // clear this in advance
//             usb_hw_clear->buf_status = bit;
//             // IN transfer for even i, OUT transfer for odd i
//             usb_handle_buff_done(i >> 1u, !(i & 1u));
//             remaining_buffers &= ~bit;
//         }
//         bit <<= 1u;
//     }
// }
//
//
#define USB_DIR_OUT 0x00u
#define USB_DIR_IN 0x80u
#include <string.h>
void usb_setup_endpoints() {}
void usb_device_init() {
  memset((uint32_t *)(0x50100000), 0, 4096);
  // uint32_t* resets_reset = (uint32_t*)(0x4000c000);
  // *resets_reset|=(1<<24)|(1<<5)|(1<<8);
  // reset_block(RESETS_RESET_USBCTRL_BITS);

  // *resets_reset &= !(1<<24);
  // *resets_reset|=(1<<24);
  // reset_block(RESETS_RESET_USBCTRL_BITS);

  // *resets_reset &= ~(1<<24);

  // *resets_reset&=~(1<<24);
  // *resets_reset&=~(1<<5);
  // *resets_reset&=~(1<<8);
  // unreset_block_wait(RESETS_RESET_USBCTRL_BITS);

  // Clear any previous state in dpram just in case
  // memset(usb_dpram, 0, sizeof(*usb_dpram)); // <1>

  // Enable USB interrupt at processor
  // irq_set_enabled(USBCTRL_IRQ, true);
  uint32_t *set_enabled = (uint32_t *)(0xe0000000 + 0xe100);
  *set_enabled |= (1 << 5);
  // Mux the controller to the onboard usb phy
  // usb_hw->muxing = USB_USB_MUXING_TO_PHY_BITS | USB_USB_MUXING_SOFTCON_BITS;

  uint32_t *muxer = (uint32_t *)(BASE_ADDRESS + OFFSET);
  *muxer = 0x00000001 | 0x00000008;
  // Force VBUS detect so the device thinks it is plugged into a host
  // usb_hw->pwr = USB_USB_PWR_VBUS_DETECT_BITS |
  // USB_USB_PWR_VBUS_DETECT_OVERRIDE_EN_BITS;

  uint32_t *pwr = (uint32_t *)(BASE_ADDRESS + 0x78);
  *pwr = 0x00000004 | 0x00000008;
  // Enable the USB controller in device mode.
  // 0x00000040

  uint32_t *mainctrl = (uint32_t *)(BASE_ADDRESS + 0x40);
  *mainctrl = (1 << 0);

  // Enable an interrupt per EP0 transaction
  uint32_t *siectrl = (uint32_t *)(BASE_ADDRESS + 0x4c);
  *siectrl |= (1 << 29) | (1 << 27);
  // usb_hw->sie_ctrl = ; // <2>

  // Enable interrupts for when a buffer is done, when the bus is reset,
  // and when a setup packet is received
  uint32_t *inte = (uint32_t *)(BASE_ADDRESS + 0x90);
  *inte |= (1 << 4) | (1 << 3) | (1 << 12) | (1 << 16);

  // Set up endpoints (endpoint control registers)
  // described by device configuration
  usb_setup_endpoints();

  // Present full speed device by enabling pull up on DP
  *siectrl |= 1 << 16;
}

struct usb_setup_packet {
  uint8_t bmRequestType;
  uint8_t bRequest;
  uint16_t wValue;
  uint16_t wIndex;
  uint16_t wLength;
} __packed;

struct usb_descriptor {
  uint8_t bLength;
  uint8_t bDescriptorType;
};
//
