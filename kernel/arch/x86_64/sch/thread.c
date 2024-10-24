// x86 thread spawning

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/thread.h>

#include <gdt.h>

void thread_setup(Thread* target, VirtAddr start, bool is_user, VirtAddr stack)
{
	target->registers.rip = start;

	// Allocate kernel stack for this thread.
	target->kernel_stack = (VirtAddr)kmalloc(CONFIG_kernel_stack_size);
	// Stack grows down, so move to the end of the allocated memory.
	target->kernel_stack += CONFIG_kernel_stack_size;

	// Allocate memory for the FPU state.
	target->saved_fpu = pm_alloc(ROUND_UP(arch_current_cpu()->fpu_size, arch_page_size)) + pm_get_phys_base();
	memset(target->saved_fpu, 0, arch_current_cpu()->fpu_size);

	Process* proc = target->parent;

	if (is_user)
	{
		target->registers.cs = offsetof(Gdt, user_code64) | CPL_USER;
		target->registers.ss = offsetof(Gdt, user_data) | CPL_USER;

		// Check if we have to allocate a stack.
		if (stack == 0)
		{
			PhysAddr phys_stack = pm_alloc(CONFIG_user_stack_size / arch_page_size);
			target->stack = phys_stack;
			for (usize i = 0; i < CONFIG_user_stack_size / arch_page_size; i++)
			{
				// Map all stack pages.
				vm_map(proc->page_map, phys_stack + (i * arch_page_size),
					   (proc->stack_top - CONFIG_user_stack_size) + (i * arch_page_size), VMProt_Read | VMProt_Write,
					   VMFlags_User, VMLevel_0);
			}

			target->registers.rsp = proc->stack_top;
			proc->stack_top -= CONFIG_user_stack_size;
		}
		else
		{
			target->registers.rsp = stack;
			target->stack = target->registers.rsp;
		}

		// TODO: Prepare FPU. This seems to cause a GPF. idk why
		// arch_current_cpu()->fpu_restore(target->saved_fpu);
		// u16 default_fcw = 0b1100111111;
		// asm volatile("fldcw %0" ::"m"(default_fcw) : "memory");
		// u32 default_mxcsr = 0b1111110000000;
		// asm volatile("ldmxcsr %0" ::"m"(default_mxcsr) : "memory");
		// arch_current_cpu()->fpu_save(target->saved_fpu);

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

void thread_destroy(Thread* thread)
{
	kfree((void*)(thread->kernel_stack - CONFIG_kernel_stack_size));
	pm_free(thread->saved_fpu - pm_get_phys_base(), ROUND_UP(arch_current_cpu()->fpu_size, arch_page_size));
}
