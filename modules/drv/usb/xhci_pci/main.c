#include <menix/common.h>
#include <menix/drv/pci/pci.h>
#include <menix/log.h>
#include <menix/module.h>

MODULE_FN i32 xhci_pci_probe(PciDevice* dev)
{
	// TODO
	return 0;
}

static const PciVariant xhci_match = {PCI_CLASS3(0xC, 0x3, 0x30)};

static PciDriver driver = {
	.name = MODULE_NAME,
	.variants = &xhci_match,
	.num_variants = 1,
	.probe = xhci_pci_probe,
};

MODULE_FN i32 init_fn()
{
	return pci_register_driver(&driver);
}

MODULE_FN void exit_fn()
{
	pci_unregister_driver(&driver);
}

MODULE_DEFAULT(init_fn, exit_fn, "usb_core");
