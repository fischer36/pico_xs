/*// Copyright (c) 2023 CarlosFTM*/
/*// This code is licensed under MIT license (see LICENSE.txt for details)*/
/*#include "RP2040.h"*/
/**/
/*/* configures UART0 to 9600 8N1*/ * /
    /*void uartConfig(void) {*/
    /*  // Reset UART0*/
    /*  RESETS_CLR->RESET_b.uart0 = 1;*/
    /*  while (RESETS->RESET_DONE_b.uart0 == 0)*/
    /*    ;*/
    /**/
    /*  UART0->UARTIBRD_b.BAUD_DIVINT = 78;*/
    /*  UART0->UARTFBRD_b.BAUD_DIVFRAC = 8;*/
    /*  UART0->UARTLCR_H = (3 << UART0_UARTLCR_H_WLEN_Pos);*/
    /*  UART0->UARTCR = ((1 << UART0_UARTCR_RXE_Pos) | (1 <<
       UART0_UARTCR_TXE_Pos) |*/
    /*                   (1 << UART0_UARTCR_UARTEN_Pos));*/
    /**/
    /*  // Set GPIO0 and 1 to function 2 (UART0)*/
    /*  IO_BANK0->GPIO0_CTRL_b.FUNCSEL = 2;*/
    /*  IO_BANK0->GPIO1_CTRL_b.FUNCSEL = 2;*/
    /**/
    /*  // Interrupt Config: RX Timeout Interrupt + RX interrupt*/
    /*  UART0->UARTIMSC =*/
    /*      ((1 << UART0_UARTIMSC_RTIM_Pos) | (1 << UART0_UARTIMSC_RXIM_Pos));*/
    /*  // Interrupt Config: Receive interrupt FIFO level select = 1/8 Full
       (smallest)*/
    /*  UART0->UARTIFLS_b.RXIFLSEL = 0;*/
    /*}*/
    /**/
    /*/* UART receive character */ * /
    /*unsigned char uartRx(void) {*/
    /*  while (UART0->UARTFR_b.RXFE != 0)*/
    /*    ;                                    // wait for RX FIFO to not be
       empty*/
    /*  return ((char)(UART0->UARTDR_b.DATA)); // Read the RX data*/
    /*}*/
    /**/
    /*/* UART Send single character */ * /
    /*void uartTx(unsigned char x) {*/
    /*  while (UART0->UARTFR_b.TXFF != 0)*/
    /*    ;                       // wait until TX FIFO is not full*/
    /*  UART0->UARTDR_b.DATA = x; // Write the TX data*/
    /*}*/
    /**/
    /*/* UART Send character string */ * /
    /*void uartTxStr(unsigned char *x) {*/
    /*  // Write the string of data until the NULL char is detected*/
    /*  while (*x != '\0') {*/
    /*    uartTx(*x);*/
    /*    *x++;*/
    /*  }*/
    /*}*/
    /**/
    /*unsigned char bin2hex(unsigned char input) {*/
    /*  unsigned char retVal = 'q';*/
    /**/
    /*  if ((input & 0x0F) <= 9) {*/
    /*    retVal = (input & 0x0F) + '0';*/
    /*  } else if ((input & 0x0F) >= 0x0a) {*/
    /*    retVal = ((input & 0x0F) - 10) + 'a';*/
    /*  }*/
    /**/
    /*  return retVal;*/
    /*}*/
    /**/
    /*void uartPrintByte(unsigned char data) {*/
    /*  uartTxStr("[0x");*/
    /*  uartTx(bin2hex(data >> 4));*/
    /*  uartTx(bin2hex(data));*/
    /*  uartTxStr("]\n\r");*/
    /*}*/
    /**/
    /*void resetSubsys(void) {*/
    /*  // Reset IO Bank*/
    /*  RESETS_CLR->RESET_b.io_bank0 = 1;*/
    /*  while (RESETS->RESET_DONE_b.io_bank0 == 0)*/
    /*    ;*/
    /**/
    /*  // Reset PADS BANK*/
    /*  RESETS_CLR->RESET_b.pads_bank0 = 1;*/
    /*  while (RESETS->RESET_DONE_b.pads_bank0 == 0)*/
    /*    ;*/
    /*}*/
    /**/
    /*void setupClocks(void) {*/
    /*  // Enable the XOSC*/
    /*  XOSC->CTRL = 0xAA0;           // Frequency range: 1_15MHZ*/
    /*  XOSC->STARTUP_b.DELAY = 0xC4; // Startup delay ( default value )*/
    /*  XOSC_SET->CTRL = 0xFAB000;    // Enable ( magic word )*/
    /*  while (!(XOSC->STATUS_b.STABLE & 1))*/
    /*    ; // Oscillator is running and stable*/
    /**/
    /*  // Set the XOSC as source clock for REF, SYS and Periferals*/
    /*  CLOCKS->CLK_REF_CTRL_b.SRC = 2;     // CLK REF source = xosc_clksrc*/
    /*  CLOCKS->CLK_SYS_CTRL_b.SRC = 0;     // CLK SYS source = clk_ref*/
    /*  CLOCKS->CLK_REF_DIV_b.INT = 1;      // CLK REF Divisor = 1*/
    /*  CLOCKS->CLK_PERI_CTRL_b.AUXSRC = 4; // CLK PERI AUX SRC = xosc_clksrc*/
    /*  CLOCKS->CLK_PERI_CTRL_b.ENABLE = 1; // CLK PERI Enable*/
    /*}*/
    /**/
    /*void irqUart0(void) {*/
    /*  PPB->NVIC_ICER_b.CLRENA = (1 << UART0_IRQ); // Interrupt disabled*/
    /*  SIO->GPIO_OUT_SET_b.GPIO_OUT_SET = (1 << GPIO_BUILT_IN_LED); // Toggle
       LED*/
    /*  uartTx(uartRx());                            // TX the RX buffer*/
    /*  PPB->NVIC_ICPR_b.CLRPEND = (1 << UART0_IRQ); // Interrupt
       Clear-Pending*/
    /*  PPB->NVIC_ISER_b.SETENA = (1 << UART0_IRQ);  // Interrupt enable*/
    /*}*/
