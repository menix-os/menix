// Kernel entry point

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/log.h>
#include <menix/module.h>
#include <menix/sys/syscall_list.h>
#include <menix/util/list.h>

#include <sys/utsname.h>

#ifdef CONFIG_acpi
#include <menix/drv/acpi/acpi.h>
#ifdef CONFIG_pci
#include <menix/drv/pci/pci_acpi.h>
#endif
#endif

void kernel_main(BootInfo* info)
{
	vfs_init();
#ifdef CONFIG_pci
	pci_init();
#endif
#ifdef CONFIG_acpi
	acpi_init(info->acpi_rsdp);
	// The PCI subsystem depends on ACPI. Now we can enable it.
#ifdef CONFIG_pci
	pci_init_acpi();
#endif
#endif

	// Initialize all modules.
	module_init(info);

	// Say hello to the console.
	struct utsname uname;
	syscall_uname((usize)&uname, 0, 0, 0, 0, 0);
	kmesg("%s %s %s (%s)\n", uname.sysname, uname.release, uname.version, uname.machine);

	// TODO: Call init program.
	// exec("/usr/init");

	// Clean up all modules.
	module_fini();

#ifdef CONFIG_pci
	pci_fini();
#endif

	kmesg("shutdown\n");	// Say goodbye.
}
