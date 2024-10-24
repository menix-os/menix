// PCI(e) driver abstraction

#pragma once
#include <menix/common.h>
#include <menix/system/device.h>
#include <menix/util/list.h>

#define pci_log(fmt, ...) kmesg("[PCI]\t" fmt, ##__VA_ARGS__)
#define pci_log_dev(dev, fmt, ...) \
	kmesg("[PCI %04hx:%04hx on %02hhx:%02hhx.%hhx (%hhu,%hhu,%hhu)]\t" fmt, (dev)->vendor, (dev)->device, \
		  (dev)->slot->bus->id, (dev)->slot->id, (dev)->function, (dev)->class, (dev)->sub_class, (dev)->prog_if, \
		  ##__VA_ARGS__)

#define PCI_ANY_ID (~0U)
#define PCI_DEVICE(ven, dev) \
	.vendor = (u16)(ven), .device = (u16)(dev), .sub_vendor = (u16)PCI_ANY_ID, .sub_device = (u16)PCI_ANY_ID
#define PCI_CLASS(cl, subcl, prog, is_subcl, is_prog) \
	PCI_DEVICE(PCI_ANY_ID, PCI_ANY_ID), .class = (cl), .sub_class = (subcl), .prog_if = (prog), .has_class = true, \
		.has_sub_class = (is_subcl), .has_prog_if = (is_prog)
#define PCI_CLASS1(cl)				PCI_CLASS(cl, 0, 0, false, false)
#define PCI_CLASS2(cl, subcl)		PCI_CLASS(cl, subcl, 0, true, false)
#define PCI_CLASS3(cl, subcl, prog) PCI_CLASS(cl, subcl, prog, true, true)

#define PCI_TYPE_GENERIC	0x00
#define PCI_TYPE_PCI_BRIDGE 0x01
#define PCI_TYPE_MF_MASK	0x80
#define PCI_TYPE_MASK		0x7F

typedef struct PciDriver PciDriver;
typedef struct PciSlot PciSlot;
typedef struct PciBus PciBus;
typedef struct PciDevice PciDevice;

// Represents a PCI(e) device.
struct PciDevice
{
	u8 function;		   // Function index on the current slot.
	u16 vendor, device;	   // Primary IDs of this device.
	u16 command, status;
	u8 revision, class, sub_class, prog_if;
	u8 cache_line_size, latency_timer, header_type, bist;

	// Fields depending on the header type.
	//! CardBus is not supported.
	union
	{
		struct
		{
			u32 bar[6];					   // Base addresses.
			u32 cardbus_cis;			   // CardBus CIS pointer.
			u16 sub_vendor, sub_device;	   // Secondary IDs of this device.
			u32 expansion_rom;			   // Expansion ROM base address.
			u8 capabilities;			   // Capabilities pointer.
			u8 int_line;				   // Interrupt line.
			u8 int_pin;					   // The pin used for the interrupt.
			u8 min_grant;				   // Burst period length (in 0.25 µs units).
			u8 max_latency;				   // How often the device needs to access the PCI bus (in 0.25 µs units).
		} generic;						   // Generic device (0)
		struct
		{
			u32 bar[2];								// Base addresses.
			u8 bus_primary, bus_secondary;			// Bus numbers.
			u8 bus_subordinate;						// Subordinate bus number.
			u8 latency_timer2;						// Secondary latency timer.
			u8 io_base, io_limit;					// IO access bytes.
			u16 status2;							// Secondary status.
			u16 mem_base, mem_limit;				// Memory base and limit.
			u16 pre_base, pre_limit;				// Prefetchable memory base and limit.
			u32 pre_base_upper, pre_limit_upper;	// Prefetchable memory base and limit upper 32 bits.
			u16 io_base_upper, io_limit_upper;		// IO access upper 16 bits.
			u8 capabilities;						// Capabilities pointer.
			u32 expansion_rom;						// Expansion ROM base address.
			u8 int_line;							// Interrupt line.
			u8 int_pin;								// The pin used for the interrupt.
			u16 bridge_control;						// Bridge control number.
		} pci_bridge;								// PCI-to-PCI bridge (1)
	};

	Device* dev;		  // Underlying device.
	PciDriver* driver;	  // The driver managing this device.
	usize variant_idx;	  // Index into a driver-defined structure array.
	PciSlot* slot;		  // The slot this device is on.
};

struct PciSlot
{
	u8 id;					  // Index of this slot.
	PciDevice* devices[8];	  // Devices connected on this slot.
	PciBus* bus;			  // Parent bus of this slot.
};

#define PCI_MAX_SLOTS 32

// Represents a PCI bus.
struct PciBus
{
	u8 id;							 // Index of this bus.
	PciSlot slots[PCI_MAX_SLOTS];	 // Slots connected to this bus.
};

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

// Abstraction for PCI mechanisms. Can be e.g. x86 port IO or ACPI.
typedef struct
{
	u8 (*pci_read8)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
	u16 (*pci_read16)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
	u32 (*pci_read32)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset);
	void (*pci_write8)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u8 value);
	void (*pci_write16)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u16 value);
	void (*pci_write32)(u16 seg, u8 bus, u8 slot, u8 func, u16 offset, u32 value);

	List(PciBus*) buses;
} PciPlatform;

#define PCI_READ8(seg, bus, slot, func, offset)	 pci_platform.pci_read8(seg, bus, slot, func, offset)
#define PCI_READ16(seg, bus, slot, func, offset) pci_platform.pci_read16(seg, bus, slot, func, offset)
#define PCI_READ32(seg, bus, slot, func, offset) pci_platform.pci_read32(seg, bus, slot, func, offset)

#define PCI_WRITE8(seg, bus, slot, func, offset, value)	 pci_platform.pci_read8(seg, bus, slot, func, offset, value)
#define PCI_WRITE16(seg, bus, slot, func, offset, value) pci_platform.pci_read16(seg, bus, slot, func, offset, value)
#define PCI_WRITE32(seg, bus, slot, func, offset, value) pci_platform.pci_read32(seg, bus, slot, func, offset, value)

extern PciPlatform pci_platform;

// Initializes the PCI subsystem.
void pci_init();

// Shuts the PCI subsystem down. This also unregisters all devices!
void pci_fini();

// Scans all PCI buses for devices.
void pci_scan_devices();

// Registers a driver. Returns 0 on success.
i32 pci_register_driver(PciDriver* driver);

// Unregisters a driver. Also unregisters all connected devices.
void pci_unregister_driver(PciDriver* driver);

// Registers a devie. Returns 0 on success.
i32 pci_register_device(PciDevice* device);

// Unregisters a device. Calls the `remove` callback if set.
void pci_unregister_device(PciDevice* device);

// Prints all registered devices.
void pci_print_devices();
