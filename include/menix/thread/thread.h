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

// Marks a thread as ready to be killed. Death may not be instant.
// `victim`: The thread to kill.
void thread_hang(Thread* victim);

// Immediately kills a thread.
// `victim`: The thread to kill.
void thread_kill(Thread* victim);
