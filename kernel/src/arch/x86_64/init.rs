use crate::{boot::BootInfo, generic::percpu::PerCpu};
use core::{arch::asm, mem::offset_of, sync::atomic::AtomicPtr};

pub fn early_init() {
    super::serial::init();
    super::idt::init();
}

pub fn init(info: &mut BootInfo) {}
