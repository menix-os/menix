// Virtual memory management for x86.

#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/util/self.h>

#define PAGE_PRESENT			 (1 << 0)
#define PAGE_READ_WRITE			 (1 << 1)
#define PAGE_USER_MODE			 (1 << 2)
#define PAGE_WRITE_THROUGH		 (1 << 3)
#define PAGE_CACHE_DISABLE		 (1 << 4)
#define PAGE_ACCESSED			 (1 << 5)
#define PAGE_DIRTY				 (1 << 6)
#define PAGE_SIZE				 (1 << 7)
#define PAGE_GLOBAL				 (1 << 8)
#define PAGE_AVAILABLE			 (1 << 9)
#define PAGE_ATTRIBUTE_TABLE	 (1 << 10)
#define PAGE_PROTECTION_KEY(key) ((key & 0xF) << 59)
#define PAGE_EXECUTE_DISABLE	 (1ULL << 63)

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

SEGMENT_DECLARE_SYMBOLS(text)
SEGMENT_DECLARE_SYMBOLS(rodata)
SEGMENT_DECLARE_SYMBOLS(data)

PageMap* kernel_map = NULL;		  // Page map used for the kernel.
static void* phys_addr = NULL;	  // Memory mapped physical memory.

void vm_init(void* phys_base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries)
{
	// TODO: Finish kalloc, then set new memory map!

	phys_addr = phys_base;
	kassert(num_entries >= 1, "No memory map entries given!");

	// Get a pointer to the first free physical memory page. Here we'll allocate our page directory structure.
	kernel_map = kalloc(sizeof(PageMap));

	// Map ourselves to the current physical address again.
	for (usize cur = (usize)SEGMENT_START(text); cur < (usize)SEGMENT_END(text); cur += CONFIG_page_size)
		vm_arch_map_page(kernel_map, cur - (usize)KERNEL_START + kernel_base, (void*)cur, PAGE_PRESENT);
	for (usize cur = (usize)SEGMENT_START(rodata); cur < (usize)SEGMENT_END(rodata); cur += CONFIG_page_size)
		vm_arch_map_page(kernel_map, cur - (usize)KERNEL_START + kernel_base, (void*)cur, PAGE_PRESENT);
	for (usize cur = (usize)SEGMENT_START(data); cur < (usize)SEGMENT_END(data); cur += CONFIG_page_size)
		vm_arch_map_page(kernel_map, cur - (usize)KERNEL_START + kernel_base, (void*)cur, PAGE_PRESENT);

	// Load the new page directory.
	// asm volatile("mov %0, %%cr3" ::"r"(mem_map[0].address) : "memory");
}

static u64* get_next_level(u64* top_level, usize idx, bool allocate)
{
	if (top_level[idx] & 1)
	{
		return (u64*)((usize)(top_level[idx] & ~((u64)0xFFF)) + phys_addr);
	}

	if (!allocate)
	{
		return NULL;
	}

	void* next_level = pm_arch_alloc(1);
	top_level[idx] = (u64)next_level | 0b111;

	return (u64*)((usize)next_level + phys_addr);
}

PhysAddr vm_arch_virt_to_phys(void* address)
{
	// TODO
	return 0;
}

bool vm_arch_map_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags)
{
	// TODO
	spin_acquire_force(&page_map->lock);

	const usize virt_val = (usize)virt_addr;

	// Mask the respective bits for the address (9 bits per level).
	usize page_map_val4 = (virt_val & ((usize)0x1FF << 39)) >> 39;
	usize page_map_val3 = (virt_val & ((usize)0x1FF << 30)) >> 30;
	usize page_map_val2 = (virt_val & ((usize)0x1FF << 21)) >> 21;
	usize page_map_val1 = (virt_val & ((usize)0x1FF << 12)) >> 12;

	u64* page_map_addr4 = page_map->head;
	u64* page_map_addr3;
	u64* page_map_addr2;
	u64* page_map_addr1;

	page_map_addr3 = get_next_level(page_map_addr4, page_map_val4, true);
	if (!page_map_addr3)
		goto fail;

	page_map_addr2 = get_next_level(page_map_addr3, page_map_val3, true);
	if (!page_map_addr2)
		goto fail;

	page_map_addr1 = get_next_level(page_map_addr2, page_map_val2, true);
	if (!page_map_addr1)
		goto fail;

	if ((page_map_addr1[page_map_val1] & PAGE_PRESENT) == 0)
	{
fail:
		spin_free(&page_map->lock);
		return false;
	}

	page_map_addr1[page_map_val1] = (((page_map_addr1[page_map_val1]) & ~((u64)CONFIG_page_size - 1))) | flags;
	vm_flush_tlb(virt_addr);

	spin_free(&page_map->lock);

	return true;
}

void vm_arch_unmap_page(PageMap* page_map, void* virt_addr)
{
	vm_flush_tlb(virt_addr);
}
