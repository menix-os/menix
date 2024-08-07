// Virtual memory management for x86.

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/util/self.h>

#include <idt.h>
#include <string.h>

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
#define PAGE_PROTECTION_KEY(key) ((key & 0xFUL) << 59)
#define PAGE_EXECUTE_DISABLE	 (1ULL << 63)

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

SEGMENT_DECLARE_SYMBOLS(text)
SEGMENT_DECLARE_SYMBOLS(rodata)
SEGMENT_DECLARE_SYMBOLS(data)

PageMap* kernel_map = NULL;		  // Page map used for the kernel.
static void* phys_addr = NULL;	  // Memory mapped lower physical memory.

void vm_init(void* phys_base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries)
{
	// Update the page fault handler.
	idt_set(0x0E, vm_page_fault_handler, IDT_TYPE(0, IDT_GATE_INT));
	idt_reload();

	phys_addr = phys_base;
	kassert(num_entries >= 1, "No memory map entries given!");

	// Get a pointer to the first free physical memory page. Here we'll allocate our page directory structure.
	// TODO: Replace with `kernel_map = kalloc(sizeof(PageMap));`
	kernel_map = phys_addr + pm_arch_alloc(1);

	kernel_map->lock = spin_new();
	kernel_map->head = phys_addr + pm_arch_alloc(1);
	memset(kernel_map->head, 0x00, CONFIG_page_size);

	// Restore the HHDM mapping.
	for (usize cur = 0; cur < 4UL * GiB; cur += 2UL * MiB)
		kassert(vm_arch_map_page(kernel_map, cur, phys_addr + cur, PAGE_PRESENT | PAGE_READ_WRITE, PageSize_2MiB),
				"Unable to recreate HHDM map!\n");

	// Map ourselves to the current physical address again.
	for (usize cur = (usize)SEGMENT_START(text); cur < (usize)SEGMENT_END(text); cur += 4UL * KiB)
		kassert(vm_arch_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, (void*)cur, PAGE_PRESENT,
								 PageSize_4KiB),
				"Unable to map text segment!\n");

	for (usize cur = (usize)SEGMENT_START(rodata); cur < (usize)SEGMENT_END(rodata); cur += 4UL * KiB)
		kassert(vm_arch_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, (void*)cur,
								 PAGE_PRESENT | PAGE_EXECUTE_DISABLE, PageSize_4KiB),
				"Unable to map rodata segment!\n");

	for (usize cur = (usize)SEGMENT_START(data); cur < (usize)SEGMENT_END(data); cur += 4UL * KiB)
		kassert(vm_arch_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, (void*)cur,
								 PAGE_PRESENT | PAGE_READ_WRITE | PAGE_EXECUTE_DISABLE, PageSize_4KiB),
				"Unable to map data segment!\n");

	// If the physical base ever changes, update it in the physical memory manager as well
	// so it can still access its bit map.
	pm_update_phys_base(phys_addr);

	// Load the new page directory.
	vm_set_page_map(kernel_map);
}

void vm_set_page_map(PageMap* map)
{
	asm_interrupt_disable();
	asm_set_register((((usize)map->head) - ((usize)phys_addr)), cr3);	 // TODO: Triple faults for some reason.
	asm_interrupt_enable();
}

static u64* get_next_level(u64* top_level, usize idx, bool allocate)
{
	if (top_level[idx] & PAGE_PRESENT)
		return (u64*)((usize)(top_level[idx] & ~((u64)(CONFIG_page_size - 1))) + phys_addr);

	if (!allocate)
		return NULL;

	PhysAddr next_level = pm_arch_alloc(1);
	memset(phys_addr + next_level, 0x0, CONFIG_page_size);
	top_level[idx] = (u64)next_level | 0b111;

	return (u64*)(phys_addr + next_level);
}

PhysAddr vm_arch_virt_to_phys(void* address)
{
	// TODO
	return 0;
}

bool vm_arch_map_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags, PageSize size)
{
	spin_acquire_force(&page_map->lock);

	const usize virt_val = (usize)virt_addr;
	u64* cur_head = page_map->head;
	usize bits = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		bits = (virt_val & ((usize)0x1FF << shift)) >> shift;
		cur_head = get_next_level(cur_head, bits, true);

		if (!cur_head)
		{
			spin_free(&page_map->lock);
			return false;
		}

		if (lvl == 3 && size == PageSize_2MiB)
		{
			flags |= PAGE_SIZE;
			break;
		}
	}

	cur_head[bits] = phys_addr | flags;

	spin_free(&page_map->lock);
	return true;
}

void vm_arch_unmap_page(PageMap* page_map, void* virt_addr)
{
	vm_flush_tlb(virt_addr);
}

void vm_page_fault_handler(u32 fault, u32 error)
{
	usize addr;
	asm_get_register(addr, cr2);

	u16 cs;
	asm_get_register(cs, cs);

	// If the current protection level wasn't 3, in other words, the page fault was caused by the supervisor
	// instead of the user, we messed up big time!
	// kassert(cs & 3, "Page fault caused while in supervisor mode!\n");
	// TODO Handle user page fault.
}
