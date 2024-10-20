// Function prototypes for interrupts

#pragma once

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/util/log.h>

// Declares an interrupt handler.
#define INT_HANDLER(num)	  interrupt_##num
#define INT_HANDLER_DECL(num) extern void INT_HANDLER(num)(void)

void interrupt_register(usize idx, void (*handler)(Context*));

// Page fault interrupt handler. Set by vm_init().
void interrupt_pf_handler(Context* regs);
