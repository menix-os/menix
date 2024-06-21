//? PCI IO implementation

#include <menix/drv/pci.h>
#include <menix/log.h>

void pci_init()
{
	for (int i = 0; i < 4; i++)
	{
		PciDevice d = pci_get_info(0, i);
		klog(LOG_DEBUG, "%x, %x\n", d.class, d.subclass);
	}
}

void pci_fini()
{
}

uint16_t pci_read16(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset)
{
	uint32_t address;
	uint32_t lbus = (uint32_t)bus;
	uint32_t lslot = (uint32_t)slot;
	uint32_t lfunc = (uint32_t)func;
	uint16_t tmp = 0;

	// Create configuration address as per Figure 1
	address = (uint32_t)((lbus << 16) | (lslot << 11) | (lfunc << 8) | (offset & 0xFC) | ((uint32_t)0x80000000));

	// Write out the address
	write32(0xCF8, address);
	// Read in the data
	// (offset & 2) * 8) = 0 will choose the first word of the 32-bit register
	tmp = (uint16_t)((read32(0xCFC) >> ((offset & 2) * 8)) & 0xFFFF);
	return tmp;
}

PciDevice pci_get_info(uint8_t bus, uint8_t slot)
{
	PciDevice result = {
		.vendor_id = pci_read16(bus, slot, 0, 0),
		.device_id = pci_read16(bus, slot, 0, 2),
		.class = pci_read16(bus, slot, 0, 12),
		.subclass = pci_read16(bus, slot, 0, 16),
	};

	if (result.vendor_id == 0xFFFF)
		klog(LOG_WARN, "Non-existant PCI device\n");

	return result;
}
