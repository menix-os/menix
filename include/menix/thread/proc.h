// Process management.

#pragma once

#include <menix/abi.h>
#include <menix/arch.h>
#include <menix/common.h>
#include <menix/fs/fd.h>
#include <menix/fs/vfs.h>
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
	ProcessState_Dead,		 // Process is dead and is waiting for cleanup.
} ProcessState;

// Thread information.
typedef struct Thread
{
	usize id;				   // Thread ID.
	SpinLock lock;			   // Access lock.
	Process* parent;		   // The parent process of this thread.
	ThreadState state;		   // Current state of the thread.
	CpuRegisters registers;	   // The register state at the time of context switch.
	usize stack;			   // The stack pointer.
	usize errno;			   // `errno` value.

	// Architecture dependent fields go here.
#if defined CONFIG_arch_x86
	u64 fs_base;		// FS register base address.
	u64 gs_base;		// GS register base address.
	void* saved_fpu;	// Saved FPU state.
#endif
} Thread;

typedef List(Process*) ProcessList;
typedef List(Thread*) ThreadList;

typedef struct Process
{
	usize id;				 // Process ID.
	char name[256];			 // Name of the process.
	VfsNode* working_dir;	 // The current working directory.
	SpinLock lock;			 // Access lock.
	PageMap* page_map;		 // Process page map.

	ThreadList threads;		 // Threads owned by the process.
	ProcessState state;		 // Current state of the process.
	Process* parent;		 // The owner of this process.
	ProcessList children;	 // Processes owned by the process.

	SpinLock fd_lock;						 // Access lock for file descriptors.
	FileDescriptor* file_descs[OPEN_MAX];	 // File descriptors for this process.

	i32 return_code;	// If the process is in a dead state, contains the code to return to the parent.
} Process;

// Creates a new process.
// `name`: Name of the process.
// `state`: Which state the process should be initialized with.
// `ip`: The instruction pointer address to initialize the process with.
// `is_user`: True if this process belongs to the user, otherwise it's a kernel process.
// `parent`: (Optional) The parent process of the to be created process.
void proc_create(char* name, ProcessState state, usize ip, bool is_user, Process* parent);

// Starts a new process from an ELF executable. Returns true if successful.
// `path`: File path pointing to the executable to run.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
bool proc_execve(const char* path, char** argv, char** envp);

// Forks an exisiting process and returns its process ID.
// `proc`: The process to fork.
// `thread`: The executing thread.
usize proc_fork(Process* proc, Thread* thread);

// Terminates a process.
// `proc`: The process to kill.
void proc_kill(Process* proc);
