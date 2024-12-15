// Task State Segment

#pragma once

#include <menix/common.h>

typedef struct
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
} ATTR(packed) ATTR(aligned(0x10)) TaskStateSegment;

// Initializes the TSS.
void tss_init(TaskStateSegment* tss);

// Reloads the current TSS.
void tss_reload();
