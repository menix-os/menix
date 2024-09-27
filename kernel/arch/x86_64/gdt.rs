use super::asm;

const GDTA_PRESENT: u8 = 1 << 7;
const GDTA_CPL0: u8 = 0;
const GDTA_CPL3: u8 = 3 << 5;
const GDTA_SEGMENT: u8 = 1 << 4;
const GDTA_EXECUTABLE: u8 = 1 << 3;
const GDTA_DIR_CONF: u8 = 1 << 2;
const GDTA_READ_WRITE: u8 = 1 << 1;
const GDTA_ACCESSED: u8 = 1 << 0;

const GDTF_GRANULARITY: u8 = 1 << 3;
const GDTF_PROT_MODE: u8 = 1 << 2;
const GDTF_LONG_MODE: u8 = 1 << 1;

pub static GDT_TABLE: GlobalDescriptorTable = GlobalDescriptorTable {
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
    /// Encode a new GDT descriptor
    const fn new(limit: u32, base: u32, access: u8, flags: u8) -> Self {
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
    pub tss: GdtDesc,
    /// TSS is a long GdtDesc, but since these values are 0, just use another GdtDesc.
    pub tss_pad: GdtDesc,
}

impl GlobalDescriptorTable {
    /// Loads a global descriptor table into memory and sets it as the active one.
    pub fn load(&self) {
        unsafe {
            let gdtr = GdtRegister {
                limit: (size_of::<GlobalDescriptorTable>() - 1) as u16,
                base: self,
            };
            asm::lgdt(&gdtr);
        }
    }
}

pub struct GdtRegister {
    limit: u16,
    base: *const GlobalDescriptorTable,
}
