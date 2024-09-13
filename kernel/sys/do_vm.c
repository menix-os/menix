// Syscalls for virtual memory management.

#include <menix/arch.h>
#include <menix/memory/vm.h>
#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

// Maps a phyiscal address to a virtual one.
// Returns the start of new memory.
SYSCALL_IMPL(mmap, VirtAddr hint, usize length, int prot, int flags)
{
	Process* proc = arch_current_cpu()->thread->parent;

	// TODO: Get fd and offset.

	void* mem = vm_map(proc->page_map, hint, length, prot, flags, NULL, 0);

	return (usize)mem;
}

// Updates the permissions of an existing mappping.
// Returns 0 upon success, otherwise -1.
SYSCALL_IMPL(mprotect, void* addr, usize length, int prot)
{
	Process* proc = arch_current_cpu()->thread->parent;

	if (vm_protect(proc->page_map, addr, length, prot) == true)
		return 0;
	else
		return -1;
}

// Destroys an existing mapping.
// Returns 0 upon success, otherwise -1.
SYSCALL_IMPL(munmap, void* addr, usize length)
{
	Process* proc = arch_current_cpu()->thread->parent;

	if (vm_unmap(proc->page_map, addr, length) == true)
		return 0;
	else
		return -1;
}
