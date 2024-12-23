// Handles archctl invocations.

#include <menix/system/arch.h>
#include <menix/system/archctl.h>
#include <menix/util/log.h>

usize arch_archctl(ArchCtl ctl, usize arg1, usize arg2)
{
	switch (ctl)
	{
		// Sets User FS base.
		case ArchCtl_SetFsBase:
		{
			asm_wrmsr(MSR_FS_BASE, arg1);
			arch_log("Setting FSBASE to 0x%p\n", arg1);
			break;
		}
		case ArchCtl_None:
		default: arch_log("Unknown archctl 0x%zx", ctl);
	}
	return 0;
}
