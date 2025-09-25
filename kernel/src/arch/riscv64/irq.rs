pub unsafe fn set_irq_state(value: bool) -> bool {
    let old: u64;
    unsafe {
        core::arch::asm!(
            "csrr {old}, sie",
            old = out(reg) old,
        );
        if value {
            core::arch::asm!("csrw sie, 1");
        } else {
            core::arch::asm!("csrw sie, 0");
        }
    }

    old != 0
}

pub fn get_irq_state() -> bool {
    let old: u64;
    unsafe {
        core::arch::asm!(
            "csrr {old}, sie",
            old = out(reg) old,
        );
    }

    old != 0
}

pub fn wait_for_irq() {
    unsafe {
        core::arch::asm!("wfi");
    }
}
