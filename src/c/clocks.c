// Copyright (c) 2023 CarlosFTM
// This code is licensed under MIT license (see LICENSE.txt for details)

#define PUT32(address, value) (*((volatile unsigned int *)address)) = value
#define GET32(address) *(volatile unsigned int *)address

#define XOR (0x1000)
#define SET (0x2000)
#define CLR (0x3000)

// Resets
#define RESETS_BASE 0x4000C000UL
#define RESETS_RESET (RESETS_BASE + 0x00)
#define RESETS_RESET_DONE (RESETS_BASE + 0x08)

// IO Bank
#define IO_BANK0_BASE 0x40014000UL
#define IO_BANK0_GPIO00_CTRL (IO_BANK0_BASE + 0x04)
#define IO_BANK0_GPIO01_CTRL (IO_BANK0_BASE + 0x0C)
#define IO_BANK0_GPIO25_CTRL (IO_BANK0_BASE + 0xCC)

// SIO
#define SIO_BASE 0xD0000000UL
#define SIO_GPIO_OUT_XOR (SIO_BASE + 0x1c)
#define SIO_GPIO_OE (SIO_BASE + 0x20)
#define SIO_GPIO_OE_SET (SIO_BASE + 0x24)

// XOSC ( 12MHz on-board crystal oscillator )
#define XOSC_BASE 0x40024000UL
#define XOSC_CTRL (XOSC_BASE + 0x00)
#define XOSC_STATUS (XOSC_BASE + 0x04)
#define XOSC_STARTUP (XOSC_BASE + 0x0C)

// Clocks
#define CLOCKS_BASE 0x40008000UL
#define CLK_REF_CTRL (CLOCKS_BASE + 0x30)
#define CLK_REF_DIV (CLOCKS_BASE + 0x34)
#define CLK_SYS_CTRL (CLOCKS_BASE + 0x3C)
#define CLK_PERI_CTRL (CLOCKS_BASE + 0x48)

#define CLK_SYS_CTRL (CLOCKS_BASE + 0x54)
#define CLK_SYS_DIV (CLOCKS_BASE + 0x58)

#define CORTEX_BASE 0xe0000000UL
#define CORTEX_SYST_CSR (CORTEX_BASE + 0xe010)
#define CORTEX_SYST_RVR (CORTEX_BASE + 0xe014)
#define CORTEX_SYST_CVR (CORTEX_BASE + 0xe018)

/* Setup XOSC and set it a source clock */
static void setupClocks(void) {
  // Enable the XOSC
  PUT32(XOSC_CTRL, 0xAA0);            // Frequency range: 1_15MHZ
  PUT32(XOSC_STARTUP, 0xc4);          // Startup delay ( default value )
  PUT32((XOSC_CTRL | SET), 0xFAB000); // Enable ( magic word )
  while (!(GET32(XOSC_STATUS) & 0x80000000))
    ; // Oscillator is running and stable

  // Set the XOSC as source clock for REF, SYS and Periferals
  PUT32(CLK_REF_CTRL, 2);       // CLK REF source = xosc_clksrc
                                //
  PUT32(CLK_SYS_CTRL, 0);       // CLK SYS source = clk_ref
  PUT32(CLK_USB_CTRL, 0);       // CLK USB source = clk_ref
                                //
  PUT32(CLK_REF_DIV, (1 << 8)); // CLK REF Divisor = 1
  PUT32(CLK_USB_DIV, (1 << 8)); // CLK USB Divisor = 1
                                //
  PUT32(CLK_USB_CTRL,
        (1 << 11) | (4 << 5)); // CLK PERI Enable & AUX SRC = xosc_clksrc
  PUT32(CLK_PERI_CTRL,
        (1 << 11) | (4 << 5)); // CLK PERI Enable & AUX SRC = xosc_clksrc
}

/* reset the subsystems used in this program */
static void resetSubsys() {
  // Reset IO Bank
  PUT32((RESETS_RESET | CLR), (1 << 5) | (1 << 24));
  while (GET32(RESETS_RESET_DONE) & (1 << 5) == 0)
    ;
}

/* ***********************************************
 * Main function
 * ********************************************* */
__attribute__((used, section(".boot.entry"))) int main(void) {
  // Setup clocks (XOSC as source clk)
  setupClocks();
  // Reset Subsystems (IO / PADS and UART0)
  resetSubsys();

  // Set GPIO25 as SIO ( F5) and GPIO OE
  PUT32((IO_BANK0_GPIO25_CTRL), 5);
  PUT32(SIO_GPIO_OE_SET, (1 << 25));

#define COUNT_250MS                                                            \
  12000000 / 4 // with XOSC (12MHZ), 12,000,000 ticks = 1 second. -> 250ms =
               // 3,000,000 ticks
  PUT32(CORTEX_SYST_RVR,
        COUNT_250MS); // start counting down from COUNT_250MS (3,000,000 ticks)
  PUT32(CORTEX_SYST_CSR, (1 << 2) | (1 << 0)); // source clock external / enable
  while (1) {
    if (GET32(CORTEX_SYST_CSR) &
        (1 << 16)) // Check for the count flag (count compleated)
    {
      PUT32(SIO_GPIO_OUT_XOR, (1 << 25)); // XOR the LED pin
    }
  }

  return (0);
}
