// Kernel entry point

#include <menix/common.h>
#include <menix/fs/vfs.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/sys/syscall_list.h>

void kernel_main(BootInfo* info)
{
	vfs_init();

	// Initialize all modules and subsystems.
	module_init(info);

	// Say hello to the console.
	struct utsname uname;
	syscall_uname((usize)&uname, 0, 0, 0, 0, 0);
	kmesg("%s %s %s %s\n", uname.sysname, uname.release, uname.version, uname.machine);

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules and subsystems.
	module_fini();

	// Say goodbye.
	kmesg("shutdown\n");
}
