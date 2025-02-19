// NVMe data structures after the v2.1 specification

#pragma once
#include <menix/common.h>
#include <menix/memory/mmio.h>

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
typedef struct
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
	union
	{
		struct
		{
			u32 cdw10;	  // Command Dword 10
			u32 cdw11;	  // Command Dword 11
			u32 cdw12;	  // Command Dword 12
			u32 cdw13;	  // Command Dword 13
			u32 cdw14;	  // Command Dword 14
			u32 cdw15;	  // Command Dword 15
		};
		struct ATTR(packed)
		{
			u8 cns;		  // Controller or Namespace Structure
			u8 _0;		  // Reserved
			u16 cntid;	  // Controller Identifier
		} identify;		  // Identify comand
	};
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

typedef struct
{
	u16 maxPower;
	u8 __reserved2;
	u8 flags;
	u32 entryLatency;
	u32 exitLatency;
	u8 readThroughput;
	u8 readLatency;
	u8 writeThroughput;
	u8 writeLatency;
	u16 idlePower;
	u8 idleScale;
	u8 __reserved19;
	u16 activePower;
	u8 activeWorkScale;
	u8 __reserved23[9];
} NvmePowerState;
static_assert(sizeof(NvmePowerState) == 32);

typedef struct
{
	u16 vid;
	u16 ssvid;
	char sn[20];
	char mn[40];
	char fr[8];
	u8 rab;
	u8 ieee[3];
	u8 cmic;
	u8 mdts;
	u16 cntlid;
	u32 ver;
	u32 rtd3r;
	u32 rtd3e;
	u32 oaes;
	u32 ctratt;
	u8 __reserved100[11];
	u8 cntrltype;
	u8 __reserved112[16];
	u16 crdt1;
	u16 crdt2;
	u16 crdt3;
	u8 __reserved134[122];
	u16 oacs;
	u8 acl;
	u8 aerl;
	u8 frmw;
	u8 lpa;
	u8 elpe;
	u8 npss;
	u8 avscc;
	u8 apsta;
	u16 wctemp;
	u16 cctemp;
	u16 mtfa;
	u32 hmpre;
	u32 hmmin;
	u8 tnvmcap[16];
	u8 unvmcap[16];
	u32 rpmbs;
	u16 edstt;
	u8 dsto;
	u8 fwug;
	u16 kas;
	u16 hctma;
	u16 mntmt;
	u16 mxtmt;
	u32 sanicap;
	u32 hmminds;
	u16 hmmaxd;
	u8 __reserved338[4];
	u8 anatt;
	u8 anacap;
	u32 anagrpmax;
	u32 nanagrpid;
	u8 __reserved352[160];
	u8 sqes;
	u8 cqes;
	u16 maxcmd;
	u32 nn;
	u16 oncs;
	u16 fuses;
	u8 fna;
	u8 vwc;
	u16 awun;
	u16 awupf;
	u8 nvscc;
	u8 nwpc;
	u16 acwu;
	u8 __reserved534[2];
	u32 sgls;
	u32 mnan;
	u8 __reserved544[224];
	char subnqn[256];
	u8 __reserved1024[768];
	u32 ioccsz;
	u32 iorcsz;
	u16 icdoff;
	u8 ctrattr;
	u8 msdbd;
	u8 __reserved1804[244];
	NvmePowerState psd[32];
	u8 vs[1024];
} NvmeIdentifyController;
static_assert(sizeof(NvmeIdentifyController) == 0x1000);

#define NVME_CS_RDY	  (1 << 0)
#define NVME_CS_CFS	  (1 << 1)
#define NVME_CS_SHST  (1 << 2)
#define NVME_CS_NSSRO (1 << 4)
#define NVME_CS_PP	  (1 << 5)

#define NVME_CAP_CSS_NCSS	 (1 << 0)
#define NVME_CAP_CSS_IOCSS	 (1 << 6)
#define NVME_CAP_CSS_NOIOCSS (1 << 7)

typedef struct ATTR(packed)
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
} NvmeControllerCap;
static_assert(sizeof(NvmeControllerCap) == 8);

typedef struct ATTR(packed)
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
} NvmeControllerConfig;
static_assert(sizeof(NvmeControllerConfig) == 4);

// Controller Properties (3.1.4)
typedef struct ATTR(packed)
{
	NvmeControllerCap cap;		// Controller Capabilities
	u32 vs;						// Version
	u32 intms;					// Interrupt Mask Set
	u32 intmc;					// Interrupt Mask Clear
	NvmeControllerConfig cc;	// Controller Configuration
	u32 _0;						// Reserved
	u32 csts;					// Controller Status
	u32 nssr;					// NVM Subsystem Reset
	u32 aqa;					// Admin Queue Attributes
	u64 asq;					// Admin Submission Queue Base Address
	u64 acq;					// Admin Completion Queue Base Address
} NvmeRegisters;

typedef struct
{
	mmio32* doorbell;	 // Address of the doorbell for this queue.
	NvmeCQEntry* entry;
	u16 entry_count;
	u16 head;
	u8 phase;
} NvmeComQueue;

typedef struct
{
	mmio32* doorbell;	   // Address of the doorbell for this queue.
	NvmeSQEntry* entry;	   // Start of the entry buffer.
	NvmeComQueue* cq;	   // Corresponding completion queue.
	u16 entry_count;
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
	PhysAddr bar;			  // Physical base address register
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

// Submits the last command to the given queue and rings the doorbell.
void nvme_cmd_submit(NvmeSQEntry* command, NvmeSubQueue* queue);
// Initializes an IO submission queue.
void nvme_io_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* cq, u16 idx);
// Initializes an IO completion queue.
void nvme_io_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx);
// Initializes a submission queue.
void nvme_sq_init(NvmeController* nvme, NvmeSubQueue* sq, NvmeComQueue* const cq, u16 idx, u16 len);
// Initializes a completion queue.
void nvme_cq_init(NvmeController* nvme, NvmeComQueue* cq, u16 idx, u16 len);
// Identifies a NVMe controller.
void nvme_ident(NvmeController* nvme);
