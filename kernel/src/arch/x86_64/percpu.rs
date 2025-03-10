use super::consts;
use super::{
    asm,
    gdt::{self, Gdt},
    idt,
    tss::TaskStateSegment,
};
use crate::arch::x86_64::asm::cpuid;
use crate::generic::percpu::PerCpu;
use core::arch::asm;
use core::mem::offset_of;

#[derive(Debug)]
#[repr(align(0x10))]
pub struct ArchPerCpu {
    /// Processor local Global Descriptor Table.
    /// The GDT refers to a different TSS every time, so unlike the IDT it has to exist for each processor.
    gdt: Gdt,
    tss: TaskStateSegment,
    lapic_id: u64,
}

impl ArchPerCpu {
    pub fn new() -> Self {
        Self {
            gdt: Gdt::new(),
            tss: TaskStateSegment::new(),
            lapic_id: 0,
        }
    }
}

/// Initializes architecture dependent data for the current processor.
pub fn setup_cpu(cpu: &mut PerCpu) {
    // Print CPUID identification string.
    {
        let (mut a, mut b, mut c, mut d) = (0x80000002, 0, 0, 0);
        cpuid(&mut a, &mut b, &mut c, &mut d);
        let cpu_name = [b, d, c];
        print!("percpu: CPU identification is \"{}\".\n", unsafe {
            str::from_utf8_unchecked(bytemuck::cast_slice(&cpu_name))
        });
    }

    // Load a GDT and TSS.
    gdt::init(&mut cpu.arch.gdt, &mut cpu.arch.tss);

    // Load the IDT.
    // Note: The IDT itself is global, but still needs to be loaded for each CPU.
    idt::set_idt();

    // Mask the legacy PIC.

    // Enable the `syscall` extension.
    let msr = asm::rdmsr(consts::MSR_EFER);
    asm::wrmsr(consts::MSR_EFER, msr | consts::MSR_EFER_SCE as u64);
    // Bits 32-47 are kernel segment base, Bits 48-63 are user segment base. Lower 32 bits (EIP) are unused.
    asm::wrmsr(
        consts::MSR_STAR,
        ((offset_of!(Gdt, kernel_code))
            | ((offset_of!(Gdt, user_code) | consts::CPL_USER as usize) << 16) << 32)
            as u64,
    );
    // Set syscall entry point.
    asm::wrmsr(
        consts::MSR_LSTAR,
        super::interrupts::amd64_syscall_stub as u64,
    );
    // Set the flag mask to everything except the second bit (always has to be enabled).
    asm::wrmsr(consts::MSR_SFMASK, (!2u32) as u64);

    // Now, start manipulating the control registers.
    let mut cr0 = 0usize;
    unsafe { asm!("mov {cr0}, cr0", cr0 = out(reg) cr0) };
    let mut cr4 = 0usize;
    unsafe { asm!("mov {cr4}, cr4", cr4 = out(reg) cr4) };

    // Enable SSE.
}
