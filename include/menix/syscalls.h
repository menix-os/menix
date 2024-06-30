//? System call interface + prototypes

#pragma once

#include <menix/common.h>

#include "menix/log.h"

#define SYSCALL1(name, arg0)
#define SYSCALL2(name, arg0, arg1)
#define SYSCALL3(name, arg0, arg1, arg2)
#define SYSCALL4(name, arg0, arg1, arg2, arg3)
#define SYSCALL5(name, arg0, arg1, arg2, arg3, arg4)
#define SYSCALL6(name, arg0, arg1, arg2, arg3, arg4, arg5)

void interrupt_syscall()
{
	int32_t num;
	asm volatile("movl %%eax, %0" : "=r"(num));
	kmesg(LOG_INFO, "Hello from system call!\n");
	kmesg(LOG_INFO, "SYSCALL(%i)\n", num);
}
