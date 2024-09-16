// Cross-platform scheduler implementations

#include <menix/boot.h>
#include <menix/common.h>
#include <menix/thread/process.h>
#include <menix/thread/scheduler.h>

void scheduler_init(BootInfo* info)
{
	process_create("kernel", ProcessState_Ready, (VirtAddr)kernel_main, false, NULL);
}
