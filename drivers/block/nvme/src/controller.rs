use crate::{
    command::Command,
    error::NvmeError,
    queue::Queue,
    spec::{self},
};
use core::cmp::min;
use menix::{
    alloc::sync::Arc,
    error,
    memory::{BitValue, MmioView, UnsafeMemoryView},
    system::pci::Address,
    util::mutex::spin::SpinMutex,
};

const ADMIN_QUEUE_SIZE: usize = 32;
const IO_QUEUE_SIZE_MAX: usize = 1024;

pub struct Controller {
    address: Address,
    regs: Arc<MmioView>,
    version: (u16, u8, u8),
    /// The amount of bytes between doorbells for different queues.
    doorbell_stride: usize,
    admin_queue: SpinMutex<Option<Queue>>,
    io_queue: SpinMutex<Option<Queue>>,
}

impl Controller {
    pub fn new_pci(address: Address, regs: MmioView) -> Result<Self, NvmeError> {
        // Read controller version.
        let vs = unsafe { regs.read_reg(spec::regs::VS) }.ok_or(NvmeError::RegisterOutOfBounds)?;
        let version = (
            vs.read_field(spec::regs::vs::MJR).value(),
            vs.read_field(spec::regs::vs::MNR).value(),
            vs.read_field(spec::regs::vs::TER).value(),
        );

        let cap =
            unsafe { regs.read_reg(spec::regs::CAP) }.ok_or(NvmeError::RegisterOutOfBounds)?;

        // Check if our host page size is supported.
        let mps_max = 1usize << (cap.read_field(spec::regs::cap::MPSMAX).value() + 12);
        let mps_min = 1usize << (cap.read_field(spec::regs::cap::MPSMIN).value() + 12);
        let page_size = menix::arch::virt::get_page_size();
        if mps_min > page_size && mps_max < page_size {
            error!("Host page size is not supported on this NVMe!");
            return Err(NvmeError::UnsupportedPageSize);
        }

        let doorbell_stride = 4 << cap.read_field(spec::regs::cap::DSTRD).value();

        Ok(Self {
            address,
            regs: Arc::new(regs),
            version,
            doorbell_stride,
            admin_queue: SpinMutex::new(None),
            io_queue: SpinMutex::new(None),
        })
    }

    pub fn reset(&self) -> Result<(), NvmeError> {
        // Disable the controller.
        self.set_status(false)?;

        let cap = unsafe { self.regs.read_reg(spec::regs::CAP) }.unwrap();

        let queue_depth = min(
            cap.read_field(spec::regs::cap::MQES).value() as usize + 1,
            IO_QUEUE_SIZE_MAX,
        );

        // Create an admin queue first so we can create more queues.
        let mut admin_queue =
            Queue::new(self.regs.clone(), self.doorbell_stride, 0, ADMIN_QUEUE_SIZE)?;

        // Set the admin queue sizes.
        let aqa = BitValue::new(0)
            .write_field(spec::regs::aqa::ACQS, (admin_queue.get_depth() - 1) as _)
            .write_field(spec::regs::aqa::ASQS, (admin_queue.get_depth() - 1) as _);
        unsafe { self.regs.write_reg(spec::regs::AQA, aqa.value()) };

        // Set the addresses to the admin submission and completion queue.
        unsafe {
            self.regs
                .write_reg(spec::regs::ASQ, admin_queue.get_sq_addr().into());

            self.regs
                .write_reg(spec::regs::ACQ, admin_queue.get_cq_addr().into());
        }

        // Re-enable the controller.
        self.set_status(true)?;

        // Create an IO queue.
        let io_queue = Queue::new(self.regs.clone(), self.doorbell_stride, 1, queue_depth)?;
        io_queue.setup_io(&mut admin_queue)?;

        *self.admin_queue.lock() = Some(admin_queue);
        *self.io_queue.lock() = Some(io_queue);

        Ok(())
    }

    /// Sets the CC.EN flag.
    fn set_status(&self, enable: bool) -> Result<(), NvmeError> {
        unsafe {
            let cc = self
                .regs
                .read_reg(spec::regs::CC)
                .ok_or(NvmeError::RegisterOutOfBounds)?
                .write_field(spec::regs::cc::EN, if enable { 1 } else { 0 });
            self.regs.write_reg(spec::regs::CC, cc.value());
        }

        Ok(())
    }

    /// Scans all existing namespaces.
    pub fn scan_ns(&self) -> Result<(), NvmeError> {
        todo!()
    }

    /// Creates a new namespace on this controller.
    pub fn create_ns(&self, nsid: u32) -> Result<(), NvmeError> {
        todo!()
    }

    pub fn submit_admin_cmd(&self, cmd: impl Command) -> Result<(), NvmeError> {
        self.admin_queue
            .lock()
            .as_mut()
            .map_or(Err(NvmeError::MissingQueue), |queue| queue.submit_cmd(cmd))
    }

    pub fn submit_io_cmd(&self, cmd: impl Command) -> Result<(), NvmeError> {
        self.io_queue
            .lock()
            .as_mut()
            .map_or(Err(NvmeError::MissingQueue), |queue| queue.submit_cmd(cmd))
    }
}
