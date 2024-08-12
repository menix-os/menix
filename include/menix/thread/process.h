// Process management.

#pragma once

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/thread/spin.h>

typedef usize ProcessId;
typedef usize ThreadId;
typedef struct Process Process;

// Active state of the thread.
typedef enum
{
	ThreadState_Running,	 // Everything is OK.
	ThreadState_Ready,		 // Ready to run.
	ThreadState_Sleeping,	 // Thread is currently sleeping.
	ThreadState_Waiting,	 // Thread is waiting for something else.
} ThreadState;

// Thread information.
typedef struct Thread
{
	ThreadId id;			   // Thread ID.
	SpinLock lock;			   // Access lock.
	Process* process;		   // The underlying process.
	ThreadState state;		   // Current state of the thread.
	CpuRegisters registers;	   // The register state at the time of context switch.
	usize errno;			   // `errno` value.
	struct Thread* next;

	// Architecture dependent fields go here.
#if defined CONFIG_arch_x86
	u64 fs_base;	// FS register base address.
	u64 gs_base;	// GS register base address, used for TLS.
#elif defined CONFIG_arch_aarch64
#elif defined CONFIG_arch_riscv64
#endif
} Thread;

typedef enum
{
	ProcessState_Running,	 // Everything is OK.
	ProcessState_Ready,		 // Ready to run.
	ProcessState_Waiting,	 // Process is waiting for another process to resume.
	ProcessState_Blocked,	 // Process is blocked.
} ProcessState;

struct Process
{
	ProcessId id;		   // Process ID.
	SpinLock lock;		   // Access lock.
	PageMap* page_map;	   // Process page map.
	ProcessState state;	   // Current state of the process.
};

// Creates a new process of an executable pointed to by `path`.
Process* process_create(const char* path);
