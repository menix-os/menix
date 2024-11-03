// Process management.

#pragma once

#include <menix/common.h>
#include <menix/fs/fd.h>
#include <menix/fs/vfs.h>
#include <menix/memory/vm.h>
#include <menix/system/abi.h>
#include <menix/system/elf.h>
#include <menix/system/sch/thread.h>
#include <menix/util/list.h>
#include <menix/util/spin.h>

#define proc_log(fmt, ...) kmesg("[Process]\t" fmt, ##__VA_ARGS__)

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
	usize id;				 // Process ID.
	char name[256];			 // Name of the process.
	VfsNode* working_dir;	 // The current working directory.
	SpinLock lock;			 // Access lock.
	PageMap* page_map;		 // Process page map.
	VirtAddr map_base;		 // Virtual base to create new memory mappings at.
	usize runtime;			 // Amount of ticks the process has been alive.
	usize permissions;		 // Process access bits.
	ElfInfo elf_info;		 // ELF information to pass to auxv.

	Process* parent;	// The owner of this process.
	Process* next;		// Linked list entry for the next process.

	ThreadList threads;		 // Threads owned by the process.
	ProcessState state;		 // Current state of the process.
	ProcessList children;	 // Processes owned by the process.

	SpinLock fd_lock;						 // Access lock for file descriptors.
	FileDescriptor* file_descs[OPEN_MAX];	 // File descriptors for this process.

	i32 return_code;	// If the process is in a dead state, contains the code to return to the parent.
} Process;

// Creates a new process. Returns a reference to the newly created process.
// `name`: Name of the process.
// `state`: Which state the process should be initialized with.
// `is_user`: True if this process belongs to the user, otherwise it's a kernel process.
// `parent`: (Optional) The parent process of the to be created process.
Process* proc_create(const char* name, ProcessState state, bool is_user, Process* parent);

// Starts a new process from an ELF executable.
// `name`: Name of the process.
// `path`: File path pointing to the executable to run.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
// `is_user`: True if this process belongs to the user, otherwise it's a kernel process.
bool proc_execve(const char* name, const char* path, char** argv, char** envp, bool is_user);

// Sets up a process context.
// `proc`: The process to set up.
// `is_user`: True if this process belongs to the user, otherwise it's a kernel process.
// ? Defined per architecture.
void proc_setup(Process* proc, bool is_user);

// Destroys a process context.
// `proc`: The process to destroy.
// ? Defined per architecture.
void proc_destroy(Process* proc);

// Forks an exisiting process and returns its process ID.
// `proc`: The process to fork.
// `thread`: The executing thread.
usize proc_fork(Process* proc, Thread* thread);

// Sets up the context for a forked process.
// `fork`: Target process.
// `source`: The process being forked from.
// ? Defined per architecture.
void proc_fork_context(Process* fork, Process* source);

// Terminates a process.
// `proc`: The process to kill.
// `is_crash`: The reason for termination is a program crash.
void proc_kill(Process* proc, bool is_crash);

// Converts a file descriptor ID for the active process to a reference.
FileDescriptor* proc_fd_to_ptr(Process* process, usize fd);
