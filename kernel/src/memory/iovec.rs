use crate::{arch, memory::VirtAddr, posix::errno::EResult, uapi::uio::iovec};

pub struct IoVecList<'a> {
    iovecs: &'a [iovec],
    total_len: usize,
    total_offset: usize,
    current_idx: usize,
    current_offset: usize,
}

impl<'a> IoVecList<'a> {
    pub fn new(iovecs: &'a [iovec]) -> Self {
        Self {
            iovecs,
            total_len: iovecs.iter().map(|x| x.len).sum(),
            total_offset: 0,
            current_idx: 0,
            current_offset: 0,
        }
    }

    /// Returns true, if all [`iovec`]s are userspace addresses.
    pub fn is_user_only(&self) -> bool {
        for i in self.iovecs {
            // When len == 0, the address is irrelevant.
            if i.len != 0 && !arch::virt::is_user_addr(i.base.addr() + VirtAddr::new(i.len)) {
                return false;
            }
        }
        true
    }

    pub fn copy_from_slice(&self, slice: &[u8]) -> EResult<()> {
        todo!()
    }

    pub fn copy_to_slice(&self, slice: &mut [u8]) -> EResult<()> {
        todo!()
    }
}
