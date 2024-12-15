// Task State Segement helper functions

#include <menix/util/log.h>

#include <gdt.h>
#include <tss.h>

void tss_init(TaskStateSegment* tss)
{
	kassert(tss != NULL, "No valid TSS was provided!");
	tss->rsp0 = 0;
	tss->rsp1 = 0;
	tss->rsp2 = 0;
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
