// x86 specific PCI IO
// TODO: Replace with UEFI PCI access

#include <menix/common.h>

#ifdef CONFIG_pci

#include <menix/drv/pci.h>

#include <io.h>

void pci_init()
{
	// Scan all buses
	for (u16 bus = 0; bus < 256; bus++)
	{
		for (u8 slot = 0; slot < 32; slot++)
		{
			const PciDevice dev = pci_get_info(bus, slot);
			if (dev.vendor_id != 0xFFFF)
				pci_log("Device %x:%x (vendor: %#x, device: %#x) | %s\n", (u32)bus, (u32)slot, dev.vendor_id,
						dev.device_id, pci_get_class_name(&dev));
		}
	}
	ktrace();
}

void pci_fini()
{
}

static const char* pci_class_names[] = {
	"Unclassified",
	"Mass Storage Controller",
	"Network Controller",
	"Display Controller",
	"Multimedia Controller",
	"Memory Controller",
	"Bridge",
	"Simple Communication Controller",
	"Base System Peripheral",
	"Input Device Controller",
	"Docking Station",
	"Processor",
	"Serial Bus Controller",
	"Wireless Controller",
	"Intelligent Controller",
	"Satellite Communication Controller",
	"Encryption Controller",
	"Signal Processing Controller",
	"Processing Accelerator",
	"Non-Essential Instrumentation",
};

const char* pci_get_class_name(const PciDevice* pci)
{
	kassert(pci->class < ARRAY_SIZE(pci_class_names), "PCI class was out of bounds!");
	return pci_class_names[pci->class];
}

PciDevice pci_get_info(u8 bus, u8 slot)
{
	const PciDevice result = {
		.vendor_id = pci_read16(bus, slot, 0, 0),
		.device_id = pci_read16(bus, slot, 0, 2),
		.subclass = pci_read16(bus, slot, 0, 12),
		.class = pci_read16(bus, slot, 0, 16),
	};

	return result;
}

u16 pci_read16(u8 bus, u8 slot, u8 func, u8 offset)
{
	u32 address = 0x80000000;
	address |= (u32)bus << 16;
	address |= (u32)slot << 11;
	address |= (u32)func << 8;
	address |= offset & 0xfc;

	// Write out the address
	arch_x86_write32(0xcf8, address);
	return (arch_x86_read32(0xcfc) >> ((offset & 2) * 8)) & 0xffff;
}

#endif
