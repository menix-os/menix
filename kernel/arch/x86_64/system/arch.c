// x86 platform initialization

#include <menix/system/arch.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <apic.h>
#include <gdt.h>
#include <idt.h>
#include <serial.h>
#include <stdatomic.h>

static BootInfo* boot_info;
static SpinLock cpu_lock = spin_new();

// Assembly stub for syscall via SYSCALL/SYSRET.
extern void sc_syscall(void);

extern bool can_smap;

// Initialize one CPU.
void arch_init_cpu(Cpu* cpu, Cpu* boot)
{
	// Make sure no other memory accesses happen before the CPUs are initialized.
	spin_acquire_force(&cpu_lock);

	gdt_reload();

	gdt_load_tss((usize)&cpu->tss);

	// Enable syscall extension (EFER.SCE).
	asm_wrmsr(MSR_EFER, asm_rdmsr(MSR_EFER) | MSR_EFER_SCE);
	// Bits 32-47 are kernel segment base, Bits 48-63 are user segment base. Lower 32 bits (EIP) are unused.
	asm_wrmsr(MSR_STAR, (offsetof(Gdt, kernel_code)) | (offsetof(Gdt, user_code) << 16) << 32);
	// Set syscall entry point.
	asm_wrmsr(MSR_LSTAR, (u64)sc_syscall);
	// Set the flag mask to everything except the second bit (always has to be enabled).
	asm_wrmsr(MSR_SFMASK, (u64) ~((u32)2));

	u32 eax = 0, ebx = 0, ecx = 0, edx = 0;
	u64 cr0, cr4;

	// Get the control registers.
	asm_get_register(cr0, cr0);
	asm_get_register(cr4, cr4);

	// Enable SSE
	cr0 &= ~CR0_EM;	   // Clear EM bit.
	cr0 |= CR0_MP;
	cr4 |= CR4_OSFXSR | CR4_OSXMMEXCPT;

	asm_cpuid(1, 0, eax, ebx, ecx, edx);
	// Enable XSAVE
	if (ecx & CPUID_1C_XSAVE)
	{
		// To access XCR0, this bit needs to be set beforehand.
		cr4 |= CR4_OSXSAVE;
		asm_set_register(cr4, cr4);

		u64 xcr0 = 0;
		xcr0 |= (u64)1 << 0;
		xcr0 |= (u64)1 << 1;

		// Enable AVX
		if (ecx & CPUID_1C_AVX)
			xcr0 |= (u64)1 << 2;

		asm_cpuid(7, 0, eax, ebx, ecx, edx);
		// Enable AVX-512
		if (ebx & CPUID_7B_AVX512F)
		{
			xcr0 |= (u64)1 << 5;
			xcr0 |= (u64)1 << 6;
			xcr0 |= (u64)1 << 7;
		}

		asm_wrxcr(0, xcr0);

		asm_cpuid(13, 0, eax, ebx, ecx, edx);

		cpu->fpu_size = ecx;
		cpu->fpu_save = asm_fpu_xsave;
		cpu->fpu_restore = asm_fpu_xrstor;
	}
	else
	{
		cpu->fpu_size = 512;
		cpu->fpu_save = asm_fpu_fxsave;
		cpu->fpu_restore = asm_fpu_fxrstor;
	}

	asm_cpuid(7, 0, eax, ebx, ecx, edx);
	// Enable UMIP
	if (ecx & CPUID_7C_UMIP)
		cr4 |= CR4_UMIP;
	// Enable SMEP
	if (ebx & CPUID_7B_SMEP)
		cr4 |= CR4_SMEP;
	// Enable SMAP
	if (ebx & CPUID_7B_SMAP)
	{
		cr4 |= CR4_SMAP;
		can_smap = true;
	}
	// Enable FSGSBASE
	if (ebx & CPUID_7B_FSGSBASE)
	{
		cr4 |= CR4_FSGSBASE;
		asm_wrmsr(MSR_KERNEL_GS_BASE, 0);
		asm_wrmsr(MSR_GS_BASE, (u64)cpu);
		asm_wrmsr(MSR_FS_BASE, 0);
	}

	// Write to the control registers.
	asm_set_register(cr0, cr0);
	asm_set_register(cr4, cr4);

	// We are present!
	cpu->is_present = true;
	atomic_fetch_add(&boot_info->cpu_active, 1);

	if (cpu->id != boot->id)
	{
		// TODO: Init local APIC.
		spin_free(&cpu_lock);

		// Stop all other cores.
		asm_interrupt_disable();
		while (1)
			asm_halt();
	}

	spin_free(&cpu_lock);
}

void arch_early_init(BootInfo* info)
{
	asm_interrupt_disable();

	gdt_init();
	idt_init();
	serial_init();

	boot_info = info;
}

void arch_init(BootInfo* info)
{
	apic_init();

	asm_interrupt_enable();
}

void arch_shutdown(BootInfo* info)
{
	arch_stop(info);
}

ATTR(noreturn) void arch_stop(BootInfo* info)
{
	asm_interrupt_disable();
	while (true)
		asm_halt();
}

Cpu* arch_current_cpu()
{
#ifdef CONFIG_smp
	// The Cpu struct starts at KERNEL_GSBASE:0
	// Since we can't "directly" access the base address, just get the first field (Cpu.id)
	// and use that to index into the CPU array.
	u64 id;
	asm volatile("mov %%gs:(0), %0" : "=r"(id) : "i"(offsetof(Cpu, id)) : "memory");
	return &boot_info->cpus[id];
#else
	return &boot_info->cpus[0];
#endif
}

void arch_get_registers(Context* regs)
{
	if (regs == NULL)
		return;

	asm_get_register(regs->rax, rax);
	asm_get_register(regs->rbx, rbx);
	asm_get_register(regs->rcx, rcx);
	asm_get_register(regs->rdx, rdx);
	asm_get_register(regs->rsi, rsi);
	asm_get_register(regs->rdi, rdi);
	asm_get_register(regs->rbp, rbp);
	asm_get_register(regs->rsp, rsp);
	asm_get_register(regs->r8, r8);
	asm_get_register(regs->r9, r9);
	asm_get_register(regs->r10, r10);
	asm_get_register(regs->r11, r11);
	asm_get_register(regs->r12, r12);
	asm_get_register(regs->r13, r13);
	asm_get_register(regs->r14, r14);
	asm_get_register(regs->r15, r15);
	asm_get_register(regs->cs, cs);
	asm_get_register(regs->ss, ss);
}

void arch_dump_registers(Context* regs)
{
	kmesg("rax: 0x%p rbx: 0x%p rcx: 0x%p rdx: 0x%p\n", regs->rax, regs->rbx, regs->rcx, regs->rdx);
	kmesg("rsi: 0x%p rdi: 0x%p rbp: 0x%p rsp: 0x%p\n", regs->rsi, regs->rdi, regs->rbp, regs->rsp);
	kmesg("r8:  0x%p r9:  0x%p r10: 0x%p r11: 0x%p\n", regs->r8, regs->r9, regs->r10, regs->r11);
	kmesg("r12: 0x%p r13: 0x%p r14: 0x%p r15: 0x%p\n", regs->r12, regs->r13, regs->r14, regs->r15);
	kmesg("core:0x%p isr: 0x%p err: 0x%p rip: 0x%p\n", regs->core, regs->isr, regs->error, regs->rip);
	kmesg("cs:  0x%p rfl: 0x%p ss:  0x%p\n", regs->cs, regs->rflags, regs->ss);
}
