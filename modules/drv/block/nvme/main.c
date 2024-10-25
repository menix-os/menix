// NVMe block devices.

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>

typedef struct
{
	u16 ver_maj, ver_min, ver_ter;
} NvmeDevice;

MODULE_FN i32 nvme_probe(PciDevice* device)
{
	NvmeDevice* nvme = kzalloc(sizeof(NvmeDevice));

	PhysAddr paddr = ((PhysAddr)device->generic.bar[1] << 32) | (device->generic.bar[0] & 0xFFFFFFF0);
	u8* vaddr = paddr + pm_get_phys_base();

	memcpy(&nvme->ver_maj, vaddr + 10, sizeof(u16));
	memcpy(&nvme->ver_min, vaddr + 9, sizeof(u8));
	memcpy(&nvme->ver_ter, vaddr + 8, sizeof(u8));
	pci_log_dev(device, "NVMe version: %hu.%hu.%hu\n", nvme->ver_maj, nvme->ver_min, nvme->ver_ter);

	return 0;
}

// Match all NVMe storage devices.
static PciVariant nvme_class = {PCI_CLASS2(1, 8)};
static PciDriver nvme_driver = {
	.name = MODULE_NAME,
	.variants = &nvme_class,
	.num_variants = 1,
	.probe = nvme_probe,
};

MODULE_FN i32 init_fn()
{
	pci_register_driver(&nvme_driver);
	return 0;
}

MODULE_DEFAULT(init_fn, NULL);
