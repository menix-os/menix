use menix::{device::block::BlockDevice, posix::errno::EResult};

pub struct Namespace {
    nsid: usize,
    lba_shift: usize,
    lba_count: usize,
}

impl Namespace {
    pub fn new(nsid: usize, lba_shift: usize, lba_count: usize) -> Self {
        Self {
            nsid,
            lba_shift,
            lba_count,
        }
    }
}

impl BlockDevice for Namespace {
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
