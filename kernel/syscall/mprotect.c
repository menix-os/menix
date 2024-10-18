// Syscalls for virtual memory management.

#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>

// Updates the permissions of an existing mappping.
// Returns 0 upon success, otherwise -1.
SYSCALL_IMPL(mprotect, VirtAddr addr, usize length, int prot)
{
	Process* proc = arch_current_cpu()->thread->parent;

	if (vm_protect(proc->page_map, addr, length, prot) == true)
		return 0;
	else
		return -1;
}
