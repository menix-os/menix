// PCI configuration using ACPI

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
	pci_platform.internal_read = pci_read_acpi;
	pci_platform.internal_write = pci_write_acpi;

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

	// TODO: Read PCI buses.

	kmesg("Successfully onfigured PCI using ACPI.\n");
}

// Reads from a PCI device using the ACPI MCFG table info.
u32 pci_read_acpi(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size)
{
	u32 ret = 0;
	const usize num_entries = (acpi_mcfg->header.length - sizeof(AcpiMcfg)) / sizeof(AcpiMcfgEntry);
	for (usize i = 0; i < num_entries; i++)
	{
		AcpiMcfgEntry* entry = &acpi_mcfg->entries[i];
		if (entry->segment_group == seg)
		{
			if (bus >= entry->bus_start && bus <= entry->bus_end)
			{
				void* addr = (void*)ACPI_ADDR(
					((entry->base + (((bus - entry->bus_start) << 20) | (slot << 15) | (func << 12))) | offset));
				switch (access_size)
				{
					case sizeof(u8): return read8(addr);
					case sizeof(u16): return read16(addr);
					case sizeof(u32): return read32(addr);
					default: kmesg("PCI: Tried to read %u bytes, but this is out of range!\n", access_size); break;
				}
			}
		}
	}
	return ret;
}

// Writes to a PCI device using the ACPI MCFG table info.
void pci_write_acpi(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size, u32 value)
{
	const usize num_entries = (acpi_mcfg->header.length - sizeof(AcpiMcfg)) / sizeof(AcpiMcfgEntry);
	for (usize i = 0; i < num_entries; i++)
	{
		AcpiMcfgEntry* entry = &acpi_mcfg->entries[i];
		if (entry->segment_group == seg)
		{
			if (bus >= entry->bus_start && bus <= entry->bus_end)
			{
				void* addr = (void*)ACPI_ADDR(
					((entry->base + (((bus - entry->bus_start) << 20) | (slot << 15) | (func << 12))) | offset));
				switch (access_size)
				{
					case sizeof(u8): write8(addr, value);
					case sizeof(u16): write16(addr, value);
					case sizeof(u32): write32(addr, value);
					default: kmesg("PCI: Tried to write %u bytes, but this is out of range!\n", access_size); break;
				}
			}
		}
	}
}
