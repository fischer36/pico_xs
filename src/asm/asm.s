
// gpio_set.s
.section .text
.global gpio_set

// void gpio_set(uint32_t gpio)
// r0 = gpio
gpio_set:
    push {r4, lr}                  // Save link register and general-purpose register

    ldr r1, =0xd0000000                       // Load SIO base address into r1, replace 0xd0000000 with actual SIO_BASE
    add r1, r1, #0x14              // Add offset for GPIO_OUT_SET (0x14) to base address

    mov r2, #1                     // Move 1 into r2
    lsl r2, r2, r0                 // Shift left r2 by r0 places, storing result in r2

    str r2, [r1]                   // Store the value from r2 into the memory address pointed to by r1

    pop {r4, pc}                   // Restore registers and return
