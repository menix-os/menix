use crate::{
    arch::x86_64::{
        ARCH_DATA, consts, irq,
        platform::{
            apic::{self, LocalApic},
            gdt, idt,
            tsc::{self, TscClock},
        },
    },
    generic::{
        boot::BootInfo,
        clock,
        percpu::{CpuData, LD_PERCPU_START},
    },
};
use alloc::boxed::Box;
use core::{arch::asm, mem::offset_of};

pub(in crate::arch) unsafe fn setup_bsp() {
    apic::disable_legacy_pic();
    idt::init();
    idt::set_idt();

    // Check if the FSGSBASE feature is available.
    let cpuid7 = super::asm::cpuid(7, 0);
    assert!(
        cpuid7.ebx & consts::CPUID_7B_FSGSBASE != 0,
        "FSGSBASE is required for the kernel to function, but the bit wasn't set"
    );

    // Enable the FSGSBASE bit.
    let mut cr4: usize;
    unsafe { asm!("mov {cr4}, cr4", cr4 = out(reg) cr4) };
    cr4 |= consts::CR4_FSGSBASE;
    unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4) };

    // Set FSGSBASE contents.
    unsafe {
        super::asm::wrmsr(consts::MSR_KERNEL_GS_BASE, 0);
        super::asm::wrmsr(consts::MSR_GS_BASE, &raw const LD_PERCPU_START as u64);
        super::asm::wrmsr(consts::MSR_FS_BASE, 0);
    }
}

pub(in crate::arch) fn get_per_cpu() -> *mut CpuData {
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

pub(in crate::arch) fn perpare_cpu(context: &mut CpuData) {
    let mut cr0: usize;
    let mut cr4: usize;

    let cpu = ARCH_DATA.get(context);

    // Load a GDT and TSS.
    gdt::init(&mut cpu.gdt, &mut cpu.tss);

    // Load the IDT.
    // Note: The IDT itself is global, but still needs to be loaded for each CPU.
    idt::set_idt();

    // Enable the `syscall` extension.
    unsafe {
        let msr = super::asm::rdmsr(consts::MSR_EFER);
        super::asm::wrmsr(consts::MSR_EFER, msr | consts::MSR_EFER_SCE as u64);
        // Bits 32-47 are kernel segment base, Bits 48-63 are user segment base. Lower 32 bits (EIP) are unused.
        super::asm::wrmsr(
            consts::MSR_STAR,
            ((offset_of!(gdt::Gdt, user_code) | consts::CPL_USER as usize) as u64) << 48
                | (offset_of!(gdt::Gdt, kernel_code) as u64) << 32,
        );
        // Set syscall entry point.
        super::asm::wrmsr(consts::MSR_LSTAR, irq::amd64_syscall_stub as u64);
        super::asm::wrmsr(
            consts::MSR_SFMASK,
            (consts::RFLAGS_AC | consts::RFLAGS_DF | consts::RFLAGS_IF) as u64,
        );

        asm!("mov {cr0}, cr0", cr0 = out(reg) cr0);
        asm!("mov {cr4}, cr4", cr4 = out(reg) cr4);
    }

    // Collect all relevant CPUIDs.
    let (cpuid1, cpuid7, cpuid13, cpuid8000_0007) = (
        super::asm::cpuid(1, 0),
        super::asm::cpuid(7, 0),
        super::asm::cpuid(13, 0),
        super::asm::cpuid(0x8000_0007, 0),
    );

    // Enable SSE.
    cr0 &= !consts::CR0_EM; // Clear EM bit.
    cr0 |= consts::CR0_MP;
    cr4 |= consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;

    // XSAVE
    if cpuid1.ecx & consts::CPUID_1C_XSAVE != 0 {
        cr4 |= consts::CR4_OSXSAVE | consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;
        unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4) };
        let mut xcr0 = 0u64;
        xcr0 |= 3;

        // AVX
        if cpuid1.ecx & consts::CPUID_1C_AVX != 0 {
            xcr0 |= 1 << 2;
        }

        // AVX-512
        if cpuid7.ebx & consts::CPUID_7B_AVX512F != 0 {
            xcr0 |= 1 << 5;
            xcr0 |= 1 << 6;
            xcr0 |= 1 << 7;
        }

        unsafe { super::asm::wrxcr(0, xcr0) };

        // Change callbacks from FXSAVE to XSAVE.
        cpu.fpu_size = cpuid13.ecx as usize; // ECX contains the size of the FPU block to save.
        cpu.fpu_save = super::asm::xsave;
        cpu.fpu_restore = super::asm::xrstor;
    }

    if cpuid7.ecx & consts::CPUID_7C_UMIP != 0 {
        cr4 |= consts::CR4_UMIP;
    }

    if cpuid7.ebx & consts::CPUID_7B_SMEP != 0 {
        cr4 |= consts::CR4_SMEP;
    }

    if cpuid7.ebx & consts::CPUID_7B_SMAP != 0 {
        cr4 |= consts::CR4_SMAP;
        cpu.can_smap = true;
    }

    // Check if the TSC exists and is also invariant.
    if cpuid1.edx & consts::CPUID_1D_TSC != 0 && cpuid8000_0007.edx & (1 << 8) != 0 {
        // TODO: This will break when called more than once.
        // TODO: Make sure to only switch if it's not already the current source.
        if tsc::init().is_ok() {
            cr4 |= consts::CR4_TSD;
            if BootInfo::get()
                .command_line
                .get_bool("tsc")
                .unwrap_or(false)
            {
                _ = clock::switch(Box::new(TscClock));
            }
        }
    }

    assert!(
        cpuid7.ebx & consts::CPUID_7B_FSGSBASE != 0,
        "FSGSBASE is required for the kernel to function, but the bit wasn't set"
    );
    cr4 |= consts::CR4_FSGSBASE;

    unsafe {
        // Write back the modified control register values.
        asm!("mov cr0, {cr0}", cr0 = in(reg) cr0);
        asm!("mov cr4, {cr4}", cr4 = in(reg) cr4);

        // Set FSGSBASE contents.
        // Slightly misleading, but KERNEL_GS_BASE is the currently inactive GSBASE value.
        super::asm::wrmsr(consts::MSR_KERNEL_GS_BASE, 0);
        // We will save a reference to this struct in GS_BASE.
        super::asm::wrmsr(consts::MSR_GS_BASE, context.this as u64);
        super::asm::wrmsr(consts::MSR_FS_BASE, 0);
    }

    LocalApic::init(context);
}

pub(in crate::arch) fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("cli; hlt");
        }
    }
}
