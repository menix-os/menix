// Process scheduling

#pragma once
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/thread.h>

extern Process* proc_list;
extern Process* hanging_proc_list;

extern Thread* thread_list;
extern Thread* hanging_thread_list;

extern Thread* sleeping_thread_list;

// Initializes the scheduler.
void sch_init(VirtAddr entry_point);

// Makes the scheduler act immediately instead of waiting for a timer.
// ? Defined per architecture.
void sch_arch_invoke();

// Saves the architecture dependent data of the `thread`.
// ? Defined per architecture.
void sch_arch_save(CpuInfo* core, Thread* thread);

// Updates the `core` thread with the relevant information from the `next` thread.
// ? Defined per architecture.
void sch_arch_update(CpuInfo* core, Thread* next);

// Stops execution on the current core and waits for another interrupt.
// ? Defined per architecture.
ATTR(noreturn) void sch_arch_stop();

// Implementation of the scheduler. Not meant to be called directly, use `sch_invoke` instead.
// Returns a pointer to the new context.
Context* sch_reschedule(Context* regs);

// Returns the next thread that's ready to get execution time.
// `list`: The head of the list to check.
Thread* sch_next(Thread* list);

// Adds a process to the scheduler.
// `target`: The process to add.
void sch_add_process(Process** list, Process* target);

// Removes a process from the scheduler.
// `target`: The process to remove.
void sch_remove_process(Process** list, Process* target);

// Adds a thread to the scheduler.
// `target`: The thread to add.
void sch_add_thread(Thread** list, Thread* target);

// Removes a thread from the scheduler.
// `target`: The thread to remove.
void sch_remove_thread(Thread** list, Thread* target);

// Translates a process ID to the corresponding process.
Process* sch_id_to_process(usize pid);

// Translates a thread ID to the corresponding thread.
Thread* sch_id_to_thread(usize tid);
