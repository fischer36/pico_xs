#pragma once
#include <cstdint>

typedef struct {
  volatile uint32_t RESET;
  volatile uint32_t WDSEL;
  volatile uint32_t DONE;
} RESETS;
