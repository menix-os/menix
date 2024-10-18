// Syscalls for virtual memory management.

#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>

// Maps a phyiscal address to a virtual one.
// Returns the start of new memory.
SYSCALL_IMPL(mmap, VirtAddr hint, usize length, int prot, int flags)
{
	Process* proc = arch_current_cpu()->thread->parent;

	// TODO: Get fd and offset.

	VirtAddr mem = vm_map(proc->page_map, hint, length, prot, flags, NULL, 0);

	return mem;
}
