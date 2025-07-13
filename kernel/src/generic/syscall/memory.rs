use crate::generic::{
    memory::{VirtAddr, virt::VmFlags},
    posix::errno::EResult,
    vfs::file::MmapFlags,
};

pub fn mmap(
    addr: VirtAddr,
    length: usize,
    prot: u32,
    flags: u32,
    fd: i32,
    offset: uapi::off_t,
) -> EResult<usize> {
    todo!()
}
