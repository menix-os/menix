// riscv64 platform initialization

#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

Cpu per_cpu_data[MAX_CPUS];

void arch_init_cpu(Cpu* cpu, Cpu* boot)
{
	static SpinLock cpu_lock = {0};
	// Make sure no other memory accesses happen before the CPUs are initialized.
	spin_lock(&cpu_lock);

	// TODO: CPU init.

	u64 stvec = STVEC_MODE_DIRECT | (u64)arch_int_internal;
	asm_write_csr(stvec, stvec);

	spin_unlock(&cpu_lock);
}

void arch_early_init()
{
	asm_interrupt_disable();
}

void arch_init(BootInfo* info)
{
	asm_interrupt_disable();
}

void arch_stop()
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
	return &per_cpu_data[id];
#else
	return &per_cpu_data[0];
#endif
}

usize arch_archctl(ArchCtl ctl, usize arg1, usize arg2)
{
	return 0;
}

void arch_dump_registers(Context* regs)
{
	print_log("pc:  0x%p ra:  0x%p sp:  0x%p gp:  0x%p\n", regs->pc, regs->x1, regs->x2, regs->x3);
	print_log("tp:  0x%p t0:  0x%p t1:  0x%p t2:  0x%p\n", regs->x4, regs->x5, regs->x6, regs->x7);
	print_log("s0:  0x%p s1:  0x%p a0:  0x%p a1:  0x%p\n", regs->x8, regs->x9, regs->x10, regs->x11);
	print_log("a2:  0x%p a3:  0x%p a4:  0x%p a5:  0x%p\n", regs->x12, regs->x13, regs->x14, regs->x15);
	print_log("a6:  0x%p a7:  0x%p s2:  0x%p s3:  0x%p\n", regs->x16, regs->x17, regs->x18, regs->x19);
	print_log("s4:  0x%p s5:  0x%p s6:  0x%p s7:  0x%p\n", regs->x20, regs->x21, regs->x22, regs->x23);
	print_log("s8:  0x%p s9:  0x%p s10: 0x%p s11: 0x%p\n", regs->x24, regs->x25, regs->x26, regs->x27);
	print_log("t3:  0x%p t4:  0x%p t5:  0x%p t6:  0x%p\n", regs->x28, regs->x29, regs->x30, regs->x31);
}
