use super::sched::thread::Thread;
use alloc::{string::String, sync::Arc};
use spin::Mutex;

pub enum IrqStatus {
    /// Interrupt was not handled.
    Ignored = 0,
    /// Handler completed the IRQ work.
    Handled = (1 << 0),
    /// Handler wants to wake up the handler thread.
    Wake = (1 << 1),
}

/// An IRQ handler callback.
pub type IrqHandlerFn = fn(irq: usize, context: *mut u8) -> IrqStatus;

pub struct IrqAction {
    irq: usize,                 // The IRQ number.
    handler: IrqHandlerFn,      // Called directly to handle the IRQ.
    worker: IrqHandlerFn,       // Function to call in a worker thread, if woken up by the handler.
    thread: Arc<Mutex<Thread>>, // The thread to execute the worker function on.
    name: String,               // Name of the IRQ.
    context: *mut u8,           // A generic context to pass to the handler.
}
