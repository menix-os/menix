// PCI management

#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>

#include <errno.h>

#ifdef CONFIG_acpi
#include <menix/drv/pci/pci_acpi.h>
#endif

PciPlatform pci_platform = {0};

static PciDriver** pci_drivers;
static usize num_drivers = 0;
static usize cap_drivers = 512;

static PciDevice** pci_devices;
static usize num_devices = 0;
static usize cap_devices = 512;

void pci_init()
{
	// Allocate memory for drivers.
	pci_drivers = kzalloc(sizeof(PciDriver*) * cap_drivers);
	// Allocate memory for devices.
	pci_devices = kzalloc(sizeof(PciDevice*) * cap_devices);
}

void pci_fini()
{
	// Remove all PCI devices.
	for (usize i = 0; i < num_devices; i++)
	{
		if (!pci_devices[i])
			continue;
		if (!pci_devices[i]->driver)
			continue;
		if (!pci_devices[i]->driver->remove)
			continue;

		pci_devices[i]->driver->remove(pci_devices[i]);
	}

	// Release the device list.
	kfree(pci_devices);

	// Release the driver list.
	kfree(pci_drivers);
}

PciDevice* pci_scan_device(u8 bus, u8 slot)
{
	u16 vendor_id = pci_platform.internal_read(0, bus, slot, 0, 0x0, sizeof(u16));

	// If no device is present, return null.
	if (vendor_id == 0xFFFF)
		return NULL;

	// Otherwise allocate memory.
	PciDevice* device = kalloc(sizeof(PciDevice));

	device->vendor = vendor_id;
	device->device = pci_platform.internal_read(0, bus, slot, 0, 0x2, sizeof(u16));
	device->sub_class = pci_platform.internal_read(0, bus, slot, 0, 0x10, sizeof(u8));
	device->class = pci_platform.internal_read(0, bus, slot, 0, 0x11, sizeof(u8));
	device->bus = bus;
	device->slot = slot;

	return device;
}

i32 pci_register_driver(PciDriver* driver)
{
	if (!driver)
		return -ENOENT;
	if (!driver->variants || driver->num_variants == 0)
		return -ENOENT;

	// TODO: Handle extending cap_drivers.

	// Find a free slot in the drivers list.
	usize free_slot = 0;
	for (usize i = 0; i < cap_drivers; i++)
	{
		if (!pci_drivers[i])
		{
			free_slot = i;
			break;
		}
	}
	pci_drivers[free_slot] = driver;
	num_drivers++;

	// Now match all devices to this driver.
	for (usize i = 0; i < cap_devices; i++)
	{
		// Skip unset entries.
		if (!pci_devices[i])
			continue;

		PciDevice* dev = pci_devices[i];

		// Match all variants to the current device in the list.
		for (usize variant = 0; variant <= driver->num_variants; variant++)
		{
			// If the IDs don't match, skip this.
			if (dev->device != driver->variants[variant].device || dev->vendor != driver->variants[variant].vendor)
				continue;

			// Connect the driver to the device.
			dev->driver = driver;
			// Copy over the variant index so the driver knows which device was matched.
			dev->variant_idx = driver->variants[variant].variant_idx;

			kmesg("Matched driver \"%s\" to live device %x:%x on %u:%u\n", driver->name, dev->vendor, dev->device,
				  dev->bus, dev->slot);

			// Now, probe the device using the registered driver.
			kassert(dev->driver->probe != NULL, "Driver has no probe set!\n");
			i32 ret = dev->driver->probe(dev);
			// Probing failed, the driver is probably faulty so disable it.
			if (ret != 0)
			{
				kmesg("Probing device %x:%x on %u:%u has failed with error code %i!\n", dev->vendor, dev->device,
					  dev->bus, dev->slot, ret);
				dev->driver = NULL;
			}
		}
	}

	kmesg("Registered PCI driver \"%s\" with %u variant(s).\n", driver->name, driver->num_variants);
	return 0;
}

void pci_unregister_driver(PciDriver* driver)
{
	kassert(driver != NULL, "Can't unregister PCI driver: None given!\n");

	PciDriver* match = NULL;

	// Check if the driver was registered.
	for (usize i = 0; i < cap_drivers; i++)
	{
		if (pci_drivers[i] == driver)
		{
			match = driver;
			break;
		}
	}

	// If we couldn't find the driver.
	if (match == NULL)
	{
		kmesg("Can't unregister PCI driver \"%s\": Driver was not previously registered!\n", driver->name);
		return;
	}

	// Find all devices matched to this driver.
	for (usize i = 0; i < cap_devices; i++)
	{
		// Keep looking if there is no device present.
		if (pci_devices[i] == NULL)
			continue;

		// Keep looking if the device has a different driver (or none) matched to it.
		if (pci_devices[i]->driver != driver)
			continue;

		// Here we can be certain that this device is matched by our driver.
		// Clean up if we're able to.
		if (driver->remove)
			driver->remove(pci_devices[i]);
	}

	num_drivers--;

	kmesg("Unregistered PCI driver \"%s\"\n", driver->name);
}

i32 pci_register_device(PciDevice* device)
{
	if (device == NULL)
		return -ENOENT;

	// TODO: Handle extending cap_devices.

	// Find a free slot in the drivers list.
	usize free_slot = 0;
	for (usize i = 0; i < cap_devices; i++)
	{
		if (!pci_devices[i])
		{
			free_slot = i;
			break;
		}
	}
	pci_devices[free_slot] = device;

	kmesg("New PCI device %x:%x on %u:%u\n", device->vendor, device->device, device->bus, device->slot);

	return 0;
}
