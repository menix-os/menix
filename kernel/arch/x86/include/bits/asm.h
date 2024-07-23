// x86 specific inline assembly snippets.

#pragma once

// Loads the GDT.
#define gdt_set(table) asm volatile("lgdt %0" ::"m"(table))

// Flushes all segment registers and reloads them.
#define flush_segment_regs(code_seg, data_seg) \
	asm volatile("push %0\n" \
				 "movq $L_reload_cs, %%rax\n" \
				 "push %%rax\n" \
				 "lretq\n" \
				 "L_reload_cs:\n" \
				 "mov %1, %%ax\n" \
				 "mov %%ax, %%ds\n" \
				 "mov %%ax, %%es\n" \
				 "mov %%ax, %%fs\n" \
				 "mov %%ax, %%gs\n" \
				 "mov %%ax, %%ss\n" \
				 : \
				 : "i"(code_seg), "i"(data_seg) \
				 : "rax")

#define interrupt_disable() asm volatile("cli")
#define interrupt_enable()	asm volatile("sti")
