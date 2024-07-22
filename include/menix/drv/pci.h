//? PCI driver model

#pragma once

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>

// Log PCI related messages.
#define pci_log(fmt, ...) kmesg("[PCI] " fmt, ##__VA_ARGS__)
#define pci_err(fmt, ...) kmesg("[PCI] " fmt, ##__VA_ARGS__)

typedef struct
{
	uint16_t vendor_id;
	uint16_t device_id;
	uint8_t class;
	uint8_t subclass;
} PciDevice;

const char* pci_get_class_name(const PciDevice* pci);

// Read 16 bits from a PCI device.
uint16_t pci_read16(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset);

// Get the info of a connected device.
PciDevice pci_get_info(uint8_t bus, uint8_t slot);

// Initializes the PCI subsystem.
void pci_init();

// Shuts the PCI subsystem down.
void pci_fini();
