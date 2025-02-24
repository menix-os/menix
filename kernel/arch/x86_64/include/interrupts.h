#pragma once

#include <menix/system/arch.h>

Context* interrupt_debug_handler(usize isr, Context* regs);
Context* interrupt_ud_handler(usize isr, Context* regs);
Context* interrupt_pf_handler(usize isr, Context* regs);
Context* syscall_handler(usize isr, Context* regs);
