// Process creation

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/process.h>

bool process_execute(const char* path, char** argv, char** envp)
{
	Process* proc = kzalloc(sizeof(Process));

	// TODO

	return true;
}
