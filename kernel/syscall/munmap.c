// Syscalls for virtual memory management.

#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>

#include <uapi/errno.h>

// Destroys an existing mapping.
SYSCALL_IMPL(munmap, VirtAddr addr, usize length)
{
	Process* proc = arch_current_cpu()->thread->parent;

	for (usize i = 0; i < length; i += arch_page_size)
	{
		if (vm_unmap(proc->page_map, addr + i) == false)
			return SYSCALL_FAIL(MAP_FAILED, 0);
	}

	return SYSCALL_OK(0);
}
