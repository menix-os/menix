// Function prototypes for interrupts

#pragma once

#include <menix/common.h>

typedef struct Context Context;
typedef Context* (*InterruptFn)(Context* regs);

// Registers a new IRQ handler. Automatically selects optimal IRQ placement.
void isr_register_irq(InterruptFn handler);

// Internal function to register an interrupt handler at a specific ISR index on the current CPU.
// ! If you thought about using this function for IRQs, you probably meant to use `isr_register_irq`.
void isr_register_handler(usize cpu, usize idx, InterruptFn handler);
