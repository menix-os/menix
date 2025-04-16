use super::VirtAddr;
use crate::arch::irq::InterruptFrame;

/// The origin of the page fault.
pub enum PageFaultKind {
    /// Issue unclear (possible corruption).
    Unknown,
    /// Page is not mapped in the current page table.
    NotMapped,
    /// Page is mapped, but can't be read from.
    IllegalRead,
    /// Page is mapped, but can't be written to.
    IllegalWrite,
    /// Page is mapped, but can't be executed on.
    IllegalExecute,
}

/// Abstract information about a page fault.
pub struct PageFaultInfo {
    /// Fault caused by the user.
    pub caused_by_user: bool,
    /// The instruction pointer address.
    pub ip: VirtAddr,
    /// The address that was attempted to access.
    pub addr: VirtAddr,
    /// The cause of this page fault.
    pub kind: PageFaultKind,
}

/// Generic page fault handler. May reschedule and return a different context.
pub fn page_fault_handler<'a>(
    context: &'a InterruptFrame,
    info: &PageFaultInfo,
) -> &'a InterruptFrame {
    if info.caused_by_user {
        // TODO: Send SIGSEGV.
        return context;
    }

    panic!(
        "Kernel caused an unrecoverable page fault! IP: {:#x}, Address: {:#x}",
        info.ip.0, info.addr.0
    );
}
