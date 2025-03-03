// Task State Segment implementation

use core::mem::size_of;

#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved0: u32,
    rsp0: u64,
    rsp1: u64,
    rsp2: u64,
    reserved1: u32,
    reserved2: u32,
    ist1: u64,
    ist2: u64,
    ist3: u64,
    ist4: u64,
    ist5: u64,
    ist6: u64,
    ist7: u64,
    reserved3: u32,
    reserved4: u32,
    reserved5: u16,
    iopb: u16,
}

impl TaskStateSegment {
    pub const fn new() -> Self {
        Self {
            reserved0: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            reserved1: 0,
            reserved2: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            reserved3: 0,
            reserved4: 0,
            reserved5: 0,
            iopb: 0,
        }
    }
}

pub fn init() {
    unsafe {
        TSS_STORAGE.rsp0 = 0;
        TSS_STORAGE.rsp1 = 0;
        TSS_STORAGE.rsp2 = 0;
        TSS_STORAGE.iopb = size_of::<TaskStateSegment>() as u16;
    }
}

pub static mut TSS_STORAGE: TaskStateSegment = TaskStateSegment::new();
