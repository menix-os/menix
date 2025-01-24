// Interrupt handlers for x86_64 exceptions.

#include <menix/syscall/syscall.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/log.h>

#include <apic.h>
#include <idt.h>

Context* interrupt_ud_handler(usize isr, Context* regs, void* data)
{
	// Make sure we're in user mode, otherwise we have to crash.
	print_log("Invalid opcode at 0x%zx on core %zu!\n", regs->rip, arch_current_cpu()->id);
	print_log("Faulty data:");

	for (usize i = 0; i < 16; i++)
	{
		u8 instr = *(u8*)regs->rip;
		print_log(" %hhx", instr);
	}

	ktrace(regs);
	kabort();

	return regs;
}

// Handles the syscall interrupt. Also referenced by system/arch.s
Context* syscall_handler(usize isr, Context* regs, void* data)
{
	// Save the registers.
	CpuInfo* const core = arch_current_cpu();
	Thread* const thread = core->thread;
	thread->registers = *regs;
	thread->stack = core->user_stack;
	sch_arch_save(core, thread);

	// Execute the system call. For x86_64, use the SysV kernel ABI.
	SyscallResult result = syscall_invoke(regs->rax, regs->rdi, regs->rsi, regs->rdx, regs->r10, regs->r8, regs->r9);
	regs->rax = result.value;
	regs->rdx = result.error;

	return regs;
}
