#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>
#include <menix/module.h>

// Driver structure.
typedef struct
{
} VirtIoGpuDevice;

static PciDevice id_table[] = {
	{PCI_DEVICE(0x1234, 0x1111), .variant_idx = 0},
};

static i32 virtio_gpu_probe(PciDevice* device)
{
	return 0;
}

static PciDriver virtio_gpu = {
	.name = MODULE_NAME,
	.variants = id_table,
	.num_variants = ARRAY_SIZE(id_table),
	.probe = virtio_gpu_probe,
};

MODULE_FN i32 init_fn()
{
	return pci_register_driver(&virtio_gpu);
}

MODULE_FN void exit_fn()
{
	pci_unregister_driver(&virtio_gpu);
}

MODULE = {
	.name = MODULE_NAME,
	.init = init_fn,
	.exit = exit_fn,
	MOULE_META_COMMON,
};
