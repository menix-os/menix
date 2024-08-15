// PCI management

#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>

#include <errno.h>

#ifdef CONFIG_acpi
#include <menix/drv/pci/pci_acpi.h>
#endif

PciPlatform pci_platform = {0};
PciDriver* pci_drivers[256];
usize pci_num_drivers = 0;

void pci_init()
{
#if defined(CONFIG_acpi)
	pci_init_acpi();
#elif defined(CONFIG_open_firmware)
// Do PCI configuration using device tree.
#elif defined(CONFIG_arch_x86)
// Do PCI configuration using legacy port IO.
#else
#error "Have no method of accessing a PCI bus! We need either ACPI, OpenFirmware or x86."
#endif
}

i32 pci_register_driver(PciDriver* driver)
{
	if (!driver)
		return -ENOENT;
	if (!driver->variants || driver->num_variants == 0)
		return -ENOENT;

	pci_drivers[pci_num_drivers] = driver;
	pci_num_drivers++;

	kmesg("Registered PCI driver \"%s\" with %u variant(s).\n", driver->name, driver->num_variants);
	return 0;
}

void pci_unregister_driver(PciDriver* driver)
{
	kassert(driver != NULL, "Can't unregister PCI driver: None given!\n");
}
