// x86 thread spawning

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/thread.h>

#include <gdt.h>

void thread_setup(Thread* target, VirtAddr start, bool is_user, VirtAddr stack)
{
	target->registers.rip = start;
	target->kernel_stack = (VirtAddr)kmalloc(CONFIG_kernel_stack_size);
	target->kernel_stack += CONFIG_kernel_stack_size;
	target->saved_fpu = pm_alloc(ROUND_UP(arch_current_cpu()->fpu_size, CONFIG_page_size)) + pm_get_phys_base();
	memset(target->saved_fpu, 0, arch_current_cpu()->fpu_size);

	Process* proc = target->parent;

	if (is_user)
	{
		target->registers.cs = offsetof(Gdt, user_code64) | CPL_USER;
		target->registers.ss = offsetof(Gdt, user_data) | CPL_USER;

		// Create stack.
		if (stack == 0)
		{
			target->registers.rsp = pm_alloc(CONFIG_user_stack_size / CONFIG_page_size);
			target->stack = target->registers.rsp;
			vm_map(proc->page_map, proc->stack_top - CONFIG_user_stack_size, CONFIG_user_stack_size,
				   PROT_READ | PROT_WRITE, MAP_ANONYMOUS, NULL, 0);

			target->registers.rsp = proc->stack_top;
		}
		else
		{
			target->registers.rsp = stack;
			target->stack = target->registers.rsp;
		}

		// Prepare FPU.
		arch_current_cpu()->fpu_restore(target->saved_fpu);
		u16 default_fcw = 0b1100111111;
		asm volatile("fldcw %0" ::"m"(default_fcw) : "memory");
		u32 default_mxcsr = 0b1111110000000;
		asm volatile("ldmxcsr %0" ::"m"(default_mxcsr) : "memory");
		arch_current_cpu()->fpu_save(target->saved_fpu);

		target->fs_base = 0;
		target->gs_base = 0;
	}
	else
	{
		target->registers.cs = offsetof(Gdt, kernel_code) | CPL_KERNEL;
		target->registers.cs = offsetof(Gdt, kernel_data) | CPL_KERNEL;

		// Load kernel stack.
		target->stack = target->kernel_stack;
		target->registers.rsp = target->stack;

		target->fs_base = asm_rdmsr(MSR_FS_BASE);
		target->gs_base = asm_rdmsr(MSR_KERNEL_GS_BASE);
	}

	target->registers.rflags = 0x202;	 // Interrupt enable
}

void thread_setup_execve(Thread* target, VirtAddr start, char** argv, char** envp)
{
	thread_setup(target, start, true, 0);

	Process* proc = target->parent;

	// Stack layout starting at CONFIG_user_stack_addr:
	// envp data
	// argv data
	// - 16 byte alignment -
	// auxval terminator
	// auxvals
	// 0
	// envp[0..n]
	// 0
	// argv[0..n]
	// argc

	void* stack = (void*)(target->stack + CONFIG_user_stack_size + pm_get_phys_base());

	// Copy envp onto the stack.
	usize num_envp;
	for (num_envp = 0; envp[num_envp] != NULL; num_envp++)
	{
		const usize envp_strlen = strlen(envp[num_envp]) + 1;
		stack -= envp_strlen;
		memcpy(stack, envp[num_envp], envp_strlen);
	}
	VirtAddr envp_addr =
		proc->stack_top + (target->stack + CONFIG_user_stack_size) - ((PhysAddr)(stack - pm_get_phys_base()));

	// Copy argv onto the stack.
	usize num_argv;
	for (num_argv = 0; argv[num_argv] != NULL; num_argv++)
	{
		const usize argv_strlen = strlen(argv[num_argv]) + 1;
		stack -= argv_strlen;
		memcpy(stack, argv[num_argv], argv_strlen);
	}
	VirtAddr argv_addr =
		proc->stack_top + (target->stack + CONFIG_user_stack_size) - ((PhysAddr)(stack - pm_get_phys_base()));

	// We are now working with pointer-width granularity.
	// Align the stack to a multiple of 16 so it can properly hold pointer data.
	usize* sized_stack = (usize*)ALIGN_DOWN((VirtAddr)stack, 16);

	// auxval terminator
	*(--sized_stack) = 0;
	*(--sized_stack) = 0;

	// TODO: auxvals

	// Set each evnp pointer.
	*(--sized_stack) = 0;		// End of envp (== NULL).
	sized_stack -= num_envp;	// Make room for all envp entries.
	usize offset = 0;
	for (isize i = num_envp - 1; i >= 0; i--)
	{
		if (i != num_envp - 1)
			offset += strlen(envp[i + 1]) + 1;
		sized_stack[i] = envp_addr + offset;
	}

	// Set each argv pointer.
	*(--sized_stack) = 0;		// End of argv (== NULL).
	sized_stack -= num_argv;	// Make room for all argv entries.
	offset = 0;
	for (isize i = num_argv - 1; i >= 0; i--)
	{
		if (i != num_argv - 1)
			offset += strlen(argv[i + 1]) + 1;
		sized_stack[i] = argv_addr + offset;
	}

	// Set argc.
	*(--sized_stack) = num_argv;

	// Update stack start.
	target->registers.rsp -=
		(target->stack + CONFIG_user_stack_size) - (((PhysAddr)sized_stack) - (PhysAddr)pm_get_phys_base());
	proc->stack_top -= CONFIG_user_stack_size;
}

void thread_destroy(Thread* thread)
{
	kfree((void*)(thread->kernel_stack - CONFIG_kernel_stack_size));
	pm_free(thread->saved_fpu - pm_get_phys_base(), ROUND_UP(arch_current_cpu()->fpu_size, CONFIG_page_size));
}
