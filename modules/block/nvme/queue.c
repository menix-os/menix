// Submission and completion queues

#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/util/log.h>

#include <block/nvme.h>
#include <string.h>

void nvme_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx, u16 len)
{
	memset(cq, 0, sizeof(NvmeComQueue));

	cq->doorbell = (mmio32*)(nvme->mmio_base + 0x1000 + (2 * idx + 1) * nvme->doorbell_stride);
	cq->phase = 1;
	cq->entry_count = len;
	cq->head = 0;
	cq->entry = vm_map_memory(pm_alloc(arch_page_size), arch_page_size, VMProt_Read | VMProt_Write);
	memset(cq->entry, 0, vm_get_page_size(VMLevel_Small));
}

void nvme_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* const cq, u16 idx, u16 len)
{
	memset(sq, 0, sizeof(NvmeSubQueue));

	sq->doorbell = (mmio32*)(nvme->mmio_base + 0x1000 + (2 * idx) * nvme->doorbell_stride);
	sq->cq = cq;
	sq->entry_count = len;
	sq->head = 0;
	sq->tail = 0;
	sq->entry = vm_map_memory(pm_alloc(arch_page_size), arch_page_size, VMProt_Read | VMProt_Write);
	memset(sq->entry, 0, vm_get_page_size(VMLevel_Small));
}

#if 0
void nvme_io_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx)
{
	// Get the length. CAP.MQES is 0 based.
	const NvmeControllerCap cap = nvme->regs->cap;
	u32 length = 1 + (cap.mqes);

	if (length > vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry))
		length = vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry);

	nvme_cq_init(nvme, cq, idx, length);

	NvmeSQEntry* cmd =
		nvme_cmd_new(&nvme->admin_sq, NVME_ACMD_CREATE_CQ, 0, vm_virt_to_phys(vm_kernel_map, (VirtAddr)cq->entry));

	cmd->cdw10 = ((cq->entry_count << 16) - 1) | (idx >> 1);
	cmd->cdw11 = 1;

	nvme_cmd_submit(&nvme->admin_sq);
}

void nvme_io_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* cq, u16 idx)
{
	// Get the length. CAP.MQES is 0 based.
	const NvmeControllerCap cap = nvme->regs->cap;
	u32 length = 1 + (cap.mqes);

	if (length > vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry))
		length = vm_get_page_size(VMLevel_Small) / sizeof(NvmeCQEntry);

	nvme_sq_init(nvme, sq, cq, idx, length);

	NvmeSQEntry* cmd =
		nvme_cmd_new(&nvme->admin_sq, NVME_ACMD_CREATE_SQ, 0, vm_virt_to_phys(vm_kernel_map, (VirtAddr)sq->entry));

	cmd->cdw10 = ((sq->entry_count << 16) - 1) | (idx >> 1);
	cmd->cdw11 = (idx >> 1) << 16 | 1;

	nvme_cmd_submit(&nvme->admin_sq);
}
#endif
