// PCI driver model

#pragma once

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>

// Log PCI related messages.
#define pci_log(fmt, ...) kmesg("[PCI] " fmt, ##__VA_ARGS__)
#define pci_err(fmt, ...) kmesg("[PCI] " fmt, ##__VA_ARGS__)

typedef struct
{
	u16 vendor_id;
	u16 device_id;
	u8 class;
	u8 subclass;
} PciDevice;

const char* pci_get_class_name(const PciDevice* pci);

// Read 16 bits from a PCI device.
u16 pci_read16(u8 bus, u8 slot, u8 func, u8 offset);

// Get the info of a connected device.
PciDevice pci_get_info(u8 bus, u8 slot);

// Initializes the PCI subsystem.
void pci_init();

// Shuts the PCI subsystem down.
void pci_fini();
