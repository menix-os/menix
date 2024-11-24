#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/system/pci/pci.h>

PciDeviceList pci_devices;

i32 pci_register_device(PciDevice* device)
{
	if (device == NULL)
		return -ENOENT;
	list_push(&pci_devices, device);
	pci_log_dev(device, "%s\n", pci_get_class_name(device->class));

	return 0;
}

static void pci_scan_device(PciSlot* slot, u8 fn)
{
	if (slot == NULL)
		return;

	const u8 slot_id = slot->id;
	const u8 bus_id = slot->bus->id;

	const u16 vendor_id = PCI_READ16(0, bus_id, slot_id, fn, 0x0);

	// If no device is present, return null.
	if (vendor_id == 0xFFFF)
		return;

	// Otherwise allocate memory.
	PciDevice* device = kmalloc(sizeof(PciDevice));

	// Read common header.
	device->vendor = vendor_id;
	device->device = PCI_READ16(0, bus_id, slot_id, fn, 0x2);
	device->command = PCI_READ16(0, bus_id, slot_id, fn, 0x4);
	device->status = PCI_READ16(0, bus_id, slot_id, fn, 0x6);
	device->revision = PCI_READ8(0, bus_id, slot_id, fn, 0x8);
	device->prog_if = PCI_READ8(0, bus_id, slot_id, fn, 0x9);
	device->sub_class = PCI_READ8(0, bus_id, slot_id, fn, 0xA);
	device->class = PCI_READ8(0, bus_id, slot_id, fn, 0xB);
	device->cache_line_size = PCI_READ8(0, bus_id, slot_id, fn, 0xC);
	device->latency_timer = PCI_READ8(0, bus_id, slot_id, fn, 0xD);
	device->header_type = PCI_READ8(0, bus_id, slot_id, fn, 0xE);
	device->bist = PCI_READ8(0, bus_id, slot_id, fn, 0xF);

	// Read header based on the type.
	switch (device->header_type & PCI_TYPE_MASK)
	{
		case PCI_TYPE_GENERIC:
		{
			device->generic.bar[0] = PCI_READ32(0, bus_id, slot_id, fn, 0x10);
			device->generic.bar[1] = PCI_READ32(0, bus_id, slot_id, fn, 0x14);
			device->generic.bar[2] = PCI_READ32(0, bus_id, slot_id, fn, 0x18);
			device->generic.bar[3] = PCI_READ32(0, bus_id, slot_id, fn, 0x1C);
			device->generic.bar[4] = PCI_READ32(0, bus_id, slot_id, fn, 0x20);
			device->generic.bar[5] = PCI_READ32(0, bus_id, slot_id, fn, 0x24);
			device->generic.cardbus_cis = PCI_READ32(0, bus_id, slot_id, fn, 0x28);
			device->generic.sub_vendor = PCI_READ16(0, bus_id, slot_id, fn, 0x2C);
			device->generic.sub_device = PCI_READ16(0, bus_id, slot_id, fn, 0x2E);
			device->generic.expansion_rom = PCI_READ32(0, bus_id, slot_id, fn, 0x30);
			device->generic.capabilities = PCI_READ8(0, bus_id, slot_id, fn, 0x34);
			device->generic.int_line = PCI_READ8(0, bus_id, slot_id, fn, 0x3C);
			device->generic.int_pin = PCI_READ8(0, bus_id, slot_id, fn, 0x3D);
			device->generic.min_grant = PCI_READ8(0, bus_id, slot_id, fn, 0x3E);
			device->generic.max_latency = PCI_READ8(0, bus_id, slot_id, fn, 0x3F);
			break;
		}
		case PCI_TYPE_PCI_BRIDGE:
		{
			device->pci_bridge.bar[0] = PCI_READ32(0, bus_id, slot_id, fn, 0x10);
			device->pci_bridge.bar[1] = PCI_READ32(0, bus_id, slot_id, fn, 0x14);
			device->pci_bridge.bus_primary = PCI_READ8(0, bus_id, slot_id, fn, 0x18);
			device->pci_bridge.bus_secondary = PCI_READ8(0, bus_id, slot_id, fn, 0x19);
			device->pci_bridge.bus_subordinate = PCI_READ8(0, bus_id, slot_id, fn, 0x1A);
			device->pci_bridge.latency_timer2 = PCI_READ8(0, bus_id, slot_id, fn, 0x1B);
			device->pci_bridge.io_base = PCI_READ8(0, bus_id, slot_id, fn, 0x1C);
			device->pci_bridge.io_limit = PCI_READ8(0, bus_id, slot_id, fn, 0x1D);
			device->pci_bridge.status2 = PCI_READ16(0, bus_id, slot_id, fn, 0x1E);
			device->pci_bridge.mem_base = PCI_READ16(0, bus_id, slot_id, fn, 0x20);
			device->pci_bridge.mem_limit = PCI_READ16(0, bus_id, slot_id, fn, 0x22);
			device->pci_bridge.pre_base = PCI_READ16(0, bus_id, slot_id, fn, 0x24);
			device->pci_bridge.pre_limit = PCI_READ16(0, bus_id, slot_id, fn, 0x26);
			device->pci_bridge.pre_base_upper = PCI_READ32(0, bus_id, slot_id, fn, 0x28);
			device->pci_bridge.pre_limit_upper = PCI_READ32(0, bus_id, slot_id, fn, 0x2C);
			device->pci_bridge.io_base_upper = PCI_READ16(0, bus_id, slot_id, fn, 0x30);
			device->pci_bridge.io_limit_upper = PCI_READ16(0, bus_id, slot_id, fn, 0x32);
			device->pci_bridge.capabilities = PCI_READ8(0, bus_id, slot_id, fn, 0x34);
			device->pci_bridge.expansion_rom = PCI_READ32(0, bus_id, slot_id, fn, 0x38);
			device->pci_bridge.int_line = PCI_READ8(0, bus_id, slot_id, fn, 0x3C);
			device->pci_bridge.int_pin = PCI_READ8(0, bus_id, slot_id, fn, 0x3D);
			device->pci_bridge.bridge_control = PCI_READ16(0, bus_id, slot_id, fn, 0x3E);
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

	device->dev = kzalloc(sizeof(Device));
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
