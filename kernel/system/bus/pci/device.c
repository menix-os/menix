#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/system/pci/pci.h>

PciDeviceList pci_devices;

i32 pci_register_device(PciDevice* device)
{
	if (device == NULL)
		return -ENOENT;
	list_push(&pci_devices, device);
	pci_log_dev(device, "%s\n", pci_get_class_name(device->config_space->class));

	return 0;
}

static void pci_scan_device(PciSlot* slot, u8 fn)
{
	if (slot == NULL)
		return;

	const u8 slot_id = slot->id;
	const u8 bus_id = slot->bus->id;

	// The MMIO config space address of this PCI device.
	PhysAddr config_space_phys = pci_platform.get_cfg_addr(0, bus_id, slot_id, fn);
	volatile PciConfigSpace* config_space = pm_get_phys_base() + config_space_phys;

	// If no device is present, return null.
	if (config_space->vendor == 0xFFFF)
		return;

	// Otherwise allocate memory.
	PciDevice* device = kmalloc(sizeof(PciDevice));
	device->slot = slot;
	device->config_space = config_space;

	// Scan all other functions if multi-function bit is set.
	if (fn == 0 && config_space->header_type & PCI_TYPE_MF_MASK)
	{
		for (usize f = 1; f < 8; f++)
			pci_scan_device(slot, f);
	}

	// Handle PCI bridges.
	if ((config_space->header_type & PCI_TYPE_MASK) == PCI_TYPE_PCI_BRIDGE)
	{
		pci_log_dev(device, "PCI-to-PCI bridge: Primary = %hhx, Secondary = %hhx, Subordinate = %hhx\n",
					config_space->pci_bridge.bus_primary, config_space->pci_bridge.bus_secondary,
					config_space->pci_bridge.bus_subordinate);
	}

	// Register the device.
	if (pci_register_device(device) != 0)
		pci_log_dev(device, "Failed to register PCI device!\n");

	slot->devices[fn] = device;

	device->dev = kzalloc(sizeof(Device));
}

void pci_scan_devices()
{
	print_log("pci: Scanning devices.\n");
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
