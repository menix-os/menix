// PCI driver registration

#include <menix/drv/pci/pci.h>
#include <menix/log.h>

#include <errno.h>

PciDriver* pci_drivers[256];
usize pci_num_drivers = 0;

i32 pci_register_driver(PciDriver* driver)
{
	if (!driver)
		return -ENOENT;
	if (!driver->variants || driver->num_variants == 0)
		return -ENOENT;

	// TODO Make this better.
	pci_drivers[pci_num_drivers] = driver;
	pci_num_drivers++;

	kmesg("Registered PCI driver \"%s\" with %i variant(s).\n", driver->name, driver->num_variants);
	return 0;
}
