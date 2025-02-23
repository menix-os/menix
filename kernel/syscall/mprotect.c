#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>

#include <uapi/errno.h>

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
		if (vm_protect(proc->page_map, addr + i, vm_prot, VMFlags_User) == false)
			return SYSCALL_FAIL(MAP_FAILED, 0);
	}

	return SYSCALL_OK(0);
}
