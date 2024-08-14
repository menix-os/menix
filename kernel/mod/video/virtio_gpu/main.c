#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>
#include <menix/module.h>

static PciDeviceVariant test_variants[] = {
	{PCI_DEVICE(0x1234, 0x1111), .variant_idx = 0},
};

static PciDriver test_driver = {
	.name = "QEMU VGA Controller",
	.variants = test_variants,
	.num_variants = ARRAY_SIZE(test_variants),
};

MODULE_FN i32 init_fn()
{
	return pci_register_driver(&test_driver);
}

MODULE_FN void exit_fn()
{
}

MODULE = {
	.name = MODULE_NAME,
	.init = init_fn,
	.exit = exit_fn,
	MOULE_META_COMMON,
};
