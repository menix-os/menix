use super::{asm, gdt::GlobalDescriptorTable};
use core::mem::offset_of;

const IDT_MAX_SIZE: usize = 256;
const IDT_GATE_INT: u8 = 0xE;
const IDT_GATE_TRAP: u8 = 0xF;

// Temporary storage to hold the limit and base of the IDT.
#[repr(C, packed)]
pub struct IdtRegister {
    limit: u16,
    base: *const InterruptDescriptorTable,
}

pub struct InterruptDescriptorTable {
    routines: [IdtEntry; IDT_MAX_SIZE],
}

impl InterruptDescriptorTable {
    /// Loads a global descriptor table into memory and sets it as the active one.
    pub fn load(&self) {
        unsafe {
            let idtr = IdtRegister {
                limit: (size_of::<InterruptDescriptorTable>() - 1) as u16,
                base: self,
            };

            asm::lidt(&idtr);
        }
    }
}

/// Stores an interrupt service routines (ISR) handler which gets invoked during an interrupt.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    /// The base is the address to jump to during an interrupt.
    /// Bits 0-15 of the base address.
    base0: u16,
    /// The value which `cs` should have during an interrupt.
    selector: u16,
    /// Which TaskStateSegment.ist* field to use (0-2) for interrupt stack.
    ist: u8,
    /// Type of this interrupt routine.
    isr_type: u8,
    /// Bits 16-31 of the base address.
    base1: u16,
    /// Bits 32-63 of the base address.
    base2: u32,
    /// Unused
    reserved: u32,
}

impl IdtEntry {
    const fn new(base: usize, ist: u8, isr_type: u8) -> Self {
        assert!(ist <= 2, "`ist` must be 0, 1 or 2!");
        Self {
            base0: base as u16,
            // Only allow handlers to be part of the kernel.
            selector: offset_of!(GlobalDescriptorTable, kernel_code) as u16,
            ist,
            isr_type,
            base1: (base >> 16) as u16,
            base2: (base >> 32) as u32,
            reserved: 0,
        }
    }
}

// TODO: Fill ISRs with respective handlers.
pub static IDT_TABLE: InterruptDescriptorTable = InterruptDescriptorTable {
    routines: [IdtEntry::new(0, 0, IDT_GATE_INT); 256],
};
