
#include <stdint.h>

// Declare symbols from the linker script for memory boundaries
extern int entry(void);
extern uint32_t __sidata, __sdata, __edata, __sbss, __ebss;

void RESET(void) {
  uint32_t *source;
  uint32_t *destination;

  // Copy the data segment initializers from flash to SRAM
  source = &__sidata;
  for (destination = &__sdata; destination < &__edata;) {
    *(destination++) = *(source++);
  }

  // Zero fill the bss segment
  for (destination = &__sbss; destination < &__ebss;) {
    *(destination++) = 0;
  }

  // Call the main application
  entry();

  // In case main returns, ensure a safe hang
  while (1) {
  }
}
