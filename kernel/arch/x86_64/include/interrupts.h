#pragma once

#include <menix/system/arch.h>

Context* interrupt_ud_handler(Context* regs, void* data);
Context* interrupt_pf_handler(Context* regs, void* data);
Context* syscall_handler(Context* regs, void* data);
