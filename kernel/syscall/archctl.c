#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>

// Does architecture specific operations.
SYSCALL_IMPL(archctl, usize operation, usize arg0, usize arg1)
{
	// TODO archctl may fail.
	return SYSCALL_OK(arch_archctl(operation, arg0, arg1));
}
