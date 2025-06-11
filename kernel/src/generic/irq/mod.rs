use crate::generic::process::task::Task;
use alloc::{boxed::Box, string::String, sync::Arc};
use core::{fmt::Debug, sync::atomic::AtomicUsize};

pub enum IrqStatus {
    /// Interrupt was not handled.
    Ignored,
    /// Handler completed the IRQ work.
    Handled,
    /// Handler wants to wake up the handler thread.
    Defer,
}

pub trait IrqHandler: Debug {
    /// Handles an interrupt when it first happens.
    /// If it returns [`IrqStatus::Defer`], then [`IrqHandler::handle_threaded`] is called later.
    fn handle_immediate(&mut self) -> IrqStatus;

    /// Called to complete heavy interrupt work which isn't required to be done immediately.
    fn handle_threaded(&mut self) -> IrqStatus {
        IrqStatus::Handled
    }
}

pub type Irq = usize;

pub struct IrqAction {
    /// The IRQ ID.
    pub irq: Irq,
    /// Callback to invoke.
    pub handler: Box<dyn IrqHandler>,
    /// The thread to execute the worker function on.
    pub thread: Arc<Task>,
    /// Name of the IRQ.
    pub name: String,
}

/// Common functionality for an interrupt controller.
pub trait IrqController {
    fn register(
        &mut self,
        irq: u32,
        name: &str,
        handler: Box<dyn IrqHandler>,
    ) -> Result<Irq, IrqError>;
}

pub enum IpiTarget {
    /// Send an interrupt to the calling CPU.
    ThisCpu,
    /// Send an interrupt to all CPUs.
    All,
    /// Send an interrupt to all CPUs except the calling CPU.
    AllButThisCpu,
    /// Send an interrupt to a specific CPU. The value is the ID of the target [`IrqController`].
    Specific(u32),
}

#[derive(Debug)]
pub enum IrqError {
    /// The interrupt controller does not support this operation.
    OperationNotSupported,
    /// There are no free IRQ slots left.
    NoIrqsLeft,
}

pub static IRQ_COUNTER: AtomicUsize = AtomicUsize::new(0);
