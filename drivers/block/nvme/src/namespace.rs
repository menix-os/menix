use crate::{command::ReadWriteCommand, controller::Controller};
use core::num::NonZeroUsize;
use menix::{
    alloc::sync::Arc,
    device::BlockDevice,
    log,
    memory::{PhysAddr, VirtAddr},
    posix::errno::{EResult, Errno},
    vfs::File,
};

pub struct Namespace {
    controller: Arc<Controller>,
    nsid: u32,
    lba_shift: u8,
    lba_count: u64,
}

impl Namespace {
    pub fn new(controller: Arc<Controller>, nsid: u32, lba_shift: u8, lba_count: u64) -> Self {
        log!(
            "New namespace: ID {nsid}, LBA size {} bytes, {} MBs total",
            1 << lba_shift,
            (lba_count << lba_shift) / 1024 / 1024
        );
        Self {
            controller,
            nsid,
            lba_shift,
            lba_count,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.nsid
    }
}

impl BlockDevice for Namespace {
    fn get_lba_size(&self) -> usize {
        1 << self.lba_shift
    }

    fn read_lba(&self, buffer: PhysAddr, sector_start: u64) -> EResult<()> {
        if sector_start >= self.lba_count {
            return Err(Errno::ENXIO);
        }

        let mut ioq_guard = self.controller.io_queue.lock();
        let ioq = ioq_guard.as_mut().ok_or(Errno::EIO)?;

        ioq.submit_cmd(ReadWriteCommand {
            buffer,
            do_write: false,
            start_lba: sector_start,
            length: NonZeroUsize::new(1).unwrap(), // 1 LBA
            control: 0,
            ds_mgmt: 0,
            ref_tag: 0,
            app_tag: 0,
            app_mask: 0,
            nsid: self.nsid,
        })
        .map_err(|_| Errno::ENXIO)?;

        if !ioq.next_completion().unwrap().status.is_success() {
            return Err(Errno::ENXIO);
        }

        Ok(())
    }

    fn write_lba(&self, buffer: PhysAddr, sector_start: u64) -> EResult<()> {
        let mut ioq_guard = self.controller.io_queue.lock();
        let ioq = ioq_guard.as_mut().ok_or(Errno::EIO)?;

        ioq.submit_cmd(ReadWriteCommand {
            buffer,
            do_write: true,
            start_lba: sector_start,
            length: NonZeroUsize::new(1).unwrap(), // 1 LBA
            control: 0,
            ds_mgmt: 0,
            ref_tag: 0,
            app_tag: 0,
            app_mask: 0,
            nsid: self.nsid,
        })
        .map_err(|_| Errno::ENXIO)?;

        if !ioq.next_completion().unwrap().status.is_success() {
            return Err(Errno::ENXIO);
        }

        Ok(())
    }

    fn handle_ioctl(&self, file: &File, request: usize, arg: VirtAddr) -> EResult<usize> {
        let _ = (file, request, arg);
        Err(Errno::EINVAL)
    }
}
