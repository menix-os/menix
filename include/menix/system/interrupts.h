// Function prototypes for interrupts

#pragma once

#include <menix/common.h>

typedef struct Context Context;
typedef Context* (*InterruptFn)(Context* regs);
typedef void (*IrqFn)(usize irq, void* data);

// Internal function to register an interrupt handler at a specific ISR index on the current CPU.
// ! If you thought about using this function for IRQs, you probably meant to use `irq_register_handler`.
void isr_register_handler(usize cpu, usize idx, InterruptFn handler);

// Registers a new IRQ handler. Automatically selects optimal IRQ placement.
// You can also pass an additional parameter which will be for
// Returns `true` upon success.
bool irq_register_handler(IrqFn handler, void* data);
