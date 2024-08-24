// Task State Segment

#include <menix/common.h>

#include <gdt.h>

typedef struct ATTR(packed)
{
	u32 reserved0;
	u64 rsp0;
	u64 rsp1;
	u64 rsp2;
	u32 reserved1;
	u32 reserved2;
	u64 ist1;
	u64 ist2;
	u64 ist3;
	u64 ist4;
	u64 ist5;
	u64 ist6;
	u64 ist7;
	u32 reserved3;
	u32 reserved4;
	u16 reserved5;
	u16 iopb;
} TaskStateSegment;

// Initializes the TSS.
void tss_init(TaskStateSegment* tss);

// Reloads the current TSS.
static inline void tss_reload()
{
	asm volatile("movw %0, %%ax\n"
				 "ltr %%ax\n" ::"r"((u16)offsetof(Gdt, tss)));
}
