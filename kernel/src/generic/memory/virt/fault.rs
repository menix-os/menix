use crate::{
    arch,
    generic::{
        memory::{VirtAddr, cache::MemoryObject, pmm::KernelAlloc, virt::VmFlags},
        sched::Scheduler,
    },
};
use alloc::sync::Arc;
use core::num::NonZeroUsize;

/// Abstract information about a page fault.
pub struct PageFaultInfo {
    /// The instruction pointer address at the point of the page fault.
    pub ip: VirtAddr,
    /// The address that was attempted to access.
    pub addr: VirtAddr,
    /// If set, the fault was caused by a user access.
    pub caused_by_user: bool,
    /// If set, the fault was caused by a write.
    pub caused_by_write: bool,
    /// If set, the fault was caused by an instruction fetch.
    pub caused_by_fetch: bool,
    /// If set, the fault occured in a present page.
    pub page_was_present: bool,
}

/// Generic page fault handler.
pub fn handler(info: &PageFaultInfo) {
    // Check if the current address space has a theoretical mapping at the faulting address.
    let page_size = arch::virt::get_page_size();
    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    let space = &inner.address_space;
    // The page index of the page fault address.
    let faulty_page = info.addr.value() / arch::virt::get_page_size();
    if let Some(mapped) = {
        let mappings = inner.address_space.mappings.lock();
        mappings
            .iter()
            .find(|x| faulty_page >= x.start_page && faulty_page < x.end_page)
            .cloned()
    } {
        // Do copy on write.
        let mut map_flags = mapped.get_flags();
        let mapped_obj = if info.caused_by_write && map_flags.contains(VmFlags::CopyOnWrite) {
            map_flags &= !VmFlags::CopyOnWrite;

            // If there is only one reference to this object, we don't have to CoW.
            if Arc::strong_count(&mapped.object) == 1 {
                mapped.set_flags(map_flags);

                for p in mapped.start_page..mapped.end_page {
                    if space.table.is_mapped((p * page_size).into()) {
                        space
                            .table
                            .remap_single::<KernelAlloc>(info.addr, map_flags)
                            .expect("Unable to remap page for CoW");
                    }
                }

                mapped.object
            } else {
                let new_obj = Arc::new(MemoryObject::new_phys());

                // Copy the data from the old page.
                let mut buf = vec![0u8; page_size];
                let num_pages = mapped.end_page - mapped.start_page;
                for page in 0..num_pages {
                    mapped
                        .object
                        .read(&mut buf, (mapped.offset_page + page) * page_size);
                    new_obj.write(&buf, (mapped.offset_page + page) * page_size);
                }

                inner
                    .address_space
                    .map_object(
                        new_obj.clone(),
                        (mapped.start_page * page_size).into(),
                        NonZeroUsize::new(num_pages * page_size).unwrap(),
                        map_flags,
                        (mapped.offset_page * page_size) as _,
                    )
                    .unwrap();
                new_obj
            }
        } else {
            mapped.object
        };

        if let Some(phys) =
            mapped_obj.try_get_page((faulty_page - mapped.start_page) + mapped.offset_page)
        {
            // If we get here, the accessed address is valid. Map it in the actual page table and return.
            space
                .table
                .map_single::<KernelAlloc>(info.addr, phys, map_flags)
                .expect("Failed to map a demand-loaded page");
            return;
        }
    }

    if info.caused_by_user {
        // TODO: Send SIGSEGV and reschedule.
        // Kill process.
        // Force immediate reschedule.
        panic!(
            "User process caused a segmentation fault! Attempted to {} a {} page at {:#x} (IP: {:#x})",
            if info.caused_by_write {
                "write to"
            } else if info.caused_by_fetch {
                "execute on"
            } else {
                "read from"
            },
            if info.page_was_present {
                "present"
            } else {
                "non-present"
            },
            info.addr.0,
            info.ip.0
        );
    }

    // If any other attempt to recover has failed, we made a mistake.
    panic!(
        "Kernel caused an unrecoverable page fault. Attempted to {} a {} page at {:#x} (IP: {:#x})",
        if info.caused_by_write {
            "write to"
        } else if info.caused_by_fetch {
            "execute on"
        } else {
            "read from"
        },
        if info.page_was_present {
            "present"
        } else {
            "non-present"
        },
        info.addr.0,
        info.ip.0
    );
}
