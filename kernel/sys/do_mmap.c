// Syscall "mmap"

#include <menix/arch.h>
#include <menix/memory/vm.h>
#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

// Maps a phyiscal address to a virtual one.
SYSCALL_IMPL(mmap)
{
	void* hint = (void*)args->a0;
	usize length = (usize)args->a1;
	u32 flags = (u32)args->a2;

	Process* proc = arch_current_cpu()->thread->parent;

	void* mem = vm_map(proc->page_map, hint, length, flags);

	return (usize)mem;
}
