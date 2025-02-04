#pragma once

#include <menix/common.h>

#include <gdt.h>

typedef struct ArchCpu
{
	Gdt gdt;
	TaskStateSegment tss;
	u32 lapic_id;					   // Local APIC ID.
	usize fpu_size;					   // Size of the FPU in bytes.
	void (*fpu_save)(void* dst);	   // Function to call when saving the FPU state.
	void (*fpu_restore)(void* dst);	   // Function to call when restoring the FPU state.
} ArchCpu;
