//? PCI IO implementation

#include <menix/common.h>
#include <menix/drv/pci.h>
#include <menix/log.h>

#ifdef CONFIG_pci

void pci_init()
{
	// Scan all buses
	for (uint16_t bus = 0; bus < 256; bus++)
	{
		for (uint8_t slot = 0; slot < 32; slot++)
		{
			const PciDevice dev = pci_get_info(bus, slot);
			if (dev.vendor_id != 0xFFFF)
				pci_log("New device %x:%x (vendor: %#x, device: %#x)\n", (uint32_t)bus, (uint32_t)slot, dev.vendor_id,
						dev.device_id);
		}
	}
}

void pci_fini()
{
}

PciDevice pci_get_info(uint8_t bus, uint8_t slot)
{
	const PciDevice result = {
		.vendor_id = pci_read16(bus, slot, 0, 0),
		.device_id = pci_read16(bus, slot, 0, 2),
		.subclass = pci_read16(bus, slot, 0, 12),
		.class = pci_read16(bus, slot, 0, 16),
	};

	return result;
}

#endif
