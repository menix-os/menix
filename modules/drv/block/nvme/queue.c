// Submission and completion queues

#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/util/log.h>

#include <block/nvme.h>
#include <string.h>

void nvme_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx, u32 len)
{
	kassert((idx & 1) == true, "Completion Queues must be on uneven indices");
	memset(cq, 0, sizeof(NvmeComQueue));

	cq->doorbell = (mmio32*)(nvme->mmio_base + 0x1000 + idx * nvme->doorbell_stride);
	cq->phase = 1;
	cq->mask = len - 1;
	cq->head = 0;
	cq->entry = vm_map_memory(pm_alloc(1), vm_get_page_size(VMLevel_Small), VMProt_Read | VMProt_Write);
	memset(cq->entry, 0, vm_get_page_size(VMLevel_Small));
}

void nvme_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* cq, u16 idx, u32 len)
{
	kassert((idx & 1) == false, "Submission Queues must be on even indices");
	memset(sq, 0, sizeof(NvmeSubQueue));

	sq->doorbell = (mmio32*)(nvme->mmio_base + 0x1000 + idx * nvme->doorbell_stride);
	sq->cq = cq;
	sq->mask = len - 1;
	sq->head = 0;
	sq->tail = 0;
	sq->entry = vm_map_memory(pm_alloc(1), vm_get_page_size(VMLevel_Small), VMProt_Read | VMProt_Write);
	memset(sq->entry, 0, vm_get_page_size(VMLevel_Small));
}

void nvme_io_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx)
{
	// Get the length. CAP.MQES is 0 based.
	u32 length = 1 + (nvme->regs->cap.mqes);

	if (length > vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry))
		length = vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry);

	nvme_cq_init(nvme, cq, idx, length);

	NvmeSQEntry* cmd =
		nvme_cmd_new(&nvme->admin_sq, NVME_ACMD_CREATE_CQ, 0, vm_virt_to_phys(vm_kernel_map, (VirtAddr)cq->entry));

	cmd->cdw10 = (cq->mask << 16) | (idx >> 1);
	cmd->cdw11 = 1;

	nvme_cmd_submit(&nvme->admin_sq);
}

void nvme_io_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* cq, u16 idx)
{
	// Get the length. CAP.MQES is 0 based.
	u32 length = 1 + (nvme->regs->cap.mqes);

	if (length > vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry))
		length = vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry);

	nvme_sq_init(nvme, sq, cq, idx, length);

	NvmeSQEntry* cmd =
		nvme_cmd_new(&nvme->admin_sq, NVME_ACMD_CREATE_SQ, 0, vm_virt_to_phys(vm_kernel_map, (VirtAddr)sq->entry));

	cmd->cdw10 = (sq->mask << 16) | (idx >> 1);
	cmd->cdw11 = (idx >> 1) << 16 | 1;

	nvme_cmd_submit(&nvme->admin_sq);
}
