// Process creation

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/process.h>
#include <menix/thread/spin.h>

#include <string.h>

static SpinLock lock = spin_new();
static usize pid_counter = 0;

void proc_create(char* name, ProcessState state, usize ip, bool is_user, Process* parent)
{
	spin_acquire_force(&lock);

	Process* proc = kzalloc(sizeof(Process));
	strncpy(proc->name, name, sizeof(proc->name));

	spin_free(&lock);
}

bool proc_execve(const char* path, char** argv, char** envp)
{
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
