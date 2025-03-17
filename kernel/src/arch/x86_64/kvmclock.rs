use bytemuck::{Pod, Zeroable};

#[repr(C, packed)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct KvmInfo {
    version: u32,
    padding0: u32,
    tsc: u64,
    time: u64,
    tsc_mul: u32,
    tsc_shift: i8,
    flags: u8,
    padding1: [u8; 2],
}
