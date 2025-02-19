#include <menix/common.h>
#include <menix/system/pci/pci.h>

#include <uapi/errno.h>

PciDriverList pci_drivers;

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
			const PciVariant* const var = &driver->variants[variant];
			const volatile PciConfigSpace* cfg = dev->config_space;

			// Check if this driver is a generic class driver. If it isn't, check if the IDs match.
			if (var->vendor != (u16)PCI_ANY_ID && var->vendor != cfg->vendor)
				continue;
			if (var->device != (u16)PCI_ANY_ID && var->device != cfg->device)
				continue;

			// If it's generic make sure the class types match.
			if (var->has_class && var->class != cfg->class)
				continue;
			if (var->has_sub_class && var->sub_class != cfg->sub_class)
				continue;
			if (var->has_prog_if && var->prog_if != cfg->prog_if)
				continue;

			// Connect the driver to the device.
			dev->driver = driver;
			// Copy over the variant index so the driver knows which device was matched.
			dev->variant_idx = driver->variants[variant].variant_idx;

			pci_log_dev(dev, "Matched driver \"%s\" to device!\n", driver->name);

			// Now, probe the device using the registered driver.
			if (dev->driver->probe == NULL)
			{
				print_log("pci: Driver \"%s\" has no probe function! Registration failed.\n", driver->name);
				return -ENOENT;
			}

			i32 ret = dev->driver->probe(dev);
			// Probing failed, the driver is probably faulty so disable it.
			if (ret != 0)
			{
				pci_log_dev(dev, "Probing device has failed with error code %i!\n", ret);
				dev->driver = NULL;
			}
		}
	}

	print_log("pci: Registered PCI driver \"%s\" with %zu variant(s).\n", driver->name, driver->num_variants);
	return 0;
}

void pci_unregister_driver(PciDriver* driver)
{
	kassert(driver != NULL, "Can't unregister PCI driver: None given!");

	// Check if the driver was registered.
	isize idx;
	list_find(&pci_drivers, idx, driver);

	// If we couldn't find the driver.
	if (idx == -1)
	{
		print_log("pci: Can't unregister PCI driver \"%s\": Driver was not previously registered!\n", driver->name);
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

	print_log("pci: Unregistered PCI driver \"%s\"\n", driver->name);
}
