// Function prototypes for interrupts

#pragma once

#include <menix/common.h>
#include <menix/log.h>

extern void int_error_handler(void);
extern void int_error_handler_with_code(void);

// Assembly stub for syscall via interrupt.
extern void int_syscall(void);

// Assembly stub for syscall via SYSCALL/SYSRET.
extern void sc_syscall(void);
