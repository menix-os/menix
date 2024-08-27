// Process management.

#pragma once

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/thread/spin.h>
#include <menix/util/list.h>

typedef struct Process Process;

// Describes the state of a thread.
typedef enum
{
	ThreadState_Running,	 // Everything is OK.
	ThreadState_Ready,		 // Ready to run.
	ThreadState_Sleeping,	 // Thread is currently sleeping.
	ThreadState_Waiting,	 // Thread is waiting for something else.
} ThreadState;

// Describes the state of a process.
typedef enum
{
	ProcessState_Running,	 // Everything is OK.
	ProcessState_Ready,		 // Ready to run.
	ProcessState_Waiting,	 // Process is waiting for another process to resume.
	ProcessState_Blocked,	 // Process is blocked.
} ProcessState;

// Thread information.
typedef struct Thread
{
	ThreadId id;			   // Thread ID.
	SpinLock lock;			   // Access lock.
	Process* parent;		   // The parent process of this thread.
	ThreadState state;		   // Current state of the thread.
	CpuRegisters registers;	   // The register state at the time of context switch.
	usize stack;			   // The stack pointer.
	usize errno;			   // `errno` value.

	// Architecture dependent fields go here.
#if defined CONFIG_arch_x86
	u64 fs_base;		// FS register base address.
	u64 gs_base;		// GS register base address, used for TLS.
	void* saved_fpu;	// FPU state.
#endif
} Thread;

//
typedef struct Process
{
	ProcessId id;			  // Process ID.
	char name[256];			  // Name of the process.
	SpinLock lock;			  // Access lock.
	PageMap* page_map;		  // Process page map.
	ProcessState state;		  // Current state of the process.
	List(Thread*) threads;	  // Threads owned by the process.
	Process* parent;		  // The owner of this process.
} Process;

// Creates a new process.
void proc_create(char* name, ProcessState state);

// Starts a new process from an ELF executable. Returns true if successful.
// `path`: File path pointing to the executable to run.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
bool proc_exec(const char* path, char** argv, char** envp);
