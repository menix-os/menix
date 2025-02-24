// x86 thread spawning

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>

#include <gdt.h>

void thread_arch_setup(Thread* target, VirtAddr start, bool is_user, VirtAddr stack)
{
	target->is_user = is_user;
	target->registers.rip = start;

	// Allocate kernel stack for this thread.
	target->kernel_stack = (VirtAddr)kmalloc(VM_KERNEL_STACK_SIZE);
	// Stack grows down, so move to the end of the allocated memory.
	target->kernel_stack += VM_KERNEL_STACK_SIZE;

	const usize page_size = vm_get_page_size(VMLevel_Small);

	// Allocate memory for the FPU state.
	target->saved_fpu = pm_alloc(ROUND_UP(arch_current_cpu()->fpu_size, page_size)) + pm_get_phys_base();
	memset(target->saved_fpu, 0, arch_current_cpu()->fpu_size);

	Process* proc = target->parent;
	if (is_user)
	{
		target->registers.cs = offsetof(Gdt, user_code64) | CPL_USER;
		target->registers.ss = offsetof(Gdt, user_data) | CPL_USER;

		// Check if we have to allocate a stack.
		if (stack == 0)
		{
			PhysAddr phys_stack = pm_alloc(VM_USER_STACK_SIZE / page_size);
			target->stack = VM_USER_STACK_BASE - VM_USER_STACK_SIZE;
			for (usize i = 0; i < VM_USER_STACK_SIZE / page_size; i++)
			{
				// Map all stack pages.
				vm_map(proc->page_map, phys_stack + (i * page_size), target->stack + (i * page_size),
					   VMProt_Read | VMProt_Write, VMFlags_User, VMLevel_Small);
			}

			target->registers.rsp = target->stack + VM_USER_STACK_SIZE;
		}
		else
		{
			target->registers.rsp = stack;
			target->stack = target->registers.rsp;
		}

		FxState* state = target->saved_fpu;
		state->fcw = 0b1100111111;
		state->mxcsr = 0b1111110000000;

		target->fs_base = 0;
		target->gs_base = 0;
	}
	else
	{
		target->registers.cs = offsetof(Gdt, kernel_code) | CPL_KERNEL;
		target->registers.ss = offsetof(Gdt, kernel_data) | CPL_KERNEL;

		// Load kernel stack.
		target->stack = target->kernel_stack;
		target->registers.rsp = target->stack;

		target->fs_base = asm_rdmsr(MSR_FS_BASE);
		target->gs_base = asm_rdmsr(MSR_KERNEL_GS_BASE);
	}

	target->registers.rflags = 0x202;	 // Interrupt enable
}

void thread_arch_destroy(Thread* thread)
{
	kfree((void*)(thread->kernel_stack - VM_KERNEL_STACK_SIZE));
	CpuInfo* info = arch_current_cpu();
	pm_free(thread->saved_fpu - pm_get_phys_base(), ROUND_UP(info->fpu_size, arch_page_size));
}

void thread_arch_fork(Thread* forked, Thread* original)
{
	forked->fs_base = original->fs_base;
	forked->gs_base = original->gs_base;

	CpuInfo* cpu = arch_current_cpu();

	// Allocate FPU memory.
	PhysAddr fpu_pages = pm_alloc(ROUND_UP(cpu->fpu_size, vm_get_page_size(VMLevel_Small)));
	forked->saved_fpu = pm_get_phys_base() + fpu_pages;
	memcpy(forked->saved_fpu, original->saved_fpu, cpu->fpu_size);

	// Return SYSCALL_OK(0) to the forked syscall process.
	forked->registers.rax = 0;
	forked->registers.rdx = 0;
}
