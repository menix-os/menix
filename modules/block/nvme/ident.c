// NVMe controller identification

#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/system/time/clock.h>
#include <menix/util/log.h>

#include <block/nvme.h>

void nvme_ident(NvmeController* nvme)
{
	NvmeSQEntry* entry = kzalloc(sizeof(NvmeSQEntry));
	const PhysAddr buf_mem = pm_alloc(0x1000);
	void* result_buf = vm_map_memory(buf_mem, sizeof(NvmeIdentifyController), VMProt_Read | VMProt_Write);

	entry->cdw0.opc = NVME_ACMD_IDENTIFY;
	entry->dptr[0] = buf_mem;

	// We want the controller identification structure.
	entry->identify.cns = 1;

	nvme_cmd_submit(entry, &nvme->admin_sq);

	clock_wait(10000);

	print_log("nvme: Identified controller:\n");
	print_log("nvme: Serial Number | %20s\n", ((NvmeIdentifyController*)result_buf)->sn);
	print_log("nvme: Model Number  | %20s\n", ((NvmeIdentifyController*)result_buf)->mn);
	print_log("nvme: Firmware Rev. | %8s\n", ((NvmeIdentifyController*)result_buf)->fr);
	*nvme->admin_sq.cq->doorbell = nvme->admin_sq.tail++;
}
