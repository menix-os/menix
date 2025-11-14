use crate::{
    command::Command,
    queue::Queue,
    spec::{self},
};
use core::cmp::min;
use menix::{
    alloc::sync::Arc,
    error,
    memory::{BitValue, MmioView, UnsafeMemoryView},
    posix::errno::{EResult, Errno},
    system::pci::Address,
    util::mutex::spin::SpinMutex,
};

const DOORBELL_OFFSET: usize = 0x1000;
const ADMIN_QUEUE_SIZE: usize = 32;
const IO_QUEUE_SIZE_MAX: usize = 1024;

pub struct Controller {
    address: Address,
    regs: Arc<MmioView>,
    version: (u16, u8, u8),
    doorbell_stride: u32,
    admin_queue: SpinMutex<Option<Queue>>,
    io_queue: SpinMutex<Option<Queue>>,
}

impl Controller {
    pub fn new_pci(address: Address, regs: MmioView) -> EResult<Controller> {
        // Read controller version.
        let vs = unsafe { regs.read_reg(spec::regs::VS) }.ok_or(Errno::ENXIO)?;
        let version = (
            vs.read_field(spec::regs::vs::MJR).value(),
            vs.read_field(spec::regs::vs::MNR).value(),
            vs.read_field(spec::regs::vs::TER).value(),
        );

        let cap = unsafe { regs.read_reg(spec::regs::CAP) }.ok_or(Errno::ENXIO)?;

        // Check if our host page size is supported.
        let mps_max = 1usize << (cap.read_field(spec::regs::cap::MPSMAX).value() + 12);
        let mps_min = 1usize << (cap.read_field(spec::regs::cap::MPSMIN).value() + 12);
        let page_size = menix::arch::virt::get_page_size();
        if mps_min > page_size && mps_max < page_size {
            error!("Host page size is not supported on this NVMe!");
            return Err(Errno::ENOTSUP);
        }

        let doorbell_stride = 4u32 << cap.read_field(spec::regs::cap::DSTRD).value();

        Ok(Self {
            address,
            regs: Arc::new(regs),
            version,
            doorbell_stride,
            admin_queue: SpinMutex::new(None),
            io_queue: SpinMutex::new(None),
        })
    }

    pub fn reset(&self) -> EResult<()> {
        let cap = unsafe { self.regs.read_reg(spec::regs::CAP) }.unwrap();

        let queue_depth = min(
            cap.read_field(spec::regs::cap::MQES).value() as usize + 1,
            IO_QUEUE_SIZE_MAX,
        );

        // Create an admin queue first so we can create more queues.
        let admin_queue = Queue::new(0, ADMIN_QUEUE_SIZE, self.regs.clone())?;

        // Set the admin queue sizes.
        let mut aqa = BitValue::new(0);
        aqa = aqa.write_field(spec::regs::aqa::ACQS, (ADMIN_QUEUE_SIZE - 1) as _);
        aqa = aqa.write_field(spec::regs::aqa::ASQS, (ADMIN_QUEUE_SIZE - 1) as _);
        unsafe { self.regs.write_reg(spec::regs::AQA, aqa.value()) };

        // Set the addresses to the admin submission and completion queue.
        unsafe {
            self.regs
                .write_reg(spec::regs::ASQ, admin_queue.get_sq_addr().into());

            self.regs
                .write_reg(spec::regs::ACQ, admin_queue.get_cq_addr().into());
        }

        *self.admin_queue.lock() = Some(admin_queue);

        // Create an IO queue.
        let io_queue = Queue::new(1, queue_depth, self.regs.clone())?;

        // TODO: Setup queue.

        *self.io_queue.lock() = Some(io_queue);

        Ok(())
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
        if let Some(queue) = self.admin_queue.lock().as_mut() {
            queue.submit_cmd(cmd);
        }
    }

    pub fn submit_io_cmd(&self, cmd: Command) {
        if let Some(queue) = self.io_queue.lock().as_mut() {
            queue.submit_cmd(cmd);
        }
    }
}
