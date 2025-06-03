use super::util::mutex::Mutex;
use alloc::{boxed::Box, collections::btree_map::BTreeMap};
use core::{fmt::Debug, sync::atomic::AtomicUsize};

pub enum IrqStatus {
    /// Interrupt was not handled.
    Ignored,
    /// Handler completed the IRQ work.
    Handled,
    /// Handler wants to wake up the handler thread.
    Wake,
}

pub trait IrqHandler: Debug {
    /// Handles an interrupt.
    fn handle(&mut self) -> IrqStatus;
}

// TODO
// pub struct IrqAction {
//     pub irq: usize,                   // The IRQ number.
//     pub handler: Box<dyn IrqHandler>, // Called directly to handle the IRQ.
//     pub worker: IrqHandlerFn, // Function to call in a worker thread, if woken up by the handler.
//     pub thread: Arc<Mutex<Task>>, // The thread to execute the worker function on.
//     pub name: String,         // Name of the IRQ.
//     pub context: *mut (),     // A generic context to pass to the handler.
// }

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
    /// There are no free IRQ slots left.
    NoIrqsLeft,
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

static IRQ_COUNTER: AtomicUsize = AtomicUsize::new(0);
static IRQ_HANDLERS: Mutex<BTreeMap<usize, Mutex<Box<dyn IrqHandler>>>> =
    Mutex::new(BTreeMap::new());

// TODO
pub fn register_irq(action: Box<dyn IrqHandler>) -> Result<usize, IrqError> {
    let mut handlers = IRQ_HANDLERS.lock();

    let irq = IRQ_COUNTER.fetch_add(1, core::sync::atomic::Ordering::Acquire);
    handlers.insert(irq, Mutex::new(action));

    crate::arch::irq::register_irq(irq)?;

    return Ok(irq);
}

// TODO
/// Dispatches the handler for a given IRQ.
pub fn dispatch(irq: usize) {
    let handlers = IRQ_HANDLERS.lock();

    match handlers.get(&irq) {
        Some(x) => {
            x.lock().handle();
        }
        None => (),
    }
}
