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

	pci_platform.pci_read8 = mcfg_read8;
	pci_platform.pci_read16 = mcfg_read16;
	pci_platform.pci_read32 = mcfg_read32;
	pci_platform.pci_write8 = mcfg_write8;
	pci_platform.pci_write16 = mcfg_write16;
	pci_platform.pci_write32 = mcfg_write32;

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
}

#define implement_read(type, name, fn) \
	type name(u16 seg, u8 bus, u8 slot, u8 func, u16 offset) \
	{ \
		const usize num_entries = (mcfg->hdr.length - sizeof(struct acpi_mcfg)) / sizeof(struct acpi_mcfg_allocation); \
		for (usize i = 0; i < num_entries; i++) \
		{ \
			struct acpi_mcfg_allocation* entry = &mcfg->entries[i]; \
			if (entry->segment != seg) \
				continue; \
			if (bus < entry->start_bus && bus > entry->end_bus) \
				continue; \
			void* addr = (void*)ACPI_ADDR( \
				((entry->address + (((bus - entry->start_bus) << 20) | (slot << 15) | (func << 12))) | offset)); \
			return fn(addr); \
		} \
		return 0; \
	}

#define implement_write(type, name, fn) \
	void name(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, type value) \
	{ \
		const usize num_entries = (mcfg->hdr.length - sizeof(struct acpi_mcfg)) / sizeof(struct acpi_mcfg_allocation); \
		for (usize i = 0; i < num_entries; i++) \
		{ \
			struct acpi_mcfg_allocation* entry = &mcfg->entries[i]; \
			if (entry->segment != seg) \
				continue; \
			if (bus < entry->start_bus && bus > entry->end_bus) \
				continue; \
			void* addr = (void*)ACPI_ADDR( \
				((entry->address + (((bus - entry->start_bus) << 20) | (slot << 15) | (func << 12))) | offset)); \
			fn(addr, value); \
		} \
	}

implement_read(u8, mcfg_read8, mmio_read8);
implement_read(u16, mcfg_read16, mmio_read16);
implement_read(u32, mcfg_read32, mmio_read32);

implement_write(u8, mcfg_write8, mmio_write8);
implement_write(u16, mcfg_write16, mmio_write16);
implement_write(u32, mcfg_write32, mmio_write32);
