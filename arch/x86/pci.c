//? x86 specific PCI IO
// TODO: Replace with UEFI PCI access

#include <menix/common.h>
#include <menix/drv/pci.h>

#ifdef CONFIG_pci

uint16_t pci_read16(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset)
{
	uint32_t address = 0x80000000;
	address |= (uint32_t)bus << 16;
	address |= (uint32_t)slot << 11;
	address |= (uint32_t)func << 8;
	address |= offset & 0xfc;

	// Write out the address
	write32(0xcf8, address);
	return (read32(0xcfc) >> ((offset & 2) * 8)) & 0xffff;
}

#endif
