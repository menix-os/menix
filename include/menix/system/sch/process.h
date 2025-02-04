// Process management.

#pragma once

#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/system/elf.h>
#include <menix/system/sch/thread.h>
#include <menix/util/list.h>
#include <menix/util/spin.h>

// Describes the state of a process.
typedef enum
{
	ProcessState_Running,	 // Everything is OK.
	ProcessState_Ready,		 // Ready to run.
	ProcessState_Waiting,	 // Process is waiting for another process to resume.
	ProcessState_Blocked,	 // Process is blocked.
	ProcessState_Dead,		 // Process is dead and is waiting for cleanup.
} ProcessState;

typedef struct Process Process;
typedef List(Process*) ProcessList;

extern ProcessList dead_processes;

typedef struct Process
{
	usize id;		   // Process ID.
	char name[256];	   // Name of the process.
	SpinLock lock;	   // Access lock.
	usize runtime;	   // Amount of ticks the process has been alive.

	ElfInfo elf_info;	 // ELF information to pass to auxv.

	Process* parent;	// The owner of this process.
	Process* next;		// Linked list entry for the next process.

	ProcessState state;		 // Current state of the process.
	ThreadList threads;		 // Threads owned by the process.
	ProcessList children;	 // Processes owned by the process.

	PageMap* page_map;		   // Process page map.
	VirtAddr map_base;		   // Virtual base address to create new memory mappings at.
	MemoryMappingList maps;	   // Mapping list of dynamically allocated pages.
} Process;

// Creates a new process. Returns a reference to the newly created process.
// `name`: Name of the process.
// `state`: Which state the process should be initialized with.
// `is_user`: True if this process belongs to the user, otherwise it's a kernel process.
// `parent`: (Optional) The parent process of the to be created process.
Process* proc_create(const char* name, ProcessState state, bool is_user, Process* parent);

// Starts a new process from an ELF executable.
// `name`: Name of the process.
// `data`: A pointer to an ELF in memory.
// `length`: The length of the ELF in bytes.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
bool proc_create_elf(const char* name, const void* data, usize length, char** argv, char** envp);

// Sets up a process context.
// `proc`: The process to set up.
// `is_user`: True if this process belongs to the user, otherwise it's a kernel process.
// ? Defined per architecture.
void proc_setup(Process* proc, bool is_user);

// Destroys a process context.
// `proc`: The process to destroy.
void proc_destroy(Process* proc);

// Forks an exisiting process and returns its process ID.
// `proc`: The process to fork.
// `thread`: The executing thread.
usize proc_fork(Process* proc, Thread* thread);

// Terminates a process.
// `proc`: The process to kill.
// `is_crash`: The reason for termination is a program crash.
void proc_kill(Process* proc, bool is_crash);
