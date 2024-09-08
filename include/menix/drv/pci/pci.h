// PCI(e) driver abstraction

#pragma once
#include <menix/common.h>
#include <menix/drv/device.h>

#define pci_log(fmt, ...) kmesg("[PCI]\t" fmt, ##__VA_ARGS__)

#define PCI_ANY_ID (~0U)
#define PCI_DEVICE(ven, dev) \
	.vendor = (u16)(ven), .device = (u16)(dev), .sub_vendor = (u16)PCI_ANY_ID, .sub_device = (u16)PCI_ANY_ID

#define PCI_CLASS(cl, subcl, prog, is_subcl, is_prog) \
	PCI_DEVICE(PCI_ANY_ID, PCI_ANY_ID), .class = (cl), .sub_class = (subcl), .prog_if = (prog), .has_class = true, \
		.has_sub_class = (is_subcl), .has_prog_if = (is_prog)
#define PCI_CLASS1(cl)				PCI_CLASS(cl, 0, 0, false, false)
#define PCI_CLASS2(cl, subcl)		PCI_CLASS(cl, subcl, 0, true, false)
#define PCI_CLASS3(cl, subcl, prog) PCI_CLASS(cl, subcl, prog, true, true)

// Abstraction for PCI mechanisms. Can be e.g. x86 port IO or ACPI.
typedef struct
{
	u8 (*pci_read8)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
	u16 (*pci_read16)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
	u32 (*pci_read32)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
	void (*pci_write8)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 value);
	void (*pci_write16)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u16 value);
	void (*pci_write32)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u32 value);
} PciPlatform;

typedef struct PciDriver PciDriver;
// Describes a PCI(e) device.
typedef struct
{
	u16 vendor, device;			   // Primary IDs of this device.
	u16 sub_vendor, sub_device;	   // Secondary IDs of this device.
	u16 command, status;
	u8 revision, class, sub_class, prog_if;
	u8 cache_line_size, latency_timer, header_type, bist;

	u8 bus, slot;		  // The bus and slot this device lives on.
	Device* dev;		  // Underlying device.
	PciDriver* driver;	  // The driver managing this device.
	usize variant_idx;	  // Index into a driver-defined structure array.
} PciDevice;

// Drivers can use this to create bindings.
typedef struct
{
	u16 vendor, device;							   // Primary IDs of this device.
	u16 sub_vendor, sub_device;					   // Secondary IDs of this device.
	u8 class, sub_class, prog_if;				   // Class type.
	bool has_class, has_sub_class, has_prog_if;	   // If `true`, then the respective class type is checked.
	usize variant_idx;							   // Index into a driver-defined structure array.
} PciVariant;

// A PCI(e) driver with callbacks.
typedef struct PciDriver
{
	const char* name;			   // Name of the device.
	const PciVariant* variants;	   // Array of device variants that the driver can match.
	const usize num_variants;	   // Amount of entries in the `variants` array.

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

extern PciPlatform pci_platform;

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
