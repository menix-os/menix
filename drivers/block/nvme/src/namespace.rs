use crate::controller::Controller;
use menix::{device::block::BlockDevice, posix::errno::EResult};

pub struct Namespace<'a> {
    controller: &'a Controller,
    nsid: usize,
    lba_shift: usize,
    lba_count: usize,
}

impl<'a> Namespace<'a> {
    pub fn new(
        controller: &'a Controller,
        nsid: usize,
        lba_shift: usize,
        lba_count: usize,
    ) -> Self {
        Self {
            controller,
            nsid,
            lba_shift,
            lba_count,
        }
    }
}

impl<'a> BlockDevice for Namespace<'a> {
    fn get_sector_size(&self) -> usize {
        1 << self.lba_shift
    }

    fn read_sectors(&self, buffer: &mut [u8], sector_start: usize) -> EResult<usize> {
        todo!()
    }

    fn write_sectors(&self, buffer: &[u8], sector_start: usize) -> EResult<usize> {
        todo!()
    }
}
