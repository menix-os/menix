// NVMe Command creation

#include <menix/util/log.h>

#include <block/nvme.h>
#include <string.h>

NvmeSQEntry* nvme_cmd_new(NvmeSubQueue* queue, u8 opcode, u64 meta, u64 data)
{
	NvmeSQEntry* entry = &queue->entry[queue->tail];
	memset(entry, 0, sizeof(NvmeSQEntry));

	entry->cdw0.opc = opcode;
	entry->cdw0.cid = queue->tail;
	entry->mptr = meta;
	entry->dptr[0] = data;

	return entry;
}

void nvme_cmd_submit(NvmeSubQueue* queue)
{
	// Update the tail.
	queue->tail = (queue->tail + 1) & queue->mask;

	// Ring the doorbell.
	*queue->doorbell = queue->tail;
}
