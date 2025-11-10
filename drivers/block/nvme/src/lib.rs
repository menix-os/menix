#![no_std]

use menix::{
    alloc::sync::Arc,
    log,
    memory::{MemoryView, MmioView, PhysAddr},
    posix::errno::{EResult, Errno},
    system::pci::{self, Address, DeviceView, Driver, PciBar, PciVariant},
};

mod commands;
mod queue;
mod spec;

struct NvmeController {
    address: Address,
    driver: &'static Driver,
    version: (u16, u8, u8),
    regs: MmioView,
}

impl pci::Device for NvmeController {
    fn address(&self) -> Address {
        self.address
    }

    fn driver(&self) -> &'static Driver {
        self.driver
    }
}

fn probe(_: &PciVariant, mut view: DeviceView<'static>) -> EResult<Arc<dyn pci::Device>> {
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
    let mut regs = unsafe { MmioView::new(PhysAddr::new(addr as _), size) };

    // Read controller version.
    let vs = regs.read_reg(spec::regs::VS).ok_or(Errno::ENXIO)?;
    let version = (
        vs.read_field(spec::regs::MJR).value(),
        vs.read_field(spec::regs::MNR).value(),
        vs.read_field(spec::regs::TER).value(),
    );

    log!(
        "Controller version {}.{}.{}",
        version.0,
        version.1,
        version.2
    );

    let cap = regs.read_reg(spec::regs::CAP).ok_or(Errno::ENXIO)?;
    let mpsmax = cap.read_field(spec::regs::MPSMAX).value();
    let mpsmin = cap.read_field(spec::regs::MPSMIN).value();
    log!(
        "mpsmin = {:#x}, mpsmax = {:#x}",
        1 << (mpsmin as usize + 12),
        1 << (mpsmax as usize + 12)
    );

    // TODO: Support legacy PCI interrupts.
    // Setup MSI-X.
    let mut cap = view
        .capabilities()
        .filter_map(|mut x| x.msix())
        .next()
        .ok_or(Errno::ENXIO)?;
    cap.set_state(true);

    Ok(Arc::new(NvmeController {
        address: view.address(),
        driver: &DRIVER,
        version,
        regs,
    }))
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
