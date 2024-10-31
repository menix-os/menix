// Thread structures

#pragma once
#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/util/list.h>
#include <menix/util/spin.h>

// Direct access to the errno of the current thread
#define thread_errno arch_current_cpu()->thread->errno

extern SpinLock thread_lock;

// Describes the state of a thread.
typedef enum
{
	ThreadState_Running,	 // Everything is OK.
	ThreadState_Ready,		 // Ready to run.
	ThreadState_Sleeping,	 // Thread is currently sleeping.
	ThreadState_Waiting,	 // Thread is waiting for something else.
} ThreadState;

typedef struct Process Process;
typedef struct Thread Thread;

// Thread information.
typedef struct Thread
{
	usize id;				  // Thread ID.
	SpinLock lock;			  // Access lock.
	bool can_exec;			  // If the thread is ready for execution.
	ThreadState state;		  // Current state of the thread.
	Context registers;		  // The register state at the time of context switch.
	VirtAddr stack;			  // The stack pointer.
	VirtAddr kernel_stack;	  // The kernel stack pointer.
	usize errno;			  // `errno` value.
	usize runtime;			  // Amount of ticks the thread has been alive.

	Process* parent;	// The parent process of this thread.
	Thread* next;		// Linked list entry for the next thread.

	// Architecture dependent fields go here.
#if defined CONFIG_arch_x86_64
	VirtAddr fs_base;	 // FS register base address.
	VirtAddr gs_base;	 // GS register base address.
	void* saved_fpu;	 // Saved FPU state.
#endif
} Thread;

typedef List(Thread*) ThreadList;

// Attempts to set errno of the currently running thread.
void thread_set_errno(usize errno);

// Creates a new thread in a process.
// `parent`: The parent process of the new thread.
Thread* thread_create(Process* parent);

// Sets up the context of a user thread using execve.
// `target`: The thread to set up.
// `start`: The virtual address where this thread will start executing from.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
void thread_execve(Thread* target, VirtAddr start, char** argv, char** envp, bool is_user);

// Sets up the context of a thread.
// `target`: The thread to set up.
// `start`: The virtual address where this thread will start executing from.
// `is_user`: True if this thread belongs to the user, otherwise it's a kernel thread.
// `stack`: (Optional) If nonzero, sets the user stack to this address instead of allocating a new stack.
// ? Defined per architecture.
void thread_setup(Thread* target, VirtAddr start, bool is_user, VirtAddr stack);

// Destroys the context of a thread.
// `target`: The thread to destroy.
// ? Defined per architecture.
void thread_destroy(Thread* thread);

// Make a thread sleep for a certain time.
// `target`: The thread to put to sleep.
// `nanoseconds`: The time to sleep in nanoseconds.
void thread_sleep(Thread* target, usize nanoseconds);

// Forks an existing thread by copying its attributes.
// `parent`: The process the target belongs to.
// `target`: The thread to fork.
void thread_fork(Process* parent, Thread* target);

// Marks a thread as ready to be killed. Death may not be instant.
// `victim`: The thread to kill.
// `reschedule`: If the scheduler should reschedule immediately after.
void thread_hang(Thread* victim, bool reschedule);

// Immediately kills a thread.
// `victim`: The thread to kill.
void thread_kill(Thread* victim);
