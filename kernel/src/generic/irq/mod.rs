use super::{sched::task::Task, util::mutex::Mutex};
use alloc::{string::String, sync::Arc};

pub enum IrqStatus {
    /// Interrupt was not handled.
    Ignored,
    /// Handler completed the IRQ work.
    Handled,
    /// Handler wants to wake up the handler thread.
    Wake,
}

/// An IRQ handler callback.
pub type IrqHandlerFn = fn(irq: usize, context: usize) -> IrqStatus;

pub struct IrqAction {
    pub irq: usize,               // The IRQ number.
    pub handler: IrqHandlerFn,    // Called directly to handle the IRQ.
    pub worker: IrqHandlerFn, // Function to call in a worker thread, if woken up by the handler.
    pub thread: Arc<Mutex<Task>>, // The thread to execute the worker function on.
    pub name: String,         // Name of the IRQ.
    pub context: *mut (),     // A generic context to pass to the handler.
}

pub enum IpiTarget {
    /// Send an interrupt to the calling CPU.
    ThisCpu,
    /// Send an interrupt to all CPUs.
    All,
    /// Send an interrupt to all CPUs except the calling CPU.
    AllButThisCpu,
    /// Send an interrupt to a specific CPU. The value is the ID of the target [`IrqController`].
    Specific(usize),
}

#[derive(Debug)]
pub enum IrqError {
    /// The interrupt controller does not support this operation.
    OperationNotSupported,
}

/// Common functionality for an interrupt controller.
pub trait IrqController {
    /// Gets the ID of this controller.
    fn id(&self) -> usize;
    /// Signals the end of an interrupt to the controller.
    fn eoi(&mut self) -> Result<(), IrqError>;
    /// Sends an inter-processor interrupt to a given `target`.
    fn send_ipi(&self, target: IpiTarget) -> Result<(), IrqError>;
}
