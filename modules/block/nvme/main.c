// NVMe block devices.

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/device.h>
#include <menix/system/module.h>
#include <menix/system/pci/pci.h>
#include <menix/system/time/clock.h>
#include <menix/util/units.h>

#include <block/nvme.h>

// Global counter for known NVMe devices.
static usize nvme_counter = 0;

void nvme_read(NvmeNameSpace* ns, PhysAddr phys, Buffer buffer)
{
	// TODO
}

void nvme_interrupt_handler(usize isr, Context* context, void* data)
{
}

// Initializes an NVMe controller (See section 3.5.1)
i32 nvme_probe(PciDevice* pdev)
{
	pci_log_dev(pdev, "Starting to probe NVMe controller.\n");

	// Allocate device data.
	NvmeController* nvme = kzalloc(sizeof(NvmeController));
	dev_set_data(pdev->dev, nvme);

	pci_set_bus_mastering(pdev, true);
	nvme->bar = pci_get_bar(pdev, 0);
	// Map at least to the start of the queues plus 512 entries.
	nvme->mmio_base = vm_map_memory(nvme->bar, 0x1000 + 0x1000, VMProt_Read | VMProt_Write);

	const NvmeControllerCap cap = nvme->regs->cap;

	// Disable the controller if it wasn't already.
	NvmeControllerConfig cc = nvme->regs->cc;
	cc.en = false;
	nvme->regs->cc = cc;

	// Wait for the controller to indicate that the reset is complete (RDY = 0).
	clock_timeout_poll(SECONDS_TO_NANO(2), !(nvme->regs->csts & NVME_CS_RDY), {
		pci_error_dev(pdev, "NVMe device didn't respond to reset within 2 seconds. Hardware is probably faulty.\n");
		return 1;
	});

	// Get the doorbell stride.
	nvme->doorbell_stride = 4 << cap.dstrd;

	// Configure Admin Queue by setting AQA, ASQ and ACQ.
	nvme_cq_init(nvme, &nvme->admin_cq, 0, 0x1000 / sizeof(NvmeCQEntry));
	nvme_sq_init(nvme, &nvme->admin_sq, &nvme->admin_cq, 0, 0x1000 / sizeof(NvmeSQEntry));
	nvme->regs->aqa = (nvme->admin_cq.entry_count - 1) << 16 | (nvme->admin_sq.entry_count - 1);
	nvme->regs->acq = vm_virt_to_phys(vm_kernel_map, (VirtAddr)nvme->admin_cq.entry);
	nvme->regs->asq = vm_virt_to_phys(vm_kernel_map, (VirtAddr)nvme->admin_sq.entry);
	nvme->regs->intms = ~0;

	// Determine supported IO Command Sets by checking CAP.CSS and initializing it.
	if ((cap.css & NVME_CAP_CSS_IOCSS) && (cap.css & NVME_CAP_CSS_NCSS))
		cc.css = 0b000;
	else if (cap.css & NVME_CAP_CSS_IOCSS)
		cc.css = 0b110;
	else if (cap.css & NVME_CAP_CSS_NOIOCSS)
		cc.css = 0b111;

	// Set arbitration to Round Robin.
	cc.ams = 0b000;

	// Calculate page size bits (page_size == 1 << (12 + MPS))
	const usize page_min = 1 << (12 + cap.mpsmin);
	const usize page_max = 1 << (12 + cap.mpsmax);
	if (vm_get_page_size(VMLevel_Small) < page_min || vm_get_page_size(VMLevel_Small) > page_max)
	{
		pci_error_dev(pdev,
					  "This machine's page size is unsupported, "
					  "this NVMe device only accepts sizes between 0x%zx and 0x%zx!\n",
					  page_min, page_max);
		return 1;
	}
	cc.mps = __builtin_ctzll(vm_get_page_size(VMLevel_Small)) - 12;

	// Enable the controller.
	cc.en = true;
	nvme->regs->cc = cc;

	clock_timeout_poll(SECONDS_TO_NANO(2), nvme->regs->csts & NVME_CS_RDY, {
		pci_error_dev(pdev, "NVMe device didn't respond to enable within 2 seconds. Something went wrong.\n");
		// Check if status is fatal.
		if (nvme->regs->csts & NVME_CS_CFS)
			pci_error_dev(pdev, "NVMe initialization failure is fatal!\n");
		return 1;
	});

	// Identify NVMe controller.
	nvme_ident(nvme);

	// Probe namespaces.
	// TODO

	// Create IO queues.
	// nvme_io_cq_init(nvme, &nvme->io_cq, 1);
	// nvme_io_sq_init(nvme, &nvme->io_sq, &nvme->io_cq, 1);

	// Init is done, register the device.
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

static i32 nvme_init()
{
	pci_register_driver(&nvme_driver);
	return 0;
}

static void nvme_exit()
{
	pci_unregister_driver(&nvme_driver);
}

MODULE_DEFAULT(nvme_init, nvme_exit);
