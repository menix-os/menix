//? Function prototypes for interrupts

#pragma once

#include <menix/common.h>
#include <menix/log.h>

extern void int_error_handler(void);
extern void int_error_handler_with_code(void);
extern void int_syscall_handler(void);
extern void sc_syscall_handler(void);
