// PCI(e) interface using ACPI for configuration.

#pragma once
#include <menix/common.h>

// Reads from a PCI device using the ACPI MCFG table info.
u32 pci_read_acpi(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size);

// Writes to a PCI device using the ACPI MCFG table info.
void pci_write_acpi(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size, u32 value);
