// Process creation

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/process.h>

Process* process_create(const char* path)
{
	Process* proc = kalloc(sizeof(Process));

	// TODO

	return proc;
}
