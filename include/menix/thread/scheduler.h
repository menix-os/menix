// Process scheduling

#pragma once
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/thread.h>

extern Process* process_list;
extern Process* hanging_process_list;

extern Thread* thread_list;
extern Thread* hanging_thread_list;

extern Thread* sleeping_thread_list;

// Initializes the scheduler.
void scheduler_init(BootInfo* info);

// Temporarily stops volatile rescheduling until a call to `scheduler_invoke` happens.
// ? Defined per architecture.
void scheduler_pause();

// Makes the scheduler act immediately instead of waiting for a timer.
// ? Defined per architecture.
void scheduler_invoke();

// Implementation of the scheduler. Not meant to be called directly.
// ? Defined per architecture.
void scheduler_reschedule(Context* regs);

// Returns the next thread that's ready to get execution time.
// `list`: The head of the list to check.
Thread* scheduler_next(Thread* list);

// Adds a process to the scheduler.
// `target`: The process to add.
void scheduler_add_process(Process** list, Process* target);

// Removes a process from the scheduler.
// `target`: The process to remove.
void scheduler_remove_process(Process** list, Process* target);

// Adds a thread to the scheduler.
// `target`: The thread to add.
void scheduler_add_thread(Thread** list, Thread* target);

// Removes a thread from the scheduler.
// `target`: The thread to remove.
void scheduler_remove_thread(Thread** list, Thread* target);

// Translates a process ID to the corresponding process.
Process* scheduler_id_to_process(usize pid);

// Translates a thread ID to the corresponding thread.
Thread* scheduler_id_to_thread(usize tid);
