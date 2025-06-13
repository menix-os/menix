use alloc::boxed::Box;
use core::{fmt::Debug, sync::atomic::AtomicUsize};

#[derive(Debug)]
pub enum IrqStatus {
    /// Interrupt was not handled.
    Ignored,
    /// Handler completed the IRQ work.
    Handled,
    /// Handler wants to wake up the handler thread.
    Defer,
}

#[derive(Debug)]
pub enum IrqFlags {
    /// The IRQ is edge-triggered.
    Edge,
    /// The IRQ is level-triggered.
    Level,
    /// The IRQ is active low.
    ActiveLow,
    /// The IRQ is active high.
    ActiveHigh,
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

/// Common functionality for an interrupt controller.
pub trait IrqController {
    /// Registers an IRQ handler for a specific IRQ.
    /// If `thread` is [`Some`], a second handler will be run in a separate thread.
    fn register(
        &self,
        name: &str,
        handler: Box<dyn IrqHandler>,
        thread: Option<Box<dyn IrqHandler>>,
        line: u32,
        flags: IrqFlags,
    ) -> Result<Irq, IrqError>;

    /// Removes an IRQ handler for a specific IRQ.
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
    OutOfRange,
}

pub static IRQ_COUNTER: AtomicUsize = AtomicUsize::new(0);
