/* Borrowed from https://github.com/rust-embedded/cortex-m/tree/master/cortex-m-rt */

MEMORY {
    BOOT_LOADER : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

EXTERN(BOOT2_FIRMWARE)
SECTIONS {
  .boot_loader ORIGIN(BOOT_LOADER) :
  {
    KEEP(*(.boot_loader*));
  } > BOOT_LOADER

} INSERT BEFORE .text;

EXTERN(__RESET_VECTOR);
EXTERN(__EXCEPTIONS); 
EXTERN(__INTERRUPTS); 
EXTERN(DEFAULT_HANDLER);

ENTRY(__RESET_VECTOR);

PROVIDE(TIMER_IRQ_0 = DEFAULT_HANDLER);
PROVIDE(TIMER_IRQ_1 = DEFAULT_HANDLER);
PROVIDE(TIMER_IRQ_2 = DEFAULT_HANDLER);
PROVIDE(TIMER_IRQ_3 = DEFAULT_HANDLER);
PROVIDE(PWM_IRQ_WRAP = DEFAULT_HANDLER);
PROVIDE(USBCTRL_IRQ = DEFAULT_HANDLER);
PROVIDE(XIP_IRQ = DEFAULT_HANDLER);
PROVIDE(PIO0_IRQ_0 = DEFAULT_HANDLER);
PROVIDE(PIO0_IRQ_1 = DEFAULT_HANDLER);
PROVIDE(PIO1_IRQ_0 = DEFAULT_HANDLER);
PROVIDE(PIO1_IRQ_1 = DEFAULT_HANDLER);
PROVIDE(DMA_IRQ_0 = DEFAULT_HANDLER);
PROVIDE(DMA_IRQ_1 = DEFAULT_HANDLER);
PROVIDE(IO_IRQ_BANK0 = DEFAULT_HANDLER);
PROVIDE(IO_IRQ_QSPI = DEFAULT_HANDLER);
PROVIDE(SIO_IRQ_PROC0 = DEFAULT_HANDLER);
PROVIDE(SIO_IRQ_PROC1 = DEFAULT_HANDLER);
PROVIDE(CLOCKS_IRQ = DEFAULT_HANDLER);
PROVIDE(SPI0_IRQ = DEFAULT_HANDLER);
PROVIDE(SPI1_IRQ = DEFAULT_HANDLER);
PROVIDE(UART0_IRQ = DEFAULT_HANDLER);
PROVIDE(UART1_IRQ = DEFAULT_HANDLER);
PROVIDE(ADC_IRQ_FIFO = DEFAULT_HANDLER);
PROVIDE(I2C0_IRQ = DEFAULT_HANDLER);
PROVIDE(I2C1_IRQ = DEFAULT_HANDLER);
PROVIDE(RTC_IRQ = DEFAULT_HANDLER);
PROVIDE(SWI_IRQ_0 = DEFAULT_HANDLER);
PROVIDE(SWI_IRQ_1 = DEFAULT_HANDLER);
PROVIDE(SWI_IRQ_2 = DEFAULT_HANDLER);
PROVIDE(SWI_IRQ_3 = DEFAULT_HANDLER);
PROVIDE(SWI_IRQ_4 = DEFAULT_HANDLER);
PROVIDE(SWI_IRQ_5 = DEFAULT_HANDLER);
PROVIDE(NON_MASKABLE_INT = DEFAULT_HANDLER);
PROVIDE(HARDFAULT_TRAMPOLINE = DEFAULT_HANDLER);
PROVIDE(HARDFAULT = DEFAULT_HANDLER);
PROVIDE(SV_CALL = DEFAULT_HANDLER);
PROVIDE(PEND_SV = DEFAULT_HANDLER);
PROVIDE(SYS_TICK = DEFAULT_HANDLER);

SECTIONS
{
  PROVIDE(_ram_start = ORIGIN(RAM));
  PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
  PROVIDE(_stack_start = _ram_end);

  .vector_table ORIGIN(FLASH) :
  {
    __vector_table = .;
    LONG(_stack_start & 0xFFFFFFF8);

    KEEP(*(.vector_table.reset_vector)); 

    __exceptions = .; 
    KEEP(*(.vector_table.exceptions)); 
    __eexceptions = .; 

    KEEP(*(.vector_table.interrupts)); 
  } > FLASH

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  .text _stext :
  {
    __stext = .;
    *(.Reset);

    *(.text .text.*);
    *(.HARDFAULT_TRAMPOLINE);
    *(.HARDFAULT.*);

    . = ALIGN(4); 
    __etext = .;
  } > FLASH

  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);

    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); 
  } > RAM AT>FLASH
  . = ALIGN(4);
  __edata = .;
  __sidata = LOADADDR(.data);
  .gnu.sgstubs : ALIGN(32)
  {
    . = ALIGN(32);
    __veneer_base = .;
    *(.gnu.sgstubs*)
    . = ALIGN(32);
  } > FLASH
  . = ALIGN(32);
  __veneer_limit = .;

  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON); 
    . = ALIGN(4); 
  } > RAM
  . = ALIGN(4);
  __ebss = .;

  .uninit (NOLOAD) : ALIGN(4)

  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM

  PROVIDE(__sheap = __euninit);
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }
  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(cortex-m-rt): the start of the FLASH region must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(cortex-m-rt): the start of the RAM region must be 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
BUG(cortex-m-rt): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
BUG(cortex-m-rt): the LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
BUG(cortex-m-rt): .bss is not 4-byte aligned");

ASSERT(__sheap % 4 == 0, "
BUG(cortex-m-rt): start of .heap is not 4-byte aligned");

ASSERT(_stack_start % 8 == 0, "
ERROR(cortex-m-rt): stack start address is not 8-byte aligned.
If you have set _stack_start, check it's set to an address which is a multiple of 8 bytes.
If you haven't, stack starts at the end of RAM by default. Check that both RAM
origin and length are set to multiples of 8 in the `memory.x` file.");
ASSERT(__exceptions == ADDR(.vector_table) + 0x8, "
BUG(cortex-m-rt): the reset vector is missing");

ASSERT(__eexceptions == ADDR(.vector_table) + 0x40, "
BUG(cortex-m-rt): the exception vectors are missing");

ASSERT(SIZEOF(.vector_table) > 0x40, "
ERROR(cortex-m-rt): The interrupt vectors are missing.
Possible solutions, from most likely to less likely:
- Link to a svd2rust generated device crate
- Check that you actually use the device/hal/bsp crate in your code
- Disable the 'device' feature of cortex-m-rt to build a generic application (a dependency
may be enabling it)
- Supply the interrupt handlers yourself. Check the documentation for details.");

ASSERT(ADDR(.vector_table) + SIZEOF(.vector_table) <= _stext, "
ERROR(cortex-m-rt): The .text section can't be placed inside the .vector_table section
Set _stext to an address greater than the end of .vector_table (See output of `nm`)");
ASSERT(_stext > ORIGIN(FLASH) && _stext < ORIGIN(FLASH) + LENGTH(FLASH), "
ERROR(cortex-m-rt): The .text section must be placed inside the FLASH memory.
Set _stext to an address within the FLASH region.");
ASSERT(SIZEOF(.got) == 0, "
ERROR(cortex-m-rt): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");
ASSERT(SIZEOF(.vector_table) <= 0xc0, "
There can't be more than 32 interrupt handlers. This may be a bug in
your device crate, or you may have registered more than 32 interrupt
handlers.");
