pub const GDTA_PRESENT: u8 = 1 << 7;
pub const GDTA_CPL0: u8 = 0;
pub const GDTA_CPL3: u8 = 3 << 5;
pub const GDTA_SEGMENT: u8 = 1 << 4;
pub const GDTA_EXECUTABLE: u8 = 1 << 3;
pub const GDTA_DIR_CONF: u8 = 1 << 2;
pub const GDTA_READ_WRITE: u8 = 1 << 1;
pub const GDTA_ACCESSED: u8 = 1 << 0;
pub const GDTF_GRANULARITY: u8 = 1 << 3;
// 0 = 16-bit, 1 = 32-bit protected-mode segment
pub const GDTF_PROT_MODE: u8 = 1 << 2;
pub const GDTF_LONG_MODE: u8 = 1 << 1;

pub static GDT_TABLE: Gdt = Gdt {
    null: GdtDesc::new(0, 0, 0, 0),
    kernel_code: GdtDesc::new(
        0xFFFFF,
        0,
        GDTA_PRESENT | GDTA_CPL0 | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
        GDTF_GRANULARITY | GDTF_LONG_MODE,
    ),
    kernel_data: GdtDesc::new(
        0xFFFFF,
        0,
        GDTA_PRESENT | GDTA_CPL0 | GDTA_SEGMENT | GDTA_READ_WRITE,
        GDTF_GRANULARITY | GDTF_LONG_MODE,
    ),
    user_code: GdtDesc::new(
        0xFFFFF,
        0,
        GDTA_PRESENT | GDTA_CPL3 | GDTA_SEGMENT | GDTA_READ_WRITE,
        GDTF_GRANULARITY | GDTF_PROT_MODE,
    ),
    user_data: GdtDesc::new(
        0xFFFFF,
        0,
        GDTA_PRESENT | GDTA_CPL3 | GDTA_SEGMENT | GDTA_READ_WRITE,
        GDTF_GRANULARITY | GDTF_LONG_MODE,
    ),
    user_code64: GdtDesc::new(
        0xFFFFF,
        0,
        GDTA_PRESENT | GDTA_CPL3 | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
        GDTF_GRANULARITY | GDTF_LONG_MODE,
    ),
    tss: GdtDesc::new(
        0xFFFFF,
        0,
        GDTA_PRESENT | GDTA_CPL0 | GDTA_EXECUTABLE | GDTA_ACCESSED,
        0,
    ),
    tss_pad: GdtDesc::new(0, 0, 0, 0),
};

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
    pub const fn new(limit: u32, base: u32, access: u8, flags: u8) -> Self {
        GdtDesc {
            limit0: limit as u16,
            base0: base as u16,
            base1: (base >> 16) as u8,
            access,
            limit1_flags: flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
            base2: (base >> 24) as u8,
        }
    }
}

// These entries are ordered exactly like this because the SYSRET instruction
// expects it.
pub struct Gdt {
    null: GdtDesc,        // Unused
    kernel_code: GdtDesc, // Kernel CS
    kernel_data: GdtDesc, // Kernel DS
    user_code: GdtDesc,   // 32-bit compatibility mode user CS
    user_data: GdtDesc,   // User DS
    user_code64: GdtDesc, // 64-bit user CS
    tss: GdtDesc,         // Task state segment
    tss_pad: GdtDesc,     // TSS padding
}
