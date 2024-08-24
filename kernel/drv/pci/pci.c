// PCI management

#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/util/list.h>

#include <errno.h>

#ifdef CONFIG_acpi
#include <menix/drv/pci/pci_acpi.h>
#endif

PciPlatform pci_platform = {0};

static List(PciDriver*) pci_drivers;
static List(PciDevice*) pci_devices;

void pci_init()
{
	list_new(pci_drivers, 128);
	list_new(pci_devices, 128);
}

void pci_fini()
{
	// Remove all PCI devices.
	list_iter(&pci_devices, dev_iter)
	{
		PciDevice* const dev = *dev_iter;

		if (dev->driver && dev->driver->remove)
			dev->driver->remove(dev);

		kfree(dev);
	}

	// Release the device list.
	list_free(&pci_drivers);

	// Release the driver list.
	list_free(&pci_drivers);
}

PciDevice* pci_scan_device(u8 bus, u8 slot)
{
	u16 vendor_id = pci_platform.read16(0, bus, slot, 0, 0x0);

	// If no device is present, return null.
	if (vendor_id == 0xFFFF)
		return NULL;

	// Otherwise allocate memory.
	PciDevice* device = kmalloc(sizeof(PciDevice));

	device->vendor = vendor_id;
	device->device = pci_platform.read16(0, bus, slot, 0, 0x2);
	device->sub_class = pci_platform.read8(0, bus, slot, 0, 0x10);
	device->class = pci_platform.read8(0, bus, slot, 0, 0x11);
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

	// Push the new driver to the list of known drivers.
	list_push(&pci_drivers, driver);

	// Now match all devices to this driver.
	list_iter(&pci_devices, dev_iter)
	{
		PciDevice* const dev = *dev_iter;

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

			kmesg("Matched driver \"%s\" to live device %hx:%hx on %hhu:%hhu\n", driver->name, dev->vendor, dev->device,
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

	kmesg("Registered PCI driver \"%s\" with %zu variant(s).\n", driver->name, driver->num_variants);
	return 0;
}

void pci_unregister_driver(PciDriver* driver)
{
	kassert(driver != NULL, "Can't unregister PCI driver: None given!\n");

	// Check if the driver was registered.
	isize idx;
	list_find(&pci_drivers, idx, driver);

	// If we couldn't find the driver.
	if (idx == -1)
	{
		kmesg("Can't unregister PCI driver \"%s\": Driver was not previously registered!\n", driver->name);
		return;
	}

	// Find all devices matched to this driver.
	list_iter(&pci_devices, dev_iter)
	{
		PciDevice* const dev = *dev_iter;

		// Keep looking if there is no device present.
		if (dev == NULL)
			continue;

		// Keep looking if the device has a different driver (or none) matched to it.
		if (dev->driver != driver)
			continue;

		// Here we can be certain that this device is matched by our driver.
		// Clean up if we're able to.
		if (driver->remove)
			driver->remove(dev);
	}

	list_pop(&pci_drivers, idx);

	kmesg("Unregistered PCI driver \"%s\"\n", driver->name);
}

i32 pci_register_device(PciDevice* device)
{
	if (device == NULL)
		return -ENOENT;

	list_push(&pci_devices, device);

	kmesg("New PCI device %hx:%hx on %hhu:%hhu\n", device->vendor, device->device, device->bus, device->slot);

	return 0;
}
