use crate::boot::BootInfo;

pub fn early_init() {
    super::serial::init();
    super::idt::init();
}

pub fn init() {}
