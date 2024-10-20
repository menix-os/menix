// Syscalls for virtual memory management.

#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>

// Maps memory to a virtual address.
// Returns the start of new memory.
SYSCALL_IMPL(mmap, VirtAddr hint, usize length, int prot, int flags, int fd, usize offset)
{
	if (length == 0)
		return (usize)MAP_FAILED;

	Thread* thread = arch_current_cpu()->thread;
	Process* proc = thread->parent;
	PageMap* page_map = proc->page_map;

	// TODO: Get fd and offset.

	VMProt vm_prot = 0;
	if (prot & PROT_READ)
		vm_prot |= VMProt_Read;
	if (prot & PROT_WRITE)
		vm_prot |= VMProt_Write;
	if (prot & PROT_EXEC)
		vm_prot |= VMProt_Execute;

	VirtAddr addr = 0;
	length = ALIGN_UP(length, arch_page_size);
	usize page_count = length / arch_page_size;
	VirtAddr aligned_hint = ALIGN_DOWN(hint, arch_page_size);

	// Check the hint and make changes if necessary.
	if (flags & MAP_FIXED)
	{
		// Check if there already is a mapping at the hinted address.
		// If there is, we can't take the hint as is.
		if (!vm_unmap(page_map, aligned_hint) && (flags & MAP_FIXED))
			return (VirtAddr)MAP_FAILED;

		// Check if we're mapping between pages. If yes, we need one more page.
		if (aligned_hint < hint)
			page_count += 1;

		addr = hint;
	}
	else
	{
		// Choose the next free region of virtual memory if no hint was given.
		addr = proc->map_base;
	}

	// TODO: The map_base should only be relevant when not doing a MAP_FIXED.
	// TODO: This might waste a ton of available virtual address space!
	proc->map_base += arch_page_size * page_count;

	for (usize i = 0; i < arch_page_size * page_count; i += arch_page_size)
	{
		PhysAddr page = pm_alloc(1);
		if (vm_map(page_map, page, addr + i, vm_prot, 0) == false)
		{
			pm_free(page, 1);
			return (VirtAddr)MAP_FAILED;
		}
	}

	return addr;
}

// Updates the permissions of an existing mappping.
// Returns 0 upon success, otherwise -1.
SYSCALL_IMPL(mprotect, VirtAddr addr, usize length, int prot)
{
	Process* proc = arch_current_cpu()->thread->parent;

	VMProt vm_prot = 0;
	if (prot & PROT_READ)
		vm_prot |= VMProt_Read;
	if (prot & PROT_WRITE)
		vm_prot |= VMProt_Write;
	if (prot & PROT_EXEC)
		vm_prot |= VMProt_Execute;

	for (usize i = 0; i < length; i += arch_page_size)
	{
		if (vm_protect(proc->page_map, addr + i, vm_prot) == false)
			return (usize)MAP_FAILED;
	}

	return 0;
}
// Destroys an existing mapping.
// Returns 0 upon success, otherwise -1.
SYSCALL_IMPL(munmap, VirtAddr addr, usize length)
{
	Process* proc = arch_current_cpu()->thread->parent;

	for (usize i = 0; i < length; i += arch_page_size)
	{
		if (vm_unmap(proc->page_map, addr + i) == false)
			return (usize)MAP_FAILED;
	}

	return 0;
}

SYSCALL_STUB(mremap)
