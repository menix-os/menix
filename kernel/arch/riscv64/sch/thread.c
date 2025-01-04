#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/thread.h>

void thread_setup(Thread* target, VirtAddr start, bool is_user, VirtAddr stack)
{
	target->is_user = is_user;
	target->registers.pc = start;

	// Allocate kernel stack for this thread.
	target->kernel_stack = (VirtAddr)kmalloc(VM_KERNEL_STACK_SIZE);
	// Stack grows down, so move to the end of the allocated memory.
	target->kernel_stack += VM_KERNEL_STACK_SIZE;

	// TODO
}

void thread_destroy(Thread* thread)
{
	// TODO
}
