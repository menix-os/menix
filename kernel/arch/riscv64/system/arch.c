// riscv64 platform initialization

#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/fw.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

static BootInfo* boot_info;
static SpinLock cpu_lock = spin_new();

void arch_init_cpu(Cpu* cpu, Cpu* boot)
{
	// Make sure no other memory accesses happen before the CPUs are initialized.
	spin_lock(&cpu_lock);

	// TODO: CPU init.

	if (cpu->id != boot->id)
	{
		boot_info->cpu_active += 1;
		spin_unlock(&cpu_lock);
		asm_interrupt_disable();
		while (1)
			asm volatile("wfi");
	}
	boot_info->cpu_active += 1;
	spin_unlock(&cpu_lock);
}

void arch_early_init(BootInfo* info)
{
	asm_interrupt_disable();

	// Initialize physical and virtual memory managers.
	pm_init(info->phys_map, info->memory_map, info->mm_num);
	vm_init(info->kernel_phys, info->memory_map, info->mm_num);

	boot_info = info;
}

void arch_shutdown(BootInfo* info)
{
	arch_stop(info);
}

void arch_stop(BootInfo* info)
{
	// TODO: Send IPI to stop all processors.
	asm_interrupt_disable();
	while (1)
		asm volatile("wfi");
}

Cpu* arch_current_cpu()
{
#ifdef CONFIG_smp
	// The Cpu struct starts at tp + 0
	// Since we can't "directly" access the base address, just get the first field (Cpu.id)
	// and use that to index into the CPU array.
	u64 id;
	asm volatile("sd %0, %1(tp)" : "=r"(id) : "i"(offsetof(Cpu, id)) : "memory");
	return &boot_info->cpus[id];
#else
	return &boot_info->cpus[0];
#endif
}

void arch_get_registers(Context* regs)
{
	if (regs == NULL)
		return;

	asm_get_register(regs->x31, x31);
	asm_get_register(regs->x30, x30);
	asm_get_register(regs->x29, x29);
	asm_get_register(regs->x28, x28);
	asm_get_register(regs->x27, x27);
	asm_get_register(regs->x26, x26);
	asm_get_register(regs->x25, x25);
	asm_get_register(regs->x24, x24);
	asm_get_register(regs->x23, x23);
	asm_get_register(regs->x22, x22);
	asm_get_register(regs->x21, x21);
	asm_get_register(regs->x20, x20);
	asm_get_register(regs->x19, x19);
	asm_get_register(regs->x18, x18);
	asm_get_register(regs->x17, x17);
	asm_get_register(regs->x16, x16);
	asm_get_register(regs->x15, x15);
	asm_get_register(regs->x14, x14);
	asm_get_register(regs->x13, x13);
	asm_get_register(regs->x12, x12);
	asm_get_register(regs->x11, x11);
	asm_get_register(regs->x10, x10);
	asm_get_register(regs->x9, x9);
	asm_get_register(regs->x8, x8);
	asm_get_register(regs->x7, x7);
	asm_get_register(regs->x6, x6);
	asm_get_register(regs->x5, x5);
	asm_get_register(regs->x4, x4);
	asm_get_register(regs->x3, x3);
	asm_get_register(regs->x2, x2);
	asm_get_register(regs->x1, x1);
}

void arch_dump_registers(Context* regs)
{
	kmesg("pc:  0x%p ra:  0x%p sp:  0x%p gp:  0x%p\n", regs->pc, regs->x1, regs->x2, regs->x3);
	kmesg("tp:  0x%p t0:  0x%p t1:  0x%p t2:  0x%p\n", regs->x4, regs->x5, regs->x6, regs->x7);
	kmesg("s0:  0x%p s1:  0x%p a0:  0x%p a1:  0x%p\n", regs->x8, regs->x9, regs->x10, regs->x11);
	kmesg("a2:  0x%p a3:  0x%p a4:  0x%p a5:  0x%p\n", regs->x12, regs->x13, regs->x14, regs->x15);
	kmesg("a6:  0x%p a7:  0x%p s2:  0x%p s3:  0x%p\n", regs->x16, regs->x17, regs->x18, regs->x19);
	kmesg("s4:  0x%p s5:  0x%p s6:  0x%p s7:  0x%p\n", regs->x20, regs->x21, regs->x22, regs->x23);
	kmesg("s8:  0x%p s9:  0x%p s10: 0x%p s11: 0x%p\n", regs->x24, regs->x25, regs->x26, regs->x27);
	kmesg("t3:  0x%p t4:  0x%p t5:  0x%p t6:  0x%p\n", regs->x28, regs->x29, regs->x30, regs->x31);
}
