use super::{
    asm, consts,
    gdt::{self, Gdt},
    idt,
    tss::TaskStateSegment,
};
use crate::{arch::x86_64::asm::cpuid, generic::sched::thread::Thread};
use crate::{
    arch::x86_64::tsc::{self, TscClock},
    generic::{clock, percpu::PerCpu, sched::Scheduler},
};
use alloc::{boxed::Box, sync::Arc};
use core::mem::offset_of;
use core::{arch::asm, ffi::CStr};

#[derive(Debug)]
#[repr(align(0x10))]
pub struct ArchPerCpu {
    /// Processor local Global Descriptor Table.
    /// The GDT refers to a different TSS every time, so unlike the IDT it has to exist for each processor.
    gdt: Gdt,
    tss: TaskStateSegment,
    /// The Local APIC ID.
    lapic_id: u64,
    /// Size of the FPU.
    fpu_size: usize,
    /// Function called to save the FPU context.
    fpu_save: fn(memory: *mut u8),
    /// Function called to restore the FPU context.
    fpu_restore: fn(memory: *const u8),
    /// If this CPU supports the STAC/CLAC instructions.
    can_smap: bool,
}

impl ArchPerCpu {
    pub fn new() -> Self {
        Self {
            gdt: Gdt::new(),
            tss: TaskStateSegment::new(),
            lapic_id: 0,
            fpu_size: 512,
            fpu_save: asm::fxsave,
            fpu_restore: asm::fxrstor,
            can_smap: false,
        }
    }
}

impl PerCpu {
    /// Returns the per-CPU data of this CPU.
    ///
    /// Safety: Accessing this data directly is inherently unsafe without first disabling preemption!
    pub unsafe fn get_per_cpu() -> *mut PerCpu {
        unsafe {
            let cpu: *mut PerCpu;
            asm!(
                "mov {cpu}, gs:[0]",
                cpu = out(reg) cpu,
                options(nostack, preserves_flags),
            );
            return cpu;
        }
    }

    /// Returns a reference to the currently running scheduler.
    pub fn get_scheduler() -> *const Scheduler {
        unsafe {
            let scheduler: *const Scheduler;
            asm!(
                "mov {scheduler}, gs:[{scheduler_offset}]",
                scheduler = out(reg) scheduler,
                scheduler_offset = const offset_of!(PerCpu, scheduler),
                options(nostack, preserves_flags),
            );
            return scheduler;
        }
    }

    /// Initializes architecture dependent data for the current processor.
    pub fn arch_setup_cpu(&mut self) {
        // Print CPUID identification string.
        {
            let m = cpuid(0, 0);
            let manufacturer = [m.ebx, m.edx, m.ecx, 0];
            let (n0, n1, n2) = (
                cpuid(0x8000_0002, 0),
                cpuid(0x8000_0003, 0),
                cpuid(0x8000_0004, 0),
            );
            let cpu_name = [
                n0.eax, n0.ebx, n0.ecx, n0.edx, n1.eax, n1.ebx, n1.ecx, n1.edx, n2.eax, n2.ebx,
                n2.ecx, n2.edx, 0,
            ];
            unsafe {
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
        }

        // Load a GDT and TSS.
        gdt::init(&mut self.arch.gdt, &mut self.arch.tss);

        // Load the IDT.
        // Note: The IDT itself is global, but still needs to be loaded for each CPU.
        idt::set_idt();

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

        // Collect all relevant CPUIDs.
        let cpuid1 = cpuid(1, 0);
        let cpuid7 = cpuid(7, 0);
        let cpuid13 = cpuid(13, 0);

        // Enable SSE.
        cr0 &= !consts::CR0_EM; // Clear EM bit.
        cr0 |= consts::CR0_MP;
        cr4 |= consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;

        print!("percpu: Enabling features:\n");

        // XSAVE
        if cpuid1.ecx & consts::CPUID_1C_XSAVE as u32 != 0 {
            cr4 |= consts::CR4_OSXSAVE | consts::CR4_OSFXSR | consts::CR4_OSXMMEXCPT;
            unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4) };
            print!("percpu: + XSAVE\n");

            let mut xcr0 = 0u64;
            xcr0 |= 3;

            // AVX
            if cpuid1.ecx & consts::CPUID_1C_AVX as u32 != 0 {
                xcr0 |= 1 << 2;
                print!("percpu: + AVX\n");
            }

            // AVX-512
            if cpuid7.ebx & consts::CPUID_7B_AVX512F as u32 != 0 {
                xcr0 |= 1 << 5;
                xcr0 |= 1 << 6;
                xcr0 |= 1 << 7;
                print!("percpu: + AVX-512\n");
            }

            asm::wrxcr(0, xcr0);

            // Change callbacks from FXSAVE to XSAVE.
            self.arch.fpu_size = cpuid13.ecx as usize;
            self.arch.fpu_save = asm::xsave;
            self.arch.fpu_restore = asm::xrstor;
        }

        if cpuid7.ecx & consts::CPUID_7C_UMIP as u32 != 0 {
            cr4 |= consts::CR4_UMIP;
            print!("percpu: + UMIP\n");
        }

        if cpuid7.ebx & consts::CPUID_7B_SMEP as u32 != 0 {
            cr4 |= consts::CR4_SMEP;
            print!("percpu: + SMEP\n");
        }

        if cpuid7.ebx & consts::CPUID_7B_SMAP as u32 != 0 {
            cr4 |= consts::CR4_SMAP;
            self.arch.can_smap = true;
            print!("percpu: + SMAP\n");
        }

        if cpuid1.edx & consts::CPUID_1D_TSC as u32 != 0 && tsc::setup() {
            clock::switch(Box::new(super::tsc::TscClock));
            cr4 |= consts::CR4_TSD;
            print!("percpu: + RDTSC\n");
        }

        // Write back the modified control register values.
        unsafe { asm!("mov cr0, {cr0}", cr0 = in(reg) cr0) };
        unsafe { asm!("mov cr4, {cr4}", cr4 = in(reg) cr4) };

        // FSGSBASE is NOT optional for us.
        assert!(
            cpuid7.ebx & consts::CPUID_7B_FSGSBASE as u32 != 0,
            "FSGSBASE is required for the kernel to function, but the bit wasn't set"
        );
        cr4 |= consts::CR4_FSGSBASE;

        // Set FSGSBASE contents.
        // Slightly misleading, but KERNEL_GS_BASE is the currently inactive GSBASE value.
        asm::wrmsr(consts::MSR_KERNEL_GS_BASE, 0);
        // We will save a reference to this struct in GS_BASE.
        asm::wrmsr(consts::MSR_GS_BASE, self as *mut PerCpu as u64);
        asm::wrmsr(consts::MSR_FS_BASE, 0);

        // TODO: Mask the legacy PIC.
        // TODO: Setup LAPIC.
    }
}
