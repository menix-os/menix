// Task State Segement helper functions

#include <menix/memory/vm.h>
#include <menix/util/log.h>

#include <gdt.h>
#include <tss.h>

void tss_init(TaskStateSegment* tss)
{
	kassert(tss != NULL, "No valid TSS was provided!");

	// Allocate stack.
	const usize stack_pages = VM_USER_STACK_SIZE / vm_get_page_size(VMLevel_Small);
	tss->rsp0 = pm_alloc(stack_pages) + (u64)pm_get_phys_base();
	tss->ist1 = pm_alloc(stack_pages) + (u64)pm_get_phys_base();
	tss->iopb = sizeof(TaskStateSegment);
}

void tss_set_stack(TaskStateSegment* tss, void* rsp)
{
	kassert(tss != NULL, "No valid TSS was provided!");
	tss->rsp0 = (u64)rsp;
	tss->rsp1 = (u64)rsp;
	tss->rsp2 = (u64)rsp;
}

void tss_reload()
{
	asm volatile("movw %0, %%ax\n"
				 "ltr %%ax\n"
				 :
				 : "r"((u16)offsetof(Gdt, tss))
				 : "ax");
}
