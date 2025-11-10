use alloc::sync::Arc;
use core::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};

mod msi;
pub use msi::*;

#[derive(Debug)]
pub enum IrqStatus {
    /// Interrupt was not handled.
    Ignored,
    /// Handler completed the IRQ work.
    Handled,
    /// Handler wants to wake up the handler thread.
    Defer,
}

pub enum Polarity {
    Low,
    High,
}

pub enum Trigger {
    Edge,
    Level,
}

pub trait IrqHandler: Debug {
    /// Handles an interrupt when it first happens.
    /// If it returns [`IrqStatus::Defer`], then [`IrqHandler::handle_threaded`] is called later.
    fn handle_immediate(&self) -> IrqStatus;

    /// Called to complete heavy interrupt work which isn't required to be done immediately.
    fn handle_threaded(&self) -> IrqStatus {
        IrqStatus::Handled
    }
}

pub type Irq = usize;

/// Common functionality for an interrupt controller.
pub trait IrqController {
    /// Registers an IRQ handler for a specific IRQ.
    /// If `thread` is [`Some`], a second handler will be run in a separate thread.
    fn register(
        &self,
        name: &str,
        handler: Arc<dyn IrqHandler>,
        threaded_handler: Option<Arc<dyn IrqHandler>>,
        line: u32,
        polarity: Polarity,
        trigger: Trigger,
    ) -> Result<Irq, IrqError>;

    /// Removes an IRQ handler.
    fn remove(&self, irq: Irq) -> Result<(), IrqError>;

    /// Masks an IRQ, preventing it from being triggered.
    fn mask(&self, irq: Irq) -> Result<(), IrqError>;

    /// Unmasks an IRQ, allowing it to be triggered.
    fn unmask(&self, irq: Irq) -> Result<(), IrqError>;
}

#[derive(Debug)]
pub enum IrqError {
    /// The interrupt controller does not support this operation.
    OperationNotSupported,
    /// There are no free IRQ slots left.
    NoIrqsLeft,
    /// The IRQ ID is invalid.
    NoSuchIrq,
    /// The IRQ is already registered.
    AlreadyRegistered,
    /// The IRQ ID is out of range for this controller.
    LineOutOfRange,
}

static IRQ_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Allocates a new IRQ handle.
pub fn allocate_irq() -> Irq {
    IRQ_COUNTER.fetch_add(1, Ordering::Acquire)
}
