use crate::{
    arch::x86_64::{
        ARCH_DATA, consts, irq,
        system::{
            apic::{self, LocalApic},
            gdt, idt,
        },
    },
    generic::percpu::{CpuData, LD_PERCPU_START},
};
use core::{arch::asm, mem::offset_of, ptr::null_mut, sync::atomic::Ordering};

fn early_init() {
    apic::disable_legacy_pic();
    idt::init();
    idt::set_idt();

    // Set FSGSBASE contents.
    unsafe {
        super::asm::wrmsr(consts::MSR_KERNEL_GS_BASE, 0);
        super::asm::wrmsr(consts::MSR_GS_BASE, &raw const LD_PERCPU_START as u64);
        super::asm::wrmsr(consts::MSR_FS_BASE, 0);
    }

    CpuData::get().present.store(true, Ordering::Relaxed);
}

init_stage! {
    #[entails(crate::arch::EARLY_INIT_STAGE)]
    EARLY_INIT_STAGE: "arch.x86_64.early-init" => early_init;
}

pub(in crate::arch) fn get_frame_pointer() -> usize {
    let mut fp: usize;
    unsafe {
        asm!("mov {fp}, rbp", fp = out(reg) fp, options(nostack));
    }
    return fp;
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
        assert_ne!(cpu, null_mut());
        return cpu;
    }
}

pub(super) fn setup_core(context: &'static CpuData) {
    let mut cr0: usize;
    let mut cr4: usize;

    unsafe {
        // Set FSGSBASE contents.
        // Slightly misleading, but KERNEL_GS_BASE is the currently inactive GSBASE value.
        super::asm::wrmsr(consts::MSR_KERNEL_GS_BASE, 0);
        // We will save a reference to this struct in GS_BASE.
        super::asm::wrmsr(consts::MSR_GS_BASE, context.this as u64);
        super::asm::wrmsr(consts::MSR_FS_BASE, 0);
    }

    let cpu = ARCH_DATA.get();

    // Load a GDT and TSS.
    gdt::init(cpu);

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
                | (offset_of!(gdt::Gdt, kernel64_code) as u64) << 32,
        );
        // Set syscall entry point.
        super::asm::wrmsr(consts::MSR_LSTAR, irq::amd64_syscall_stub as u64);
        super::asm::wrmsr(
            consts::MSR_SFMASK,
            (consts::RFLAGS_AC | consts::RFLAGS_DF | consts::RFLAGS_IF) as u64,
        );

        asm!("mov {cr0}, cr0", cr0 = out(reg) cr0, options(nostack));
        asm!("mov {cr4}, cr4", cr4 = out(reg) cr4, options(nostack));
    }

    // Collect all relevant CPUIDs.
    let (cpuid1, cpuid7, cpuid13) = (
        super::asm::cpuid(1, 0),
        super::asm::cpuid(7, 0),
        super::asm::cpuid(13, 0),
    );

    // Enable SSE.
    cr0 &= !consts::CR0_EM; // Clear EM bit.
    cr0 |= consts::CR0_MP;
    cr4 |= consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;

    // XSAVE
    if cpuid1.ecx & consts::CPUID_1C_XSAVE != 0 {
        cr4 |= consts::CR4_OSXSAVE | consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;
        unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4, options(nostack)) };
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
        cpu.fpu_size.store(cpuid13.ecx as usize, Ordering::Relaxed); // ECX contains the size of the FPU block to save.
        cpu.fpu_save
            .store(super::asm::xsave as _, Ordering::Relaxed);
        cpu.fpu_restore
            .store(super::asm::xrstor as _, Ordering::Relaxed);
    }

    if cpuid7.ecx & consts::CPUID_7C_UMIP != 0 {
        cr4 |= consts::CR4_UMIP;
    }

    if cpuid7.ebx & consts::CPUID_7B_SMEP != 0 {
        cr4 |= consts::CR4_SMEP;
    }

    if cpuid7.ebx & consts::CPUID_7B_SMAP != 0 {
        cr4 |= consts::CR4_SMAP;
        cpu.can_smap.store(true, Ordering::Relaxed);
    }

    if cpuid7.ebx & consts::CPUID_7B_FSGSBASE != 0 {
        cr4 |= consts::CR4_FSGSBASE;
    }

    unsafe {
        // Write back the modified control register values.
        asm!("mov cr0, {cr0}", cr0 = in(reg) cr0, options(nostack));
        asm!("mov cr4, {cr4}", cr4 = in(reg) cr4, options(nostack));
    }

    LocalApic::init();

    context.present.store(true, Ordering::Release);
    context.online.store(true, Ordering::Release);
}

pub(in crate::arch) fn halt() -> ! {
    // TODO: Send panic IPI to actually store all CPUs.
    loop {
        unsafe {
            core::arch::asm!("cli; hlt");
        }
    }
}
