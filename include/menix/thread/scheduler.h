// Process scheduling

#pragma once
#include <menix/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/thread.h>

// Initializes the scheduler.
void scheduler_init(BootInfo* info);

// Makes the scheduler act immediately instead of waiting for a timer.
// ? Defined per architecture.
void scheduler_invoke();

// Implementation of the scheduler. Not meant to be called directly.
// ? Defined per architecture.
void scheduler_reschedule(CpuRegisters* regs);

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
