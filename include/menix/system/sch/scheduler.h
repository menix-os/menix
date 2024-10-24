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
void sch_init(BootInfo* info);

// Temporarily stops volatile rescheduling until a call to `sch_invoke` happens.
// ? Defined per architecture.
void sch_pause();

// Makes the scheduler act immediately instead of waiting for a timer.
// ? Defined per architecture.
void sch_invoke();

// Implementation of the scheduler. Not meant to be called directly.
// ? Defined per architecture.
void sch_reschedule(Context* regs);

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
