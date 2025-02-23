#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>

#include <uapi/errno.h>

SYSCALL_IMPL(mmap, VirtAddr hint, usize length, int prot, int flags, int fd, usize offset)
{
	// If length is not given or if the hint addr is not page aligned.
	if (length == 0 || hint % vm_get_page_size(VMLevel_Small) != 0)
		return SYSCALL_ERR(EINVAL);

	Thread* thread = arch_current_cpu()->thread;
	Process* proc = thread->parent;
	PageMap* page_map = proc->page_map;

	// TODO: Get fd and offset.

	const usize page_size = vm_get_page_size(VMLevel_Small);

	VMProt vm_prot = 0;
	if (prot & PROT_READ)
		vm_prot |= VMProt_Read;
	if (prot & PROT_WRITE)
		vm_prot |= VMProt_Write;
	if (prot & PROT_EXEC)
		vm_prot |= VMProt_Execute;

	VirtAddr addr = 0;
	length = ALIGN_UP(length, page_size);
	usize page_count = length / page_size;
	VirtAddr aligned_hint = ALIGN_DOWN(hint, page_size);

	// If the mapping already exists and MAP_FIXED_NOREPLACE was set, the mapping can't succeed.
	if ((flags & MAP_FIXED_NOREPLACE))
	{
		for (usize i = 0; i < page_size * page_count; i += page_size)
		{
			if (vm_is_mapped(page_map, hint + i, vm_prot))
				return SYSCALL_ERR(EEXIST);
		}
	}

	// Check the hint and make changes if necessary.
	if (flags & MAP_FIXED)
	{
		// Check if we're mapping between pages. If yes, we need one more page.
		if (aligned_hint < hint)
			page_count += 1;

		addr = hint;
	}
	else
	{
		// Choose the next free region of virtual memory if no hint was given.
		addr = proc->map_base;
		// TODO: The map_base should only be relevant when not doing a MAP_FIXED.
		// TODO: This might waste a ton of available virtual address space!
		proc->map_base += page_size * page_count;
	}

	PhysAddr page = pm_alloc(page_count);
	for (usize i = 0; i < page_size * page_count; i += page_size)
	{
		if (vm_map(page_map, page + i, addr + i, vm_prot, VMFlags_User, VMLevel_Small) == false)
		{
			pm_free(page, page_count);
			return SYSCALL_ERR(ENOMEM);
		}
	}
	MemoryMapping mapping = {
		.physical = page,
		.virtual = addr,
		.num_pages = page_count,
		.prot = vm_prot,
		.flags = VMFlags_User,
	};
	list_push(&proc->page_map->maps, mapping);

	return SYSCALL_OK(addr);
}
