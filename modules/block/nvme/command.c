// NVMe Command creation

#include <menix/util/log.h>

#include <block/nvme.h>

void nvme_cmd_submit(NvmeSQEntry* command, NvmeSubQueue* queue)
{
	command->cdw0.cid = queue->tail;
	queue->entry[queue->tail] = *command;

	// Update the tail.
	if (queue->tail >= queue->entry_count - 1)
		queue->tail = 0;
	else
		queue->tail++;

	// Ring the doorbell.
	*queue->doorbell = queue->tail;
}
