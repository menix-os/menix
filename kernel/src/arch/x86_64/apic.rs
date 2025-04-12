use super::asm;

const PIC1_COMMAND_PORT: u16 = 0x20;
const PIC1_DATA_PORT: u16 = 0x21;
const PIC2_COMMAND_PORT: u16 = 0xA0;
const PIC2_DATA_PORT: u16 = 0xA1;

/// Masks the legacy Programmable Interrupt Controller so it doesn't get in our way.
pub fn disable_legacy_pic() {
    unsafe {
        // Note: We initialize the PIC properly, but completely disable it and use the APIC in favor of it.
        // Remap IRQs so they start at 0x20 since interrupts 0x00..0x1F are used by CPU exceptions.
        asm::write8(PIC1_COMMAND_PORT, 0x11); // ICW1: Begin initialization and set cascade mode.
        asm::write8(PIC1_DATA_PORT, 0x20); // ICW2: Set where interrupts should be mapped to (0x20-0x27).
        asm::write8(PIC1_DATA_PORT, 0x04); // ICW3: Connect IRQ2 (0x04) to the slave PIC.
        asm::write8(PIC1_DATA_PORT, 0x01); // ICW4: Set the PIC to operate in 8086/88 mode.
        asm::write8(PIC1_DATA_PORT, 0xFF); // Mask all interrupts.

        // Same for the slave PIC.
        asm::write8(PIC2_COMMAND_PORT, 0x11); // ICW1: Begin initialization.
        asm::write8(PIC2_DATA_PORT, 0x28); // ICW2: Set where interrupts should be mapped to (0x28-0x2F).
        asm::write8(PIC2_DATA_PORT, 0x02); // ICW3: Connect to master PIC at IRQ2.
        asm::write8(PIC2_DATA_PORT, 0x01); // ICW4: Set the PIC to operate in 8086/88 mode.
        asm::write8(PIC2_DATA_PORT, 0xFF); // Mask all interrupts.
    }
}
