use super::gdt::Gdt;
use super::irq::*;
use crate::generic::memory::VirtAddr;
use core::arch::asm;
use core::mem::offset_of;
use seq_macro::seq;
use spin::Mutex;

pub const IDT_SIZE: usize = 256;

// Temporary storage to hold the limit and base of the IDT.
#[repr(C, packed)]
pub struct IdtRegister {
    limit: u16,
    base: *const Idt,
}

#[derive(Debug)]
#[repr(align(0x1000))]
pub struct Idt {
    routines: [IdtEntry; IDT_SIZE],
}

impl Idt {
    pub const fn new() -> Self {
        Self {
            routines: [IdtEntry::empty(); IDT_SIZE],
        }
    }
}

/// Loads the ISRs into the static table.
pub fn init() {
    unsafe {
        // Create a new table.
        let mut idt = IDT_TABLE.lock();

        // Set all gates to their respective handlers.
        seq!(N in 0..256 {
            idt.routines[N] = IdtEntry::new((interrupt_stub~N as usize).into(), 0, IdtIsrType::Interrupt);
        });
    }
}

/// Sets the IDT on this CPU.
pub fn set_idt() {
    let idt = IDT_TABLE.lock();
    let idtr = IdtRegister {
        limit: (size_of::<Idt>() - 1) as u16,
        base: (&*idt),
    };
    unsafe {
        asm!("lidt [{0}]", in(reg) &idtr);
    }
}

/// Global storage for the interrupt descriptor table.
static IDT_TABLE: Mutex<Idt> = Mutex::new(Idt::new());

/// Stores an interrupt service routines (ISR) handler which gets invoked during an interrupt.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct IdtEntry {
    /// The base is the address to jump to during an interrupt.
    /// Bits 0-15 of the base address.
    base0: u16,
    /// The value which `cs` should have during an interrupt.
    selector: u16,
    /// Which TaskStateSegment::ist* field to use (0-2) for interrupt stack.
    ist: u8,
    /// Type of this interrupt routine.
    attributes: u8,
    /// Bits 16-31 of the base address.
    base1: u16,
    /// Bits 32-63 of the base address.
    base2: u32,
    /// Unused
    reserved: u32,
}

#[repr(u8)]
enum IdtIsrType {
    Interrupt = 0xE,
    Trap = 0xF,
}

impl IdtEntry {
    /// Creates an empty entry. This is used to not waste binary space and make the entry be part of the .bss
    const fn empty() -> Self {
        Self {
            base0: 0,
            selector: 0,
            ist: 0,
            attributes: 0,
            base1: 0,
            base2: 0,
            reserved: 0,
        }
    }

    /// Creates a new ISR entry.
    const fn new(base: VirtAddr, interrupt_stack: u8, isr_type: IdtIsrType) -> Self {
        assert!(interrupt_stack <= 2, "`ist` must be 0, 1 or 2!");

        Self {
            base0: base.value() as u16,
            // Only allow handlers to be part of the kernel.
            selector: offset_of!(Gdt, kernel_code) as u16,
            ist: interrupt_stack,
            attributes: 1 << 7 // = Present
                | match isr_type {
                    IdtIsrType::Interrupt => 0xE,
                    IdtIsrType::Trap => 0xF,
                },
            base1: (base.value() >> 16) as u16,
            base2: (base.value() >> 32) as u32,
            reserved: 0,
        }
    }
}
