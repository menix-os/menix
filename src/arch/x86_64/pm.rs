// Physical memory management

use crate::{arch::PhysAddr, boot::BootInfo, memory::pm::CommonPhysManager};
use core::ptr::null_mut;

pub struct PhysManager;
impl CommonPhysManager for PhysManager {
    unsafe fn init(info: &BootInfo) {
        PHYS_BASE = info.hhdm_base as *mut u8;
        // TODO
    }

    unsafe fn alloc(num_pages: usize) -> PhysAddr {
        todo!()
    }

    unsafe fn free(addr: PhysAddr, num_pages: usize) {
        todo!()
    }

    unsafe fn get_phys_base() -> *mut u8 {
        return PHYS_BASE;
    }
}

/// Base address of memory region where physical pages are mapped 1:1
static mut PHYS_BASE: *mut u8 = null_mut();

/// Global bitmap which stores if a page is used or not.
static mut PM_BITMAP: *mut u8 = null_mut();
