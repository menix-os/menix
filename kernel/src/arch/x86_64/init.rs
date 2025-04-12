use super::{apic, idt, serial};

pub fn early_init() {
    apic::disable_legacy_pic();
    serial::init();
    idt::init();
}

pub fn init() {}
