// PCI(e) interface using ACPI for configuration.

#pragma once
#include <menix/common.h>

// Do PCI configuration using ACPI "MCFG". This is the preferred way.
void mcfg_init();

// Returns the MMIO address of the configuration space of the given card.
PhysAddr mcfg_get_cfg_addr(u16 segment, u16 bus, u8 slot, u8 function);
