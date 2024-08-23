// PCI(e) driver abstraction

#pragma once
#include <menix/common.h>
#include <menix/drv/device.h>

#define PCI_ANY_ID			 (~0)
#define PCI_DEVICE(ven, dev) .vendor = (ven), .device = (dev), .sub_vendor = PCI_ANY_ID, .sub_device = PCI_ANY_ID

typedef struct
{
	u32 (*internal_read)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size);
	void (*internal_write)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 access_size, u32 value);
} PciPlatform;

extern PciPlatform pci_platform;

typedef struct PciDriver PciDriver;

// Describes a PCI(e) device.
typedef struct
{
	u16 vendor, device;			   // Vendor and device ID.
	u16 sub_vendor, sub_device;	   // Subsystem IDs.
	u8 class, sub_class;		   // PCI class.
	bool is_pcie;				   // True if the device supports PCIe.
	u8 bus, slot;				   // The bus and slot this device lives on.

	PciDriver* driver;	  // The driver managing this device.
	usize variant_idx;	  // Index into a driver-defined structure array.
} PciDevice;

// A PCI(e) driver with callbacks.
typedef struct PciDriver
{
	const char* name;			  // Name of the device.
	const PciDevice* variants;	  // Array of device variants that the driver can match.
	const usize num_variants;	  // Amount of entries in the `variants` array.

	// Called when a new device is being connected. Returns 0 if successful.
	i32 (*probe)(PciDevice* dev);
	// Called when a device is being removed. (Optional).
	void (*remove)(PciDevice* dev);
	// Called to put a device to sleep. Returns 0 if successful.
	i32 (*suspend)(PciDevice* dev);
	// Called to wake it back up again. Returns 0 if successful.
	i32 (*resume)(PciDevice* dev);
	// Called to deinitialize a device during shutdown.
	void (*shutdown)(PciDevice* dev);
} PciDriver;

// Initializes the PCI subsystem.
void pci_init();

// Shuts the PCI subsystem down. This also unregisters all devices!
void pci_fini();

// Gets the PCI device information in a `slot` on a `bus`. Returns NULL if no device is connected.
PciDevice* pci_scan_device(u8 bus, u8 slot);

// Registers a driver. Returns 0 on success.
i32 pci_register_driver(PciDriver* driver);

// Unregisters a driver. Also unregisters all connected devices.
void pci_unregister_driver(PciDriver* driver);

// Registers a devie. Returns 0 on success.
i32 pci_register_device(PciDevice* device);

// Unregisters a device. Calls the `remove` callback if set.
void pci_unregister_device(PciDevice* device);
