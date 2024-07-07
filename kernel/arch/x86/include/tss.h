//? Task State Segment
//! menix uses software multitasking, the TSS is only really used as a placeholder.

#include <menix/common.h>

typedef struct
{
	uint32_t prev_tss;	  // The previous TSS - with hardware task switching these form a kind of backward linked list.
	uint32_t esp0;		  // The stack pointer to load when changing to kernel mode.
	uint32_t ss0;		  // The stack segment to load when changing to kernel mode.
	// Everything below here is unused.
	uint32_t esp1;	  // esp and ss 1 and 2 would be used when switching to rings 1 or 2.
	uint32_t ss1;
	uint32_t esp2;
	uint32_t ss2;
	uint32_t cr3;
	uint32_t eip;
	uint32_t eflags;
	uint32_t eax;
	uint32_t ecx;
	uint32_t edx;
	uint32_t ebx;
	uint32_t esp;
	uint32_t ebp;
	uint32_t esi;
	uint32_t edi;
	uint32_t es;
	uint32_t cs;
	uint32_t ss;
	uint32_t ds;
	uint32_t fs;
	uint32_t gs;
	uint32_t ldt;
	uint16_t trap;
	uint16_t iomap_base;
} ATTR(packed) TaskStateSegment;
