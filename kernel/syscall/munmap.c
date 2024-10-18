// Syscalls for virtual memory management.

#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>

// Destroys an existing mapping.
// Returns 0 upon success, otherwise -1.
SYSCALL_IMPL(munmap, VirtAddr addr, usize length)
{
	Process* proc = arch_current_cpu()->thread->parent;

	if (vm_unmap(proc->page_map, addr, length) == true)
		return 0;
	else
		return -1;
}
