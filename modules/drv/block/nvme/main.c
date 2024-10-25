// NVMe block devices.

#include <menix/common.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>

MODULE_FN i32 pci_probe(PciDevice* device)
{
	return 0;
}

// Match all NVMe storage devices.
static PciVariant nvme_class = {PCI_CLASS2(1, 8)};

static PciDriver nvme_driver = {
	.name = MODULE_NAME,
	.variants = &nvme_class,
	.num_variants = 1,
	.probe = pci_probe,
};

MODULE_FN i32 init_fn()
{
	pci_register_driver(&nvme_driver);
	return 0;
}

MODULE_DEFAULT(init_fn, NULL);
