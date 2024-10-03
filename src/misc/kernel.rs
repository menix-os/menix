// Information about the kernel binary

extern "C" {
    static LD_KERNEL_START: usize;
    static LD_KERNEL_END: usize;
    static LD_TEXT_START: usize;
    static LD_TEXT_END: usize;
    static LD_RODATA_START: usize;
    static LD_RODATA_END: usize;
    static LD_DATA_START: usize;
    static LD_DATA_END: usize;
    static LD_MOD_START: usize;
    static LD_MOD_END: usize;
}
