use super::{
    apic::{self, LocalApic},
    asm, consts,
    gdt::{self, Gdt, TaskStateSegment},
    idt, serial,
};
use crate::{
    arch::x86_64::tsc::{self, TscClock},
    generic::{
        clock,
        cpu::{CpuData, LD_PERCPU_START},
    },
    per_cpu,
};
use crate::{
    arch::{self, x86_64::asm::cpuid},
    generic::{self, cpu, irq::IrqHandlerFn},
};
use alloc::boxed::Box;
use core::{arch::asm, ffi::CStr, mem::offset_of, ptr::null_mut};

#[derive(Debug)]
#[repr(C)]
pub struct ArchPerCpu {
    /// Processor local Global Descriptor Table.
    /// The GDT refers to a different TSS every time, so unlike the IDT it has to exist for each processor.
    pub gdt: Gdt,
    pub tss: TaskStateSegment,
    /// Callback functions to handle a given interrupt.
    pub irq_handlers: [Option<IrqHandlerFn>; 256],
    /// A map of ISRs to IRQs.
    pub irq_map: [usize; 256],
    /// Context passed to an IRQ handler.
    pub irq_ctx: [usize; 256],
    /// The Local APIC ID.
    pub lapic_id: u64,
    /// Size of the FPU.
    pub fpu_size: usize,
    /// Function called to save the FPU context.
    pub fpu_save: unsafe fn(memory: *mut u8),
    /// Function called to restore the FPU context.
    pub fpu_restore: unsafe fn(memory: *const u8),
    /// If this CPU supports the STAC/CLAC instructions.
    pub can_smap: bool,
}

per_cpu!(
    CPU_DATA,
    ArchPerCpu,
    ArchPerCpu {
        gdt: Gdt::new(),
        tss: TaskStateSegment::new(),
        irq_handlers: [None; 256],
        irq_map: [0; 256],
        irq_ctx: [0; 256],
        lapic_id: 0,
        fpu_size: 512,
        fpu_save: asm::fxsave,
        fpu_restore: asm::fxrstor,
        can_smap: false,
    }
);

/// Returns the per-CPU data of this CPU.
/// # Safety
/// Accessing this data directly is inherently unsafe without first disabling preemption!
#[inline]
pub(crate) unsafe fn get_per_cpu() -> *mut CpuData {
    unsafe {
        let cpu: *mut CpuData;
        asm!(
            "mov {cpu}, gs:[{this}]",
            cpu = out(reg) cpu,
            this = const offset_of!(CpuData, this),
            options(nostack, preserves_flags),
        );
        return cpu;
    }
}

pub(crate) fn setup_bsp() {
    apic::disable_legacy_pic();
    serial::init();
    idt::init();

    // Check if the FSGSBASE feature is available.
    let cpuid7 = unsafe { cpuid(7, 0) };
    assert!(
        cpuid7.ebx & consts::CPUID_7B_FSGSBASE != 0,
        "FSGSBASE is required for the kernel to function, but the bit wasn't set"
    );

    // Enable the FSGSBASE bit.
    let mut cr4 = 0usize;
    unsafe { asm!("mov {cr4}, cr4", cr4 = out(reg) cr4) };
    cr4 |= consts::CR4_FSGSBASE;
    unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4) };

    // Set FSGSBASE contents.
    unsafe {
        asm::wrmsr(consts::MSR_KERNEL_GS_BASE, 0);
        asm::wrmsr(consts::MSR_GS_BASE, &raw const LD_PERCPU_START as u64);
        asm::wrmsr(consts::MSR_FS_BASE, 0);
    }
}

/// Initializes architecture dependent data for the current processor.
pub fn setup(context: &mut CpuData) {
    // Allocate a new CPU.
    let mut cpu = CPU_DATA.get(context);

    // Print CPUID identification string.
    unsafe {
        let m = cpuid(0, 0);
        let manufacturer = [m.ebx, m.edx, m.ecx, 0];
        let (n0, n1, n2) = (
            cpuid(0x8000_0002, 0),
            cpuid(0x8000_0003, 0),
            cpuid(0x8000_0004, 0),
        );
        let cpu_name = [
            n0.eax, n0.ebx, n0.ecx, n0.edx, n1.eax, n1.ebx, n1.ecx, n1.edx, n2.eax, n2.ebx, n2.ecx,
            n2.edx, 0,
        ];
        print!(
            "percpu: {}, {}\n",
            CStr::from_bytes_until_nul(bytemuck::cast_slice(&manufacturer))
                .unwrap()
                .to_str()
                .unwrap(),
            CStr::from_bytes_until_nul(bytemuck::cast_slice(&cpu_name))
                .unwrap()
                .to_str()
                .unwrap()
        );
    }

    // Load a GDT and TSS.
    gdt::init(&mut cpu.gdt, &mut cpu.tss);

    // Load the IDT.
    // Note: The IDT itself is global, but still needs to be loaded for each CPU.
    idt::set_idt();

    // Enable the `syscall` extension.
    unsafe {
        let msr = asm::rdmsr(consts::MSR_EFER);
        asm::wrmsr(consts::MSR_EFER, msr | consts::MSR_EFER_SCE as u64);
        // Bits 32-47 are kernel segment base, Bits 48-63 are user segment base. Lower 32 bits (EIP) are unused.
        asm::wrmsr(
            consts::MSR_STAR,
            ((offset_of!(Gdt, user_code) | consts::CPL_USER as usize) as u64) << 48
                | (offset_of!(Gdt, kernel_code) as u64) << 32,
        );
        // Set syscall entry point.
        asm::wrmsr(
            consts::MSR_LSTAR,
            super::irq::amd64_syscall_stub as usize as u64,
        );
        // Set the flag mask to everything except the second bit (always has to be enabled).
        asm::wrmsr(consts::MSR_SFMASK, (!2u32) as u64);
    }

    // Now, start manipulating the control registers.
    let mut cr0 = 0usize;
    unsafe { asm!("mov {cr0}, cr0", cr0 = out(reg) cr0) };
    let mut cr4 = 0usize;
    unsafe { asm!("mov {cr4}, cr4", cr4 = out(reg) cr4) };

    // Collect all relevant CPUIDs.
    let (cpuid1, cpuid7, cpuid13, cpuid8000_0007) = unsafe {
        (
            cpuid(1, 0),
            cpuid(7, 0),
            cpuid(13, 0),
            cpuid(0x8000_0007, 0),
        )
    };

    // Enable SSE.
    cr0 &= !consts::CR0_EM; // Clear EM bit.
    cr0 |= consts::CR0_MP;
    cr4 |= consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;

    print!("percpu: Enabling features:\n");

    // XSAVE
    if cpuid1.ecx & consts::CPUID_1C_XSAVE != 0 {
        cr4 |= consts::CR4_OSXSAVE | consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;
        unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4) };
        print!("percpu: + XSAVE\n");

        let mut xcr0 = 0u64;
        xcr0 |= 3;

        // AVX
        if cpuid1.ecx & consts::CPUID_1C_AVX != 0 {
            xcr0 |= 1 << 2;
            print!("percpu: + AVX\n");
        }

        // AVX-512
        if cpuid7.ebx & consts::CPUID_7B_AVX512F != 0 {
            xcr0 |= 1 << 5;
            xcr0 |= 1 << 6;
            xcr0 |= 1 << 7;
            print!("percpu: + AVX-512\n");
        }

        unsafe { asm::wrxcr(0, xcr0) };

        // Change callbacks from FXSAVE to XSAVE.
        cpu.fpu_size = cpuid13.ecx as usize;
        cpu.fpu_save = asm::xsave;
        cpu.fpu_restore = asm::xrstor;
    }

    if cpuid7.ecx & consts::CPUID_7C_UMIP != 0 {
        cr4 |= consts::CR4_UMIP;
        print!("percpu: + UMIP\n");
    }

    if cpuid7.ebx & consts::CPUID_7B_SMEP != 0 {
        cr4 |= consts::CR4_SMEP;
        print!("percpu: + SMEP\n");
    }

    if cpuid7.ebx & consts::CPUID_7B_SMAP != 0 {
        cr4 |= consts::CR4_SMAP;
        cpu.can_smap = true;
        print!("percpu: + SMAP\n");
    }

    // Check if the TSC exists and is also invariant.
    if cpuid1.edx & consts::CPUID_1D_TSC != 0 && cpuid8000_0007.edx & (1 << 8) != 0 {
        match clock::switch(Box::new(super::tsc::TscClock)) {
            Ok(x) => {
                cr4 |= consts::CR4_TSD;
                print!("percpu: + TSC\n");
            }
            Err(x) => {
                warn!("percpu: Unable to setup TSC: {:?}\n", x)
            }
        }
    }

    assert!(
        cpuid7.ebx & consts::CPUID_7B_FSGSBASE != 0,
        "FSGSBASE is required for the kernel to function, but the bit wasn't set"
    );
    cr4 |= consts::CR4_FSGSBASE;

    // Write back the modified control register values.
    unsafe { asm!("mov cr0, {cr0}", cr0 = in(reg) cr0) };
    unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4) };

    // Set FSGSBASE contents.
    unsafe {
        // Slightly misleading, but KERNEL_GS_BASE is the currently inactive GSBASE value.
        asm::wrmsr(consts::MSR_KERNEL_GS_BASE, 0);
        // We will save a reference to this struct in GS_BASE.
        asm::wrmsr(consts::MSR_GS_BASE, context.this as u64);
        asm::wrmsr(consts::MSR_FS_BASE, 0);
    }

    super::apic::LocalApic::init(context);
}

pub fn stop_all() -> ! {
    // TODO: This only halts the current CPU.
    loop {
        unsafe { asm!("cli", "hlt") };
    }
}
