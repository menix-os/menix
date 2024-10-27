// NVMe block devices.

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/device.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>
#include <menix/util/units.h>

#include <block/nvme.h>

static usize nvme_counter = 0;

void nvme_read(NvmeNameSpace* ns, PhysAddr phys, Buffer buffer)
{
	// TODO
}

// Initializes an NVMe controller (See section 3.5.1)
i32 nvme_probe(PciDevice* pdev)
{
	// Allocate device data.
	NvmeController* nvme = kzalloc(sizeof(NvmeController));
	dev_set_data(pdev->dev, nvme);
	nvme->mmio_base =
		pm_get_phys_base() + (((PhysAddr)pdev->generic.bar[1] << 32) | (pdev->generic.bar[0] & 0xFFFFC000));

	// Disable the controller if it wasn't already.
	nvme->regs->cc.en = false;
	// Wait for the controller to indicate that the reset is complete (RDY = 0).
	// TODO: Replace with timeout
	while (nvme->regs->csts & NVME_CS_RDY)
		asm_pause();

	// Get the doorbell stride.
	nvme->doorbell_stride = 4 << nvme->regs->cap.dstrd;

	// Configure Admin Queue by setting AQA, ASQ and ACQ.
	nvme_cq_init(nvme, &nvme->admin_cq, 1, vm_get_page_size(VMLevel_0) / sizeof(NvmeCQEntry));
	nvme_sq_init(nvme, &nvme->admin_sq, &nvme->admin_cq, 0, vm_get_page_size(VMLevel_0) / sizeof(NvmeSQEntry));
	nvme->regs->aqa = nvme->admin_cq.mask << 16 | nvme->admin_sq.mask;
	nvme->regs->acq = (VirtAddr)nvme->admin_cq.entry - (VirtAddr)pm_get_phys_base();
	nvme->regs->asq = (VirtAddr)nvme->admin_sq.entry - (VirtAddr)pm_get_phys_base();
	nvme->regs->intms = ~0;

	// Determine supported IO Command Sets by checking CAP.CSS and initializing it.
	const auto cap = &nvme->regs->cap;
	if ((cap->css & NVME_CAP_CSS_IOCSS) && (cap->css & NVME_CAP_CSS_NCSS))
		nvme->regs->cc.css = 0b000;
	else if (cap->css & NVME_CAP_CSS_IOCSS)
		nvme->regs->cc.css = 0b110;
	else if (cap->css & NVME_CAP_CSS_NOIOCSS)
		nvme->regs->cc.css = 0b111;

	// Set arbitration to Round Robin.
	nvme->regs->cc.ams = 0b000;

	// Calculate page size bits (page_size == 1 << (12 + MPS))
	const usize page_min = 1 << (12 + cap->mpsmin);
	const usize page_max = 1 << (12 + cap->mpsmax);
	if (vm_get_page_size(VMLevel_0) < page_min || vm_get_page_size(VMLevel_0) > page_max)
	{
		pci_log_dev(pdev,
					"This machine's page size is unsupported, "
					"this NVMe device only accepts sizes between 0x%zx and 0x%zx!\n",
					page_min, page_max);
		return 1;
	}
	nvme->regs->cc.mps = __builtin_ctzll(vm_get_page_size(VMLevel_0)) - 12;

	// Enable the controller.
	nvme->regs->cc.en = true;
	// TODO: Replace with timeout
	while (!(nvme->regs->csts & NVME_CS_RDY))
	{
		// Check if status is fatal.
		if (nvme->regs->csts & NVME_CS_CFS)
		{
			pci_log_dev(pdev, "NVMe initialization has failed!\n");
			return 1;
		}
		asm_pause();
	}

	// Indentify NVMe drive.
	// TODO

	// Probe namespaces.
	// TODO

	// Create IO queues.
	nvme_io_cq_init(nvme, &nvme->io_cq, 3);
	nvme_io_sq_init(nvme, &nvme->io_sq, &nvme->io_cq, 2);

	// Init done.
	nvme_counter++;
	// TODO: block_register_device();
	pci_log_dev(pdev, "Initialized NVMe controller\n");
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

MODULE_FN i32 nvme_init()
{
	pci_register_driver(&nvme_driver);
	return 0;
}

MODULE_DEFAULT(nvme_init, NULL);
