use menix::{
    error,
    memory::{MemoryView, MmioView},
    posix::errno::{EResult, Errno},
    system::pci::{self, Address},
    util::mutex::spin::SpinMutex,
};

use crate::{command::Command, spec};

pub struct Controller {
    address: Address,
    regs: SpinMutex<MmioView>,
    version: (u16, u8, u8),
    doorbell_stride: u32,
}

impl Controller {
    pub fn new_pci(address: Address, regs: MmioView) -> EResult<Self> {
        // Read controller version.
        let vs = regs.read_reg(spec::regs::VS).ok_or(Errno::ENXIO)?;
        let version = (
            vs.read_field(spec::regs::MJR).value(),
            vs.read_field(spec::regs::MNR).value(),
            vs.read_field(spec::regs::TER).value(),
        );

        let cap = regs.read_reg(spec::regs::CAP).ok_or(Errno::ENXIO)?;

        // Check if our host page size is supported.
        let mps_max = 1usize << (cap.read_field(spec::regs::MPSMAX).value() + 12);
        let mps_min = 1usize << (cap.read_field(spec::regs::MPSMIN).value() + 12);
        let page_size = menix::arch::virt::get_page_size();
        if mps_min > page_size && mps_max < page_size {
            error!("Host page size is not supported on this NVMe!");
            return Err(Errno::ENOTSUP);
        }

        let doorbell_stride = 4u32 << cap.read_field(spec::regs::DSTRD).value();

        Ok(Self {
            address,
            regs: SpinMutex::new(regs),
            version,
            doorbell_stride,
        })
    }

    pub fn reset(&self) {
        todo!()
    }

    /// Scans all existing namespaces.
    pub fn scan_ns(&self) {
        todo!()
    }

    /// Creates a new namespace on this controller.
    pub fn create_ns(&self, nsid: u32) {
        todo!()
    }

    pub fn submit_admin_cmd(&self, cmd: Command) {
        todo!()
    }

    pub fn submit_io_cmd(&self, cmd: Command) {
        todo!()
    }
}

impl pci::Device for Controller {
    fn address(&self) -> Address {
        self.address
    }
}
