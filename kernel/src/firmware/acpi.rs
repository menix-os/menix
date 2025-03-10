// Advanced Configuration and Power Interface
// Wrapper for uacpi-rs

use crate::{arch::PhysAddr, generic::percpu::setup_cpu};

pub fn init(rsdp: PhysAddr) {
    print!("acpi: ACPI RSDP at 0x{:x}\n", rsdp);

    print!("acpi: Booting CPUs using MADT\n");
    // Setup the boot CPU.
    setup_cpu();
    // TODO: Evaluate MADT and initialize all remaining CPUs.
}
