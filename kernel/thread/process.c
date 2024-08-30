// Process creation

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/proc.h>
#include <menix/thread/spin.h>

#include <string.h>

SpinLock lock = spin_new();
usize pid_counter = 0;

bool proc_execve(const char* path, char** argv, char** envp)
{
	Process* proc = kzalloc(sizeof(Process));

	// TODO

	return true;
}

usize proc_fork(Process* proc, Thread* thread)
{
	spin_acquire_force(&lock);

	Process* fork = kzalloc(sizeof(Process));
	fixed_strncpy(fork->name, proc->name);

	fork->id = pid_counter++;
	fork->parent = proc;
	fork->working_dir = proc->working_dir;

	fork->children = (ProcessList) {0};
	fork->threads = (ThreadList) {0};

	spin_free(&lock);
	return fork->id;
}
