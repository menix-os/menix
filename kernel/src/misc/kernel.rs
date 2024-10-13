// Information about the kernel binary

#![allow(unused)]
extern "C" {
    pub static LD_KERNEL_START: usize;
    pub static LD_KERNEL_END: usize;
    pub static LD_TEXT_START: usize;
    pub static LD_TEXT_END: usize;
    pub static LD_RODATA_START: usize;
    pub static LD_RODATA_END: usize;
    pub static LD_DATA_START: usize;
    pub static LD_DATA_END: usize;
    pub static LD_MOD_START: usize;
    pub static LD_MOD_END: usize;
}
