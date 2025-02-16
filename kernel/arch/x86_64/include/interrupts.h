#pragma once

#include <menix/system/arch.h>

Context* interrupt_debug_handler(usize isr, Context* regs, void* data);
Context* interrupt_ud_handler(usize isr, Context* regs, void* data);
Context* interrupt_pf_handler(usize isr, Context* regs, void* data);
Context* syscall_handler(usize isr, Context* regs, void* data);
