// uDRM Bridge

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>

#include <udrm/kernel_api.h>
#include <udrm/udrm.h>

// Bochs GPU
#include <udrm/bochs.h>

static i32 bochs_probe(PciDevice* dev)
{
	return udrm_bochs_probe(dev);
}

static void bochs_remove(PciDevice* dev)
{
	udrm_bochs_remove(dev);
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
	.remove = bochs_remove,
};

static i32 init_fn()
{
	uapi_status status = udrm_initialize();
	if (status != UAPI_STATUS_OK)
	{
		print_error("uDRM has failed to initialize!\n");
		return status;
	}

	// Register all device drivers.
	i32 pci_status;

	pci_status = pci_register_driver(&bochs_driver);
	if (pci_status != 0)
	{
		print_error("Failed to register the Bochs PCI driver.\n");
		return pci_status;
	}

	return 0;
}

MODULE_DEFAULT(init_fn, NULL);
