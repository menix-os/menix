// Syscall "mmap"

#include <menix/arch.h>
#include <menix/memory/vm.h>
#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

// Maps a phyiscal address to a virtual one.
SYSCALL_IMPL(mmap, void* hint, usize length, int prot, int flags)
{
	Process* proc = arch_current_cpu()->thread->parent;

	void* mem = vm_map(proc->page_map, hint, length, prot, flags);

	return (usize)mem;
}
