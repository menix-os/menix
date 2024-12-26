// uDRM Bridge

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>

#include <udrm/kernel_api.h>
#include <udrm/udrm.h>

static i32 bochs_probe(PciDevice* dev)
{
	uapi_status status = udrm_bochs_probe(dev);
	return status;
}

static PciVariant bochs_variant = {
	.vendor = 0x1234,
	.device = 0x1111,
};

static PciDriver bochs_driver = {
	.name = "udrm_bochs",
	.variants = &bochs_variant,
	.num_variants = 1,
	.probe = bochs_probe,
};

static i32 init_fn()
{
	udrm_initialize();

	// Register all device drivers.
	pci_register_driver(&bochs_driver);

	return 0;
}

MODULE_DEFAULT(init_fn, NULL);
