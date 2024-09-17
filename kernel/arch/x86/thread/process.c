// x86 process setup

#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>

void process_setup(Process* proc, bool is_user)
{
	if (is_user)
		proc->page_map = vm_page_map_new();
	else
		proc->page_map = vm_get_kernel_map();
}

void process_fork_context(Process* fork, Process* source)
{
	fork->page_map = vm_page_map_fork(source->page_map);
}

void process_destroy(Process* proc)
{
	if (arch_current_cpu()->thread == NULL)
		vm_set_page_map(vm_get_kernel_map());
	vm_page_map_destroy(proc->page_map);
}
