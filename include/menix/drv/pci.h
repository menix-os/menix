//? PCI driver model

#pragma once

#include <menix/arch.h>
#include <menix/common.h>

#define PCI_UNCLASSIFIED	  0x00
#define PCI_MASS_STORAGE_CTRL 0x01
#define PCI_NETWORK_CTRL	  0x02
#define PCI_DISPLAY_CTRL	  0x03
#define PCI_MULTIMEDIA_CTRL	  0x04

typedef struct
{
	uint16_t vendor_id;
	uint16_t device_id;
	uint8_t class;
	uint8_t subclass;
} PciDevice;

// Read 16 bits from a PCI device.
uint16_t pci_read16(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset);

// Get the info of a connected device.
PciDevice pci_get_info(uint8_t bus, uint8_t slot);

// Initializes the PCI subsystem.
void pci_init();

// Shuts the PCI subsystem down.
void pci_fini();
