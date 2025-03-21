#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/util/log.h>

#include <string.h>

bool vm_map(PageMap* page_map, PhysAddr phys_addr, VirtAddr virt_addr, VMProt prot, VMFlags flags, VMLevel size)
{
	// TODO
	return true;
}

bool vm_protect(PageMap* page_map, VirtAddr virt_addr, VMProt prot, VMFlags flags)
{
	// TODO
	return true;
}

bool vm_unmap(PageMap* page_map, VirtAddr virt_addr)
{
	// TODO
	return true;
}

bool vm_is_mapped(PageMap* page_map, VirtAddr address, VMProt prot)
{
	// TODO
	return true;
}

void vm_set_page_map(PageMap* page_map)
{
	// TODO
}

PageMap* vm_page_map_new()
{
	// TODO
	return NULL;
}

PageMap* vm_page_map_fork(PageMap* source)
{
	// TODO
	return NULL;
}

void vm_page_map_destroy(PageMap* map)
{
	// TODO
}

PhysAddr vm_virt_to_phys(PageMap* page_map, VirtAddr address)
{
	// TODO
	return 0;
}

void vm_user_show()
{
}

void vm_user_hide()
{
}

usize vm_get_page_size(VMLevel level)
{
	switch (level)
	{
		case VMLevel_Small: return 4 * KiB;
		case VMLevel_Medium: return 2 * MiB;
		case VMLevel_Large: return 1 * GiB;
		default: return 0;
	}
}
