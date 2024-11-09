// Information about the kernel binary
// These are filled in by the linker at link time.

#![allow(unused)]
use crate::arch::VirtAddr;

extern "C" {
    pub static LD_KERNEL_START: VirtAddr;
    pub static LD_KERNEL_END: VirtAddr;
    pub static LD_TEXT_START: VirtAddr;
    pub static LD_TEXT_END: VirtAddr;
    pub static LD_RODATA_START: VirtAddr;
    pub static LD_RODATA_END: VirtAddr;
    pub static LD_DATA_START: VirtAddr;
    pub static LD_DATA_END: VirtAddr;
    pub static LD_MOD_START: VirtAddr;
    pub static LD_MOD_END: VirtAddr;
}
