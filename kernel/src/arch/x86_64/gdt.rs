use super::{
    asm,
    consts::{CPL_KERNEL, CPL_USER},
    tss::{self, TaskStateSegment, TSS_STORAGE},
};
use crate::arch::x86_64::asm::{interrupt_disable, interrupt_enable};
use core::mem::offset_of;

const GDTA_PRESENT: u8 = 1 << 7;
const GDTA_KERNEL: u8 = CPL_KERNEL << 5;
const GDTA_USER: u8 = CPL_USER << 5;
const GDTA_SEGMENT: u8 = 1 << 4;
const GDTA_EXECUTABLE: u8 = 1 << 3;
// const GDTA_DIR_CONF: u8 = 1 << 2;
const GDTA_READ_WRITE: u8 = 1 << 1;
const GDTA_ACCESSED: u8 = 1 << 0;

const GDTF_GRANULARITY: u8 = 1 << 3;
const GDTF_PROT_MODE: u8 = 1 << 2;
const GDTF_LONG_MODE: u8 = 1 << 1;

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
    const fn new(limit: u32, base: u32, access: u8, flags: u8) -> Self {
        Self {
            limit0: limit as u16,
            base0: base as u16,
            base1: (base >> 16) as u8,
            access,
            limit1_flags: ((flags << 4) & 0xF0) | (((limit >> 16) as u8) & 0x0F),
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
    const fn new(limit: u32, base: u64, access: u8, flags: u8) -> Self {
        Self {
            limit0: limit as u16,
            base0: base as u16,
            base1: (base >> 16) as u8,
            access,
            limit1_flags: ((flags << 4) & 0xF0) | (((limit >> 16) as u8) & 0x0F),
            base2: (base >> 24) as u8,
            base3: (base >> 32) as u32,
            reserved: 0,
        }
    }
}

/// Global Descriptor Table.
/// These entries are ordered exactly like this because the SYSRET instruction expects it.
#[repr(C, packed)]
pub struct GlobalDescriptorTable {
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

impl GlobalDescriptorTable {
    const fn new() -> Self {
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

pub fn load() {
    unsafe {
        // Allocate a new GDT.
        let gdt = GlobalDescriptorTable {
            null: GdtDesc::new(0, 0, 0, 0),
            kernel_code: GdtDesc::new(
                0xFFFFF,
                0,
                GDTA_PRESENT
                    | GDTA_KERNEL
                    | GDTA_SEGMENT
                    | GDTA_EXECUTABLE
                    | GDTA_READ_WRITE
                    | GDTA_ACCESSED,
                GDTF_GRANULARITY | GDTF_LONG_MODE,
            ),
            kernel_data: GdtDesc::new(
                0xFFFFF,
                0,
                GDTA_PRESENT | GDTA_KERNEL | GDTA_SEGMENT | GDTA_READ_WRITE | GDTA_ACCESSED,
                GDTF_GRANULARITY | GDTF_LONG_MODE,
            ),
            user_code: GdtDesc::new(
                0xFFFFF,
                0,
                GDTA_PRESENT | GDTA_USER | GDTA_SEGMENT | GDTA_READ_WRITE,
                GDTF_GRANULARITY | GDTF_PROT_MODE,
            ),
            user_data: GdtDesc::new(
                0xFFFFF,
                0,
                GDTA_PRESENT | GDTA_USER | GDTA_SEGMENT | GDTA_READ_WRITE,
                GDTF_GRANULARITY | GDTF_LONG_MODE,
            ),
            user_code64: GdtDesc::new(
                0xFFFFF,
                0,
                GDTA_PRESENT | GDTA_USER | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
                GDTF_GRANULARITY | GDTF_LONG_MODE,
            ),
            tss: GdtLongDesc::new(
                0xFFFFF,
                &raw const TSS_STORAGE as u64,
                GDTA_PRESENT | GDTA_KERNEL | GDTA_EXECUTABLE | GDTA_ACCESSED,
                0,
            ),
        };

        // Initialize the TSS.
        tss::init();

        // Save the GDT.
        GDT_TABLE = gdt;

        // Construct a register to hold the GDT base and limit.
        let gdtr = GdtRegister {
            limit: (size_of::<GlobalDescriptorTable>() - 1) as u16,
            base: &raw const GDT_TABLE,
        };

        // Load the table into the register.
        asm::lgdt(&gdtr);

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
            code_seg = const offset_of!(GlobalDescriptorTable, kernel_code),
            data_seg = const offset_of!(GlobalDescriptorTable, kernel_data),
            lateout("rax") _
        );
    }
}

#[repr(C, packed)]
pub struct GdtRegister {
    limit: u16,
    base: *const GlobalDescriptorTable,
}

static mut GDT_TABLE: GlobalDescriptorTable = GlobalDescriptorTable::new();
