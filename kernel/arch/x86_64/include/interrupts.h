#pragma once

#include <menix/system/arch.h>

Context* interrupt_ud_handler(Context* regs);
Context* interrupt_pf_handler(Context* regs);
Context* syscall_handler(Context* regs);
