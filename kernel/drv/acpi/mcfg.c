// PCI configuration using the MCFG table.

#include <menix/common.h>
#include <menix/drv/acpi/acpi.h>
#include <menix/drv/acpi/types.h>
#include <menix/drv/pci/pci.h>
#include <menix/drv/pci/pci_acpi.h>
#include <menix/io/mmio.h>
#include <menix/log.h>

AcpiMcfg* acpi_mcfg;

void pci_init_acpi()
{
	acpi_mcfg = acpi_find_table("MCFG", 0);
	pci_platform.read8 = mcfg_read8;
	pci_platform.read16 = mcfg_read16;
	pci_platform.read32 = mcfg_read32;
	pci_platform.write8 = mcfg_write8;
	pci_platform.write16 = mcfg_write16;
	pci_platform.write32 = mcfg_write32;

	// There are some x86 systems that don't have a MCFG table.
	// In that case we can still use the port IO to configure the devices as a fallback.
	// On the other hand, if we aren't on x86 and there is no MCFG table, we cannot configure PCI.
	if (acpi_mcfg == NULL)
	{
		kmesg("Unable to configure PCI system using ACPI: The MCFG table was not present. ");
#ifdef CONFIG_arch_x86
		kmesg("Falling back to x86 mode.\n");
		// TODO: Set function callbacks to x86.
#else
		kmesg("Disable the PCI subsystem with `pci=0;` to continue.\n");
		kabort();
#endif
		return;
	}

	const usize num_entries = (acpi_mcfg->header.length - sizeof(AcpiMcfg)) / sizeof(AcpiMcfgEntry);

	// Scan all buses for devices.
	for (usize i = 0; i < num_entries; i++)
	{
		for (usize slot = 0; slot < 16; slot++)
		{
			PciDevice* dev = pci_scan_device(i, slot);
			if (!dev)
				continue;
			if (pci_register_device(dev) != 0)
				kmesg("Failed to register PCI device %hx:%hx on bus %hhu, slot %hhu!\n", dev->vendor, dev->device,
					  dev->bus, dev->slot);
		}
	}

	kmesg("Configured PCI using ACPI.\n");
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

implement_read(u8, mcfg_read8, read8);
implement_read(u16, mcfg_read16, read16);
implement_read(u32, mcfg_read32, read32);

implement_write(u8, mcfg_write8, write8);
implement_write(u16, mcfg_write16, write16);
implement_write(u32, mcfg_write32, write32);
