// Function prototypes for interrupts

#pragma once

#include <menix/common.h>
#include <menix/log.h>

// Declares an interrupt handler.
#define INT_HANDLER(num)	  interrupt_##num
#define INT_HANDLER_DECL(num) extern void INT_HANDLER(num)(void)

// Assembly stub for syscall via SYSCALL/SYSRET.
extern void sc_syscall(void);
