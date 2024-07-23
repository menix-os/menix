// Task State Segment
//! menix uses software multitasking, the TSS is only really used as a placeholder.

#include <menix/common.h>

typedef struct ATTR(packed)
{
	uint32_t reserved0;
	uint64_t rsp0;
	uint64_t rsp1;
	uint64_t rsp2;
	uint32_t reserved1;
	uint32_t reserved2;
	uint64_t ist1;
	uint64_t ist2;
	uint64_t ist3;
	uint64_t ist4;
	uint64_t ist5;
	uint64_t ist6;
	uint64_t ist7;
	uint32_t reserved3;
	uint32_t reserved4;
	uint32_t iopb;
} TaskStateSegment;
