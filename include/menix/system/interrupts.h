// Interrupt handling

#pragma once

#include <menix/common.h>
#include <menix/util/list.h>

typedef usize Irq;
typedef struct Context Context;

typedef enum
{
	IrqIgnored = 0,			  // Interrupt was not handled.
	IrqHandled = (1 << 0),	  // Handler completed the IRQ work.
	IrqWake = (1 << 1),		  // Handler wants to wake up the handler thread.
} IrqStatus;

typedef enum
{
	IrqFlags_None = 0,
} IrqFlags;

// An IRQ handler callback.
typedef IrqStatus (*IrqHandlerFn)(Irq irq, void* context);

typedef struct IrqAction
{
	struct IrqAction* next;	   // Next action in the list.
	Irq irq;				   // The IRQ number.
	IrqFlags flags;			   // Flags for this action.
	IrqHandlerFn handler;	   // Called directly to handle the IRQ.
	IrqHandlerFn worker;	   // Function to call in a worker thread, if woken up by the handler.
	struct Thread* thread;	   // The thread to execute the worker function on.
	const char* name;		   // Name of the IRQ.
	void* context;			   // A generic context to pass to the handler.
} IrqAction;

extern IrqAction* irq_actions;

// Platform independent handler that runs the given IRQ. To be called by architecture specific interrupt handlers.
void irq_generic_handler(Irq irq);

// Registers a new IRQ handler. Automatically selects optimal CPU placement.
// Returns true upon success.
// `handler`: The main interrupt handler. Must not be NULL.
// `thread_handler`: The threaded handler. Optional.
// `flags`: Flags to control how this IRQ handler behaves.
// `name`: Name of this IRQ.
// `data`: Context to pass to the IRQ on invocation.
bool irq_allocate(IrqHandlerFn handler, IrqHandlerFn thread_handler, IrqFlags flags, const char* name, void* data);
