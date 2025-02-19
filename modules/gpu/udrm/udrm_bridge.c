// uDRM Bridge

#include <menix/common.h>
#include <menix/memory/mmio.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>

#include <udrm/core/udrm.h>
#include <udrm/kernel_api.h>

// Virtio GPU
#include <udrm/drivers/virtio.h>

static i32 virtio_probe(PciDevice* dev)
{
	return udrm_virtio_probe(dev);
}

static void virtio_remove(PciDevice* dev)
{
	udrm_virtio_remove(dev);
}

static PciVariant virtio_variant = {
	.vendor = 0x1af4,
	.device = 0x1050,
};

static PciDriver virtio_driver = {
	.name = "udrm_virtio",
	.variants = &virtio_variant,
	.num_variants = 1,
	.probe = virtio_probe,
	.remove = virtio_remove,
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

	pci_status = pci_register_driver(&virtio_driver);
	if (pci_status != 0)
	{
		print_error("Failed to register the VirtIO PCI driver.\n");
		return pci_status;
	}

	return 0;
}

MODULE_DEFAULT(init_fn, NULL);
