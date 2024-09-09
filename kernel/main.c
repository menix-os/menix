// Kernel entry point

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/sys/syscall_list.h>
#include <menix/thread/process.h>

void kernel_main(BootInfo* info)
{
	arch_current_cpu()->thread = kzalloc(sizeof(Thread));
	vfs_init();

	// Load initrd(s).
	for (usize i = 0; i < info->file_num; i++)
		ustarfs_init(vfs_get_root(), info->files[i].address, info->files[i].size);

	// Register all terminal devices.
	terminal_init();

	// Initialize all modules and subsystems.
	module_init(info);

	// Say hello to the console.
	struct utsname uname;
	syscall_uname((usize)&uname, 0, 0, 0, 0, 0);
	kmesg("%s %s [%s] %s\n", uname.sysname, uname.release, uname.version, uname.machine);

	// Call init program.
	char* argv[] = {"/boot/init", NULL};
	proc_execve("init", argv, NULL);

	// Clean up all modules and subsystems.
	module_fini();
}
