// PCI configuration using the MCFG table.

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/mcfg.h>
#include <menix/system/acpi/types.h>
#include <menix/system/pci/pci.h>
#include <menix/util/log.h>

#include <uacpi/acpi.h>
#include <uacpi/tables.h>

struct acpi_mcfg* mcfg;

void mcfg_init()
{
	uacpi_table mcfg_table;

	// There are some systems that don't have a MCFG table.
	// If there is no MCFG table, we cannot configure PCI using ACPI.
	if (uacpi_table_find_by_signature(ACPI_MCFG_SIGNATURE, &mcfg_table) != UACPI_STATUS_OK)
	{
		print_log("pci: Unable to configure PCI system using ACPI: The MCFG table was not present.\n");
		print_log("Disable the PCI subsystem with `pci=0` or use a device tree to continue booting.\n");
		kabort();
	}

	mcfg = mcfg_table.ptr;
	pci_platform.get_cfg_addr = mcfg_get_cfg_addr;

	const usize num_entries = (mcfg->hdr.length - sizeof(struct acpi_mcfg)) / sizeof(struct acpi_mcfg_allocation);
	list_new(pci_platform.buses, num_entries);

	// Scan all buses for devices.
	for (usize i = 0; i < num_entries; i++)
	{
		PciBus* bus = kzalloc(sizeof(PciBus));
		bus->id = i;

		list_push(&pci_platform.buses, bus);
	}

	print_log("pci: Configured PCI using ACPI.\n");

	pci_init();
	uacpi_table_unref(&mcfg_table);
}

PhysAddr mcfg_get_cfg_addr(u16 segment, u16 bus, u8 slot, u8 function)
{
	const usize num_entries = (mcfg->hdr.length - sizeof(struct acpi_mcfg)) / sizeof(struct acpi_mcfg_allocation);
	for (usize i = 0; i < num_entries; i++)
	{
		struct acpi_mcfg_allocation* entry = &mcfg->entries[i];
		if (entry->segment != segment)
			continue;
		if (bus < entry->start_bus && bus > entry->end_bus)
			continue;
		return (PhysAddr)(entry->address + (((bus - entry->start_bus) << 20) | (slot << 15) | (function << 12)));
	}
	return 0;
}
