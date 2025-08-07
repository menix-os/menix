use crate::{
    arch::virt::get_page_size,
    generic::{
        memory::{VirtAddr, virt::VmFlags},
        posix::errno::{EResult, Errno},
        sched::Scheduler,
        util::align_up,
        vfs::file::MmapFlags,
    },
};
use core::num::NonZeroUsize;

pub fn mmap(
    addr: VirtAddr,
    length: usize,
    prot: u32,
    flags: u32,
    fd: i32,
    offset: uapi::off_t,
) -> EResult<usize> {
    let flags = MmapFlags::from_bits_truncate(flags);

    // Flags must contain either MAP_PRIVATE or MAP_SHARED. Not both or none.
    if flags.intersects(MmapFlags::Shared | MmapFlags::Private) {
        if flags.contains(MmapFlags::Shared | MmapFlags::Private) {
            return Err(Errno::EINVAL);
        }
    } else {
        return Err(Errno::EINVAL);
    }

    let mut vm_prot = VmFlags::empty();
    vm_prot.set(VmFlags::Read, prot & uapi::PROT_READ != 0);
    vm_prot.set(VmFlags::Write, prot & uapi::PROT_WRITE != 0);
    vm_prot.set(VmFlags::Exec, prot & uapi::PROT_EXEC != 0);

    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.inner.lock();

    // If MAP_FIXED isn't specified, we must find a suitable address.
    let addr = if !flags.contains(MmapFlags::Fixed) {
        let cur = proc_inner.mmap_head;
        proc_inner.mmap_head = align_up((cur + length).value(), get_page_size()).into();

        cur
    } else {
        addr
    };

    let file = match flags.contains(MmapFlags::Anonymous) {
        true => None,
        false => {
            // Look up the corresponding fd.
            Some(
                proc_inner
                    .open_files
                    .get(&(fd as usize))
                    .ok_or(Errno::EBADF)
                    .map(|x| x.clone())?,
            )
        }
    };
    crate::generic::vfs::mmap(
        file,
        &proc_inner.address_space,
        addr,
        NonZeroUsize::new(length).ok_or(Errno::EINVAL)?,
        vm_prot,
        flags,
        offset,
    )
    .map(|x| x.value())
}

pub fn mprotect(addr: VirtAddr, size: usize, prot: u32) -> EResult<usize> {
    let mut vm_prot = VmFlags::empty();
    vm_prot.set(VmFlags::Read, prot & uapi::PROT_READ != 0);
    vm_prot.set(VmFlags::Write, prot & uapi::PROT_WRITE != 0);
    vm_prot.set(VmFlags::Exec, prot & uapi::PROT_EXEC != 0);

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    proc_inner.address_space.protect(
        addr,
        NonZeroUsize::new(size).ok_or(Errno::EINVAL)?,
        vm_prot,
    )?;

    Ok(0)
}
