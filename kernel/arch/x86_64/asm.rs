use super::gdt::GdtRegister;
use super::idt::IdtRegister;

/// Wrapper for the `lgdt` instruction.
/// Only changing the GDT on its own is technically unsafe.
pub unsafe fn lgdt(gdt: &GdtRegister) {
    core::arch::asm!("lgdt [{0}]", in(reg) gdt);
}

/// Wrapper for the `lidt` instruction.
/// Only changing the IDT on its own is technically unsafe.
pub unsafe fn lidt(idt: &IdtRegister) {
    core::arch::asm!("lidt [{0}]", in(reg) idt);
}
