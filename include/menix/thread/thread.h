// Thread structures

#pragma once
#include <menix/arch.h>
#include <menix/common.h>
#include <menix/thread/spin.h>
#include <menix/util/list.h>

// Direct access to the errno of the current thread
#define thread_errno arch_current_cpu()->thread->errno

// Describes the state of a thread.
typedef enum
{
	ThreadState_Running,	 // Everything is OK.
	ThreadState_Ready,		 // Ready to run.
	ThreadState_Sleeping,	 // Thread is currently sleeping.
	ThreadState_Waiting,	 // Thread is waiting for something else.
} ThreadState;

typedef struct Process Process;

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

typedef List(Thread*) ThreadList;

// Creates a new thread in a process.
// `parent`: The parent process of the new thread.
// `start`: The start address of the new thread.
// `is_user`: If true, the thread is a user thread, otherwise it's a kernel thread.
void thread_create(Process* parent, VirtAddr start, bool is_user);

// Prepares a thread for `process_execve`.
// `parent`: The parent process of the new thread.
// `target`: The thread to prepare.
// `start`: The start address of the new thread.
// `argv`: A NULL-terminated list of program arguments to be passed to the thread.
// `envp`: A NULL-terminated list of environment variables to be passed to the thread.
void thread_execve(Process* parent, Thread* target, VirtAddr start, char** argv, char** envp);

// Forks an existing thread by copying its attributes.
// `parent`: The process the target and new thread belong to.
// `target`: The thread to fork.
void thread_fork(Process* parent, Thread* target);

// Marks a thread as ready to be killed. Death may not be instant.
// `victim`: The thread to kill.
void thread_hang(Thread* victim);

// Immediately kills a thread.
// `victim`: The thread to kill.
void thread_kill(Thread* victim);
