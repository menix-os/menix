// PCI configuration using ACPI

#include <menix/common.h>

#ifdef CONFIG_acpi
#include <menix/drv/acpi/acpi.h>
#include <menix/drv/acpi/types.h>
#include <menix/drv/pci/pci_acpi.h>
#include <menix/io/mmio.h>
#include <menix/log.h>

extern AcpiMcfg* acpi_mcfg;

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
					default: kmesg("PCI: Tried to read %x bytes, but this is out of range!\n", access_size); break;
				}
			}
		}
	}
	return ret;
}

// Writes to a PCI device using the ACPI MCFG table info.
void pci_write_acpi(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size, u32 value)
{
}

#endif
