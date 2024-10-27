// NVMe data structures after the v2.1 specification

#pragma once
#include <menix/common.h>
#include <menix/io/mmio.h>

// Generic Command Set
#define NVME_CMD_FLUSH 0x00
#define NVME_CMD_WRITE 0x01
#define NVME_CMD_READ  0x02

// Admin Command Set
#define NVME_ACMD_DELETE_SQ	   0x00
#define NVME_ACMD_CREATE_SQ	   0x01
#define NVME_ACMD_DELETE_CQ	   0x04
#define NVME_ACMD_CREATE_CQ	   0x05
#define NVME_ACMD_IDENTIFY	   0x06
#define NVME_ACMD_ABORT		   0x08
#define NVME_ACMD_SET_FEATURES 0x09
#define NVME_ACMD_GET_FEATURES 0x0A

// Submission Queue Entry (4.1)
typedef struct ATTR(packed)
{
	struct ATTR(packed)
	{
		u8 opc;			 // Opcode
		Bits fuse:2;	 // Fused Operation
		Bits _0:4;		 // Reserved
		Bits psdt:2;	 // PRP or SGL for Data Transfer
		u16 cid;		 // Command Identifier
	} cdw0;				 // Command Dword 0
	u32 nsid;			 // Namespace Identifier
	u32 cdw2;			 // Command Dword 2
	u32 cdw3;			 // Command Dword 3
	PhysAddr mptr;		 // Metadata Pointer
	PhysAddr dptr[2];	 // Data Pointer
	u32 cdw10;			 // Command Dword 10
	u32 cdw11;			 // Command Dword 11
	u32 cdw12;			 // Command Dword 12
	u32 cdw13;			 // Command Dword 13
	u32 cdw14;			 // Command Dword 14
	u32 cdw15;			 // Command Dword 15
} NvmeSQEntry;
static_assert(sizeof(NvmeSQEntry) == 64);

// Completion Queue Entry (4.2)
typedef struct ATTR(packed)
{
	u32 dw0;	   // Command Specific Dword 0
	u32 dw1;	   // Command Specific Dword 1
	u16 sqid;	   // SQ Identifier
	u16 sqhd;	   // SQ Head Pointer
	u16 cid;	   // Command Identifier
	u16 status;	   // Status
} NvmeCQEntry;
static_assert(sizeof(NvmeCQEntry) == 16);

#define NVME_CS_RDY	  (1 << 0)
#define NVME_CS_CFS	  (1 << 1)
#define NVME_CS_SHST  (1 << 2)
#define NVME_CS_NSSRO (1 << 4)
#define NVME_CS_PP	  (1 << 5)

#define NVME_CAP_CSS_NCSS	 (1 << 0)
#define NVME_CAP_CSS_IOCSS	 (1 << 6)
#define NVME_CAP_CSS_NOIOCSS (1 << 7)

// Controller Properties (3.1.4)
typedef struct ATTR(packed)
{
	struct ATTR(packed)
	{
		Bits mqes:16;	 // Maximum Queue Entries Supported
		Bits cqr:1;		 // Contiguous Queues Required
		Bits ams:2;		 // Arbitration Mechanism Supported
		Bits _0:5;		 // Reserved
		Bits to:8;		 // Timeout
		Bits dstrd:4;	 // Doorbell Stride
		Bits nssrs:1;	 // NVM Subsystem Reset Supported
		Bits css:8;
		Bits bps:1;
		Bits _1:2;
		Bits mpsmin:4;
		Bits mpsmax:4;
		Bits pmrs:1;
		Bits cmbs:1;
		Bits nsss:1;
		Bits crms:2;
		Bits nsses:1;
		Bits _2:2;
	} cap;		  // Controller Capabilities
	u32 vs;		  // Version
	u32 intms;	  // Interrupt Mask Set
	u32 intmc;	  // Interrupt Mask Clear
	struct ATTR(packed)
	{
		Bits en:1;	  // Enabled
		Bits _0:3;	  // Reserved
		Bits css:3;
		Bits mps:4;	   // Memory Page Size
		Bits ams:4;
		Bits shn:2;
		Bits iosqes:4;
		Bits iocqes:4;
		Bits crime:1;
		Bits _1:6;	  // Reserved
	} cc;			  // Controller Configuration
	u32 _0;			  // Reserved
	u32 csts;		  // Controller Status
	u32 nssr;		  // NVM Subsystem Reset
	u32 aqa;		  // Admin Queue Attributes
	u64 asq;		  // Admin Submission Queue Base Address
	u64 acq;		  // Admin Completion Queue Base Address
} NvmeRegisters;

typedef struct
{
	mmio32* doorbell;	 // Address of the doorbell for this queue.
	NvmeCQEntry* entry;
	u16 mask;
	u16 head;
	u8 phase;
} NvmeComQueue;

typedef struct
{
	mmio32* doorbell;	   // Address of the doorbell for this queue.
	NvmeSQEntry* entry;	   // Start of the entry buffer.
	NvmeComQueue* cq;	   // Corresponding completion queue.
	u16 mask;
	u16 head;
	u16 tail;
} NvmeSubQueue;

typedef struct
{
	union
	{
		void* mmio_base;
		volatile NvmeRegisters* regs;
	};
	NvmeSubQueue admin_sq;	  // Admin submission queue
	NvmeComQueue admin_cq;	  // Admin completion queue
	NvmeSubQueue io_sq;		  // IO submission queue
	NvmeComQueue io_cq;		  // IO completion queue
	u32 doorbell_stride;	  // The size in bytes between doorbell entries.
} NvmeController;

typedef struct
{
	NvmeController* controller;
	u32 id;
	u64 num_lba;
	u32 block_size;
	u32 meta_size;
} NvmeNameSpace;

// Creates a new command entry.
NvmeSQEntry* nvme_cmd_new(NvmeSubQueue* queue, u8 opcode, u64 meta, u64 data);
// Submits the last command to the given queue and rings the doorbell.
void nvme_cmd_submit(NvmeSubQueue* queue);
// Initializes an IO submission queue.
void nvme_io_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* cq, u16 idx);
// Initializes an IO completion queue.
void nvme_io_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx);
// Initializes a submission queue.
void nvme_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* cq, u16 idx, u32 len);
// Initializes a completion queue.
void nvme_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx, u32 len);
