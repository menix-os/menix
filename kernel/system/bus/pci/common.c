// PCI management

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/system/pci/pci.h>
#include <menix/util/list.h>
#include <menix/util/log.h>

PciPlatform pci_platform = {0};

void pci_init()
{
	list_new(pci_drivers, 32);
	list_new(pci_devices, 32);
	pci_scan_devices();
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

const char* pci_get_class_name(u8 class)
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
