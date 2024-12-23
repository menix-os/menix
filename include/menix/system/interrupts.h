// Function prototypes for interrupts

#pragma once

#include <menix/common.h>

typedef struct Context Context;
typedef Context* (*InterruptFn)(usize isr, Context* regs, void* priv);

// Handler called by the processor.
Context* int_handler(usize isr, Context* regs);

// Registers a new IRQ handler. Automatically selects optimal IRQ placement.
// You can also pass an additional parameter for context passed to the handler.
// Returns `true` upon success.
bool irq_allocate_handler(InterruptFn handler, void* data);

// Internal function to register an interrupt handler at a specific ISR index on the current CPU.
// ! If you thought about using this function for IRQs, you probably meant to use `irq_register_handler`.
bool isr_register_handler(usize cpu, usize idx, InterruptFn handler, void* data);
