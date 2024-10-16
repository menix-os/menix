// PCI management

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/system/pci/pci.h>
#include <menix/util/list.h>
#include <menix/util/log.h>

PciPlatform pci_platform = {0};

static List(PciDriver*) pci_drivers;
static List(PciDevice*) pci_devices;

void pci_init()
{
	list_new(pci_drivers, 32);
	list_new(pci_devices, 32);
	pci_scan_devices();
}

static void pci_scan_device(PciSlot* slot, u8 fn)
{
	if (slot == NULL)
		return;

	const u8 slot_id = slot->id;
	const u8 bus_id = slot->bus->id;

	u16 vendor_id = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x0);

	// If no device is present, return null.
	if (vendor_id == 0xFFFF)
		return;

	// Otherwise allocate memory.
	PciDevice* device = kmalloc(sizeof(PciDevice));

	// Read common header.
	device->vendor = vendor_id;
	device->device = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x2);
	device->command = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x4);
	device->status = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x6);
	device->revision = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x8);
	device->prog_if = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x9);
	device->sub_class = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0xA);
	device->class = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0xB);
	device->cache_line_size = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0xC);
	device->latency_timer = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0xD);
	device->header_type = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0xE);
	device->bist = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0xF);

	// Read header based on the type.
	switch (device->header_type & PCI_TYPE_MASK)
	{
		case PCI_TYPE_GENERIC:
		{
			device->generic.bar[0] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x10);
			device->generic.bar[1] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x14);
			device->generic.bar[2] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x18);
			device->generic.bar[3] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x1C);
			device->generic.bar[4] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x20);
			device->generic.bar[5] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x24);
			device->generic.cardbus_cis = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x28);
			device->generic.sub_vendor = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x2C);
			device->generic.sub_device = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x2E);
			device->generic.expansion_rom = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x30);
			device->generic.capabilities = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x34);
			device->generic.int_line = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x3C);
			device->generic.int_pin = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x3D);
			device->generic.min_grant = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x3E);
			device->generic.max_latency = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x3F);
			break;
		}
		case PCI_TYPE_PCI_BRIDGE:
		{
			device->pci_bridge.bar[0] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x10);
			device->pci_bridge.bar[1] = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x14);
			device->pci_bridge.bus_primary = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x18);
			device->pci_bridge.bus_secondary = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x19);
			device->pci_bridge.bus_subordinate = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x1A);
			device->pci_bridge.latency_timer2 = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x1B);
			device->pci_bridge.io_base = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x1C);
			device->pci_bridge.io_limit = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x1D);
			device->pci_bridge.status2 = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x1E);
			device->pci_bridge.mem_base = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x20);
			device->pci_bridge.mem_limit = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x22);
			device->pci_bridge.pre_base = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x24);
			device->pci_bridge.pre_limit = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x26);
			device->pci_bridge.pre_base_upper = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x28);
			device->pci_bridge.pre_limit_upper = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x2C);
			device->pci_bridge.io_base_upper = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x30);
			device->pci_bridge.io_limit_upper = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x32);
			device->pci_bridge.capabilities = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x34);
			device->pci_bridge.expansion_rom = pci_platform.pci_read32(0, bus_id, slot_id, fn, 0x38);
			device->pci_bridge.int_line = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x3C);
			device->pci_bridge.int_pin = pci_platform.pci_read8(0, bus_id, slot_id, fn, 0x3D);
			device->pci_bridge.bridge_control = pci_platform.pci_read16(0, bus_id, slot_id, fn, 0x3E);
			break;
		}
		default:
		{
			pci_log_dev(device, "Unsupported header type %hhu, skipping!\n", device->header_type);
			return;
		}
	}

	device->slot = slot;
	device->function = fn;

	// Scan all other functions if multi-function bit is set.
	if (fn == 0 && device->header_type & PCI_TYPE_MF_MASK)
	{
		for (usize f = 1; f < 8; f++)
			pci_scan_device(slot, f);
	}

	// Handle PCI bridges.
	if ((device->header_type & PCI_TYPE_MASK) == PCI_TYPE_PCI_BRIDGE)
	{
		pci_log_dev(device, "PCI-to-PCI bridge: Primary = %hhx, Secondary = %hhx, Subordinate = %hhx\n",
					device->pci_bridge.bus_primary, device->pci_bridge.bus_secondary,
					device->pci_bridge.bus_subordinate);
	}

	// Register the device.
	if (pci_register_device(device) != 0)
		pci_log_dev(device, "Failed to register PCI device!\n");

	slot->devices[fn] = device;
}

void pci_scan_devices()
{
	list_iter(&pci_platform.buses, bus)
	{
		for (usize slot = 0; slot < PCI_MAX_SLOTS; slot++)
		{
			// Prepare a slot.
			PciSlot* s = (*bus)->slots + slot;
			s->bus = *bus;
			s->id = slot;
			pci_scan_device(s, 0);
		}
	}
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

	// Release the bus list.
	list_free(&pci_platform.buses);
}

static const char* pci_get_class_name(u8 class)
{
	switch (class)
	{
		case 0x01: return "Mass Storage Controller";
		case 0x02: return "Network Controller";
		case 0x03: return "Display Controller";
		case 0x04: return "Multimedia Controller";
		case 0x05: return "Memory Controller";
		case 0x06: return "Bridge";
		case 0x07: return "Simple Communication Controller";
		case 0x08: return "Base System Peripheral";
		case 0x09: return "Input Device Controller";
		case 0x0A: return "Docking Station";
		case 0x0B: return "Processor";
		case 0x0C: return "Serial Bus Controller";
		case 0x0D: return "Wireless Controller";
		case 0x0E: return "Intelligent Controller";
		case 0x0F: return "Satellite Communication Controller";
		case 0x10: return "Encryption Controller";
		case 0x11: return "Signal Processing Controller";
		case 0x12: return "Processing Accelerator";
		case 0x13: return "Non-Essential Instrumentation";
		case 0x40: return "Co-Processor";
		case 0xFF: return "Unassigned";
	}
	return "Unclassified";
}

void pci_print_devices()
{
	list_iter(&pci_devices, dev_iter)
	{
		PciDevice* const dev = *dev_iter;
		pci_log_dev(dev, "%s\n", pci_get_class_name(dev->class));
	}
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
			const PciVariant* const var = &driver->variants[variant];

			// Check if this driver is a generic class driver. If it isn't, check if the IDs match.
			if (var->vendor != (u16)PCI_ANY_ID && var->vendor != dev->vendor)
				continue;
			if (var->device != (u16)PCI_ANY_ID && var->device != dev->device)
				continue;

			// If it's generic make sure the class types match.
			if (var->has_class && var->class != dev->class)
				continue;
			if (var->has_sub_class && var->sub_class != dev->sub_class)
				continue;
			if (var->has_prog_if && var->prog_if != dev->prog_if)
				continue;

			// Connect the driver to the device.
			dev->driver = driver;
			// Copy over the variant index so the driver knows which device was matched.
			dev->variant_idx = driver->variants[variant].variant_idx;

			pci_log_dev(dev, "Matched driver \"%s\" to device!\n", driver->name);

			// Now, probe the device using the registered driver.
			if (dev->driver->probe == NULL)
			{
				pci_log("Driver \"%s\" has no probe function! Registration failed.\n", driver->name);
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

	pci_log("Registered PCI driver \"%s\" with %zu variant(s).\n", driver->name, driver->num_variants);
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
		pci_log("Can't unregister PCI driver \"%s\": Driver was not previously registered!\n", driver->name);
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

	pci_log("Unregistered PCI driver \"%s\"\n", driver->name);
}

i32 pci_register_device(PciDevice* device)
{
	if (device == NULL)
		return -ENOENT;

	list_push(&pci_devices, device);

	pci_log_dev(device, "Registered new %s!\n", pci_get_class_name(device->class));

	return 0;
}
