use super::asm::{interrupt_disable, interrupt_enable};
use super::consts::{CPL_KERNEL, CPL_USER};
use super::tss::{self, TaskStateSegment};
use crate::arch::{VirtAddr, x86_64::asm};
use bitflags::bitflags;
use core::arch::asm;
use core::mem::offset_of;

/// Global Descriptor Table.
/// These entries are ordered exactly like this because the SYSRET instruction expects it.
#[repr(C, packed)]
#[derive(Debug)]
pub struct Gdt {
    /// Unused
    pub null: GdtDesc,
    /// Kernel CS
    pub kernel_code: GdtDesc,
    /// Kernel DS
    pub kernel_data: GdtDesc,
    /// 32-bit compatibility mode user CS (unused)
    pub user_code: GdtDesc,
    /// User DS
    pub user_data: GdtDesc,
    /// 64-bit user CS
    pub user_code64: GdtDesc,
    /// Task state segment
    pub tss: GdtLongDesc,
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            null: GdtDesc::empty(),
            kernel_code: GdtDesc::empty(),
            kernel_data: GdtDesc::empty(),
            user_code: GdtDesc::empty(),
            user_data: GdtDesc::empty(),
            user_code64: GdtDesc::empty(),
            tss: GdtLongDesc::empty(),
        }
    }
}

bitflags! {
    struct GdtAccess: u8 {
        const None = 0;
        const Present = 1 << 7;
        const Kernel = CPL_KERNEL << 5;
        const User = CPL_USER << 5;
        const Segment = 1 << 4;
        const Executable = 1 << 3;
        const ReadWrite = 1 << 1;
        const Accessed = 1 << 0;
    }

    struct GdtFlags: u8 {
        const None = 0;
        const Granularity = 1 << 3;
        const ProtMode = 1 << 2;
        const LongMode = 1 << 1;
    }
}

/// GDT segment descriptor
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GdtDesc {
    /// Limit[0..15]
    limit0: u16,
    /// Base[0..15]
    base0: u16,
    /// Base[16..23]
    base1: u8,
    /// Access modifider
    access: u8,
    /// Limit[16..19] and Flags
    limit1_flags: u8,
    /// Base[24..31]
    base2: u8,
}

impl GdtDesc {
    const fn empty() -> Self {
        Self {
            limit0: 0,
            base0: 0,
            base1: 0,
            access: 0,
            limit1_flags: 0,
            base2: 0,
        }
    }

    /// Encode a new GDT descriptor
    const fn new(limit: u32, base: u32, access: GdtAccess, flags: GdtFlags) -> Self {
        Self {
            limit0: limit as u16,
            base0: base as u16,
            base1: (base >> 16) as u8,
            access: access.bits(),
            limit1_flags: ((flags.bits() << 4) & 0xF0) | (((limit >> 16) as u8) & 0x0F),
            base2: (base >> 24) as u8,
        }
    }
}

/// GDT segment descriptor
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GdtLongDesc {
    /// Limit[0..15]
    limit0: u16,
    /// Base[0..15]
    base0: u16,
    /// Base[16..23]
    base1: u8,
    /// Access modifider
    access: u8,
    /// Limit[16..19] and Flags
    limit1_flags: u8,
    /// Base[24..31]
    base2: u8,
    /// Base[32..63]
    base3: u32,
    /// Reserved
    reserved: u32,
}

impl GdtLongDesc {
    const fn empty() -> Self {
        Self {
            limit0: 0,
            base0: 0,
            base1: 0,
            access: 0,
            limit1_flags: 0,
            base2: 0,
            base3: 0,
            reserved: 0,
        }
    }

    /// Encode a new GDT descriptor
    const fn new(limit: u32, base: u64, access: GdtAccess, flags: GdtFlags) -> Self {
        Self {
            limit0: limit as u16,
            base0: base as u16,
            base1: (base >> 16) as u8,
            access: access.bits(),
            limit1_flags: ((flags.bits() << 4) & 0xF0) | (((limit >> 16) as u8) & 0x0F),
            base2: (base >> 24) as u8,
            base3: (base >> 32) as u32,
            reserved: 0,
        }
    }
}

#[repr(C, packed)]
pub struct GdtRegister {
    limit: u16,
    base: *const Gdt,
}

pub fn init(gdt: &mut Gdt, tss: &mut TaskStateSegment) {
    // Might be overkill. Who cares?
    *gdt = Gdt {
        null: GdtDesc::new(0, 0, GdtAccess::None, GdtFlags::None),
        kernel_code: GdtDesc::new(
            0xFFFFF,
            0,
            GdtAccess::Present
                | GdtAccess::Kernel
                | GdtAccess::Segment
                | GdtAccess::Executable
                | GdtAccess::ReadWrite
                | GdtAccess::Accessed,
            GdtFlags::Granularity | GdtFlags::LongMode,
        ),
        kernel_data: GdtDesc::new(
            0xFFFFF,
            0,
            GdtAccess::Present
                | GdtAccess::Kernel
                | GdtAccess::Segment
                | GdtAccess::ReadWrite
                | GdtAccess::Accessed,
            GdtFlags::Granularity | GdtFlags::LongMode,
        ),
        user_code: GdtDesc::new(
            0xFFFFF,
            0,
            GdtAccess::Present | GdtAccess::User | GdtAccess::Segment | GdtAccess::ReadWrite,
            GdtFlags::Granularity | GdtFlags::ProtMode,
        ),
        user_data: GdtDesc::new(
            0xFFFFF,
            0,
            GdtAccess::Present | GdtAccess::User | GdtAccess::Segment | GdtAccess::ReadWrite,
            GdtFlags::Granularity | GdtFlags::LongMode,
        ),
        user_code64: GdtDesc::new(
            0xFFFFF,
            0,
            GdtAccess::Present
                | GdtAccess::User
                | GdtAccess::Segment
                | GdtAccess::Executable
                | GdtAccess::ReadWrite,
            GdtFlags::Granularity | GdtFlags::LongMode,
        ),
        tss: GdtLongDesc::new(
            0xFFFFF,
            &raw const tss as u64,
            GdtAccess::Present | GdtAccess::Kernel | GdtAccess::Executable | GdtAccess::Accessed,
            GdtFlags::None,
        ),
    };

    // Initialize the TSS.
    tss::init(tss);

    // Construct a register to hold the GDT base and limit.
    let gdtr = GdtRegister {
        limit: (size_of::<Gdt>() - 1) as u16,
        base: gdt,
    };

    unsafe {
        // Load the table into the register.
        asm!("lgdt [{0}]", in(reg) &gdtr);

        // Flush and reload the segment registers.
        asm!(
            "push {code_seg}",
            "lea rax, [rip + 2f]",
            "push rax",
            "retfq",
            "2:",
            "mov ax, {data_seg}",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            code_seg = const offset_of!(Gdt, kernel_code),
            data_seg = const offset_of!(Gdt, kernel_data),
            lateout("rax") _ // (R)AX was modified.
        );
    }
}
