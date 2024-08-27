// Process creation

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/proc.h>

bool proc_exec(const char* path, char** argv, char** envp)
{
	Process* proc = kzalloc(sizeof(Process));

	// TODO

	return true;
}
