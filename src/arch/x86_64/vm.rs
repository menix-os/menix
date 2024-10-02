// Virtual memory management

use crate::memory::vm::CommonVirtManager;

pub struct VirtManager;
impl CommonVirtManager for VirtManager {
    unsafe fn init(info: &crate::boot::BootInfo) {
        todo!()
    }
}
