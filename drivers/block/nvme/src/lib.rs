#![no_std]

use crate::controller::Controller;
use menix::{
    alloc::sync::Arc,
    log,
    memory::{MmioView, PhysAddr},
    posix::errno::{EResult, Errno},
    system::pci::{self, DeviceView, Driver, PciBar, PciVariant},
};

mod command;
mod controller;
mod namespace;
mod queue;
mod spec;

fn probe(
    _: Arc<Driver>,
    _: &PciVariant,
    view: DeviceView<'static>,
) -> EResult<Arc<dyn pci::Device>> {
    log!("Probing NVMe device on {}", view.address());
    let bar = view.bar(0).ok_or(Errno::ENXIO)?;
    let (addr, size) = match bar {
        PciBar::Mmio32 {
            address,
            size,
            prefetchable: _,
        } => (address as usize, size),
        PciBar::Mmio64 {
            address,
            size,
            prefetchable: _,
        } => (address as _, size),
        _ => unreachable!("PCI NVMe devices are MMIO-only"),
    };
    let regs = unsafe { MmioView::new(PhysAddr::new(addr as _), size) };

    // TODO: Support legacy PCI interrupts.
    // Setup MSI-X.
    // let mut cap = view
    //     .capabilities()
    //     .filter_map(|mut x| x.msix())
    //     .next()
    //     .ok_or(Errno::ENXIO)?;

    let mut controller = Controller::new_pci(view.address(), regs)?;

    // Reset the controller to initialize all queues and other structures.
    log!("Resetting controller");
    controller.reset()?;

    Ok(Arc::new(controller))
}

static DRIVER: Driver = Driver {
    name: "nvme",
    probe,
    variants: &[PciVariant::new().class(1).sub_class(8).function(2)],
};

fn main() {
    _ = DRIVER.register();
}

menix::module!("NVMe block devices", "Marvin Friedrich", main);
