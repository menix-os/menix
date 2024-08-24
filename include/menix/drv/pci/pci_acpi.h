// PCI(e) interface using ACPI for configuration.

#pragma once
#include <menix/common.h>

// Do PCI configuration using ACPI "MCFG". This is the preferred way.
void pci_init_acpi();

// Reads 8 bits from a PCI device using the ACPI MCFG table info.
u8 mcfg_read8(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
// Reads 16 bits from a PCI device using the ACPI MCFG table info.
u16 mcfg_read16(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
// Reads 32 bits from a PCI device using the ACPI MCFG table info.
u32 mcfg_read32(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);

// Writes 8 bits to a PCI device using the ACPI MCFG table info.
void mcfg_write8(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 value);
// Writes 16 bits to a PCI device using the ACPI MCFG table info.
void mcfg_write16(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u16 value);
// Writes 32 bits to a PCI device using the ACPI MCFG table info.
void mcfg_write32(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u32 value);
