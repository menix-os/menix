// System contol for syscalls, similar to ioctl.

#pragma once

#include <menix/common.h>

typedef enum : usize
{
	ArchCtl_None = 0,

#ifdef CONFIG_arch_x86_64
	ArchCtl_SetFsBase = 1,
#endif

} ArchCtl;

void archctl(ArchCtl ctl, usize arg1, usize arg2);
