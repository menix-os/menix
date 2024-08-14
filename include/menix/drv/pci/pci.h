// PCI(e) driver abstraction

#pragma once
#include <menix/common.h>

#define PCI_ANY_ID			 (~0)
#define PCI_DEVICE(ven, dev) .vendor = (ven), .device = (dev), .sub_vendor = PCI_ANY_ID, .sub_device = PCI_ANY_ID

// Describes a variant of a PCI(e) device.
typedef struct
{
	u32 vendor, device;			   // Vendor and device ID.
	u32 sub_vendor, sub_device;	   // Subsystem IDs.
	u32 class, class_mask;		   // PCI class.
	usize variant_idx;			   // Index into a driver-defined structure array.
} PciDeviceVariant;

// Describes a PCI(e) device.
typedef struct
{
	// TODO
} PciDevice;

// A PCI(e) driver with callbacks.
typedef struct
{
	const char* name;			   // Name of the device.
	PciDeviceVariant* variants;	   // Array of device variants that the driver can match.
	usize num_variants;			   // Amount of entries in the `variants` array.

	// Called when a new device is being connected. Returns 0 if successful.
	i32 (*probe)(PciDevice* dev, const PciDeviceVariant* id);
	// Called when a device is being removed. (Optional).
	void (*remove)(PciDevice* dev);
	// Called to put a device to sleep. Returns 0 if successful.
	i32 (*suspend)(PciDevice* dev);
	// Called to wake it back up again. Returns 0 if successful.
	i32 (*resume)(PciDevice* dev);
	// Called to deinitialize a device during shutdown.
	void (*shutdown)(PciDevice* dev);
} PciDriver;

// Initializes the PCI subsystem and calls .probe on all registered and matching drivers.
void pci_init();

// Shuts the PCI subsystem down. This also unregisters all devices!
void pci_fini();

// TODO: Move this to dynamically allocated memory!
extern PciDriver* pci_drivers[256];
extern usize pci_num_drivers;

// Registers a driver.
i32 pci_register_driver(PciDriver* driver);
