// PCI management

#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>

#include <errno.h>

#ifdef CONFIG_acpi
#include <menix/drv/acpi/acpi.h>
#include <menix/drv/acpi/types.h>
#include <menix/drv/pci/pci_acpi.h>
AcpiMcfg* acpi_mcfg;
#endif

PciDriver* pci_drivers[256];
usize pci_num_drivers = 0;

static u32 (*pci_internal_read)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size);
static void (*pci_internal_write)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size, u32 value);

void pci_init()
{
#if defined(CONFIG_acpi)
	// Do PCI configuration using ACPI "MCFG". This is the preferred way.
	acpi_mcfg = acpi_find_table("MCFG", 0);
	pci_internal_read = pci_read_acpi;
	pci_internal_write = pci_write_acpi;
	const usize num_entries = (acpi_mcfg->header.length - sizeof(AcpiMcfg)) / sizeof(AcpiMcfgEntry);
	// TODO: If we don't have a MCFG table, resort to legacy PCI on x86.
#elif defined(CONFIG_open_firmware)
	// Do PCI configuration using device tree.
#elif defined(CONFIG_arch_x86)
	// Do PCI configuration using legacy port IO.
#else
#error "Have no method of accessing a PCI bus!"
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
