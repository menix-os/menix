// PCI configuration using the MCFG table.

#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/io/mmio.h>
#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/mcfg.h>
#include <menix/system/acpi/types.h>
#include <menix/util/log.h>

AcpiMcfg* acpi_mcfg;

void mcfg_init()
{
	acpi_mcfg = acpi_find_table("MCFG", 0);
	pci_platform.pci_read8 = mcfg_read8;
	pci_platform.pci_read16 = mcfg_read16;
	pci_platform.pci_read32 = mcfg_read32;
	pci_platform.pci_write8 = mcfg_write8;
	pci_platform.pci_write16 = mcfg_write16;
	pci_platform.pci_write32 = mcfg_write32;

	// There are some x86 systems that don't have a MCFG table.
	// In that case we can still use the port IO to configure the devices as a fallback.
	// On the other hand, if we aren't on x86 and there is no MCFG table, we cannot configure PCI.
	if (acpi_mcfg == NULL)
	{
		pci_log("Unable to configure PCI system using ACPI: The MCFG table was not present. ");
#ifdef CONFIG_arch_x86_64
		kmesg("Falling back to x86 mode.\n");
		// TODO: Set function callbacks to x86.
#else
		kmesg("Disable the PCI subsystem with `pci=0;` to continue.\n");
		kabort();
#endif
		return;
	}

	const usize num_entries = (acpi_mcfg->header.length - sizeof(AcpiMcfg)) / sizeof(AcpiMcfgEntry);
	list_new(pci_platform.buses, num_entries);

	// Scan all buses for devices.
	for (usize i = 0; i < num_entries; i++)
	{
		PciBus* bus = kzalloc(sizeof(PciBus));
		bus->id = i;

		list_push(&pci_platform.buses, bus);
	}

	pci_log("Configured PCI using ACPI.\n");
}

#define implement_read(type, name, fn) \
	type name(u16 seg, u8 bus, u8 slot, u8 func, u16 offset) \
	{ \
		const usize num_entries = (acpi_mcfg->header.length - sizeof(AcpiMcfg)) / sizeof(AcpiMcfgEntry); \
		for (usize i = 0; i < num_entries; i++) \
		{ \
			AcpiMcfgEntry* entry = &acpi_mcfg->entries[i]; \
			if (entry->segment_group != seg) \
				continue; \
			if (bus < entry->bus_start && bus > entry->bus_end) \
				continue; \
			void* addr = (void*)ACPI_ADDR( \
				((entry->base + (((bus - entry->bus_start) << 20) | (slot << 15) | (func << 12))) | offset)); \
			return fn(addr); \
		} \
		return 0; \
	}

#define implement_write(type, name, fn) \
	void name(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, type value) \
	{ \
		const usize num_entries = (acpi_mcfg->header.length - sizeof(AcpiMcfg)) / sizeof(AcpiMcfgEntry); \
		for (usize i = 0; i < num_entries; i++) \
		{ \
			AcpiMcfgEntry* entry = &acpi_mcfg->entries[i]; \
			if (entry->segment_group != seg) \
				continue; \
			if (bus < entry->bus_start && bus > entry->bus_end) \
				continue; \
			void* addr = (void*)ACPI_ADDR( \
				((entry->base + (((bus - entry->bus_start) << 20) | (slot << 15) | (func << 12))) | offset)); \
			fn(addr, value); \
		} \
	}

implement_read(u8, mcfg_read8, mmio_read8);
implement_read(u16, mcfg_read16, mmio_read16);
implement_read(u32, mcfg_read32, mmio_read32);

implement_write(u8, mcfg_write8, mmio_write8);
implement_write(u16, mcfg_write16, mmio_write16);
implement_write(u32, mcfg_write32, mmio_write32);
