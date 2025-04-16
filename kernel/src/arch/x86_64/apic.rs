use core::{ptr::read_volatile, u32};

use super::{
    asm::{self, interrupt_disable},
    consts,
    page::PageTableEntry,
};
use crate::{
    arch::x86_64::asm::interrupt_enable,
    boot::BootInfo,
    generic::{
        clock,
        irq::{IpiTarget, IrqController, IrqError},
        memory::PhysAddr,
        percpu::PerCpu,
    },
};

#[derive(Debug)]
pub struct LocalApic {
    has_x2apic: bool,
    lapic_addr: PhysAddr,
    // How many ticks pass in 10 milliseconds.
    ticks_per_10ms: u32,
}

impl LocalApic {
    pub fn init(cpu_info: &PerCpu) -> Self {
        let mut result = Self {
            has_x2apic: false,
            lapic_addr: PhysAddr(0),
            ticks_per_10ms: 0,
        };

        // Enable the APIC flag.
        let mut apic_msr = unsafe { asm::rdmsr(0x1B) };
        apic_msr |= 1 << 11;

        // Enable the x2APIC if we have it.
        result.has_x2apic = {
            let cpuid = unsafe { asm::cpuid(1, 0) };
            if cpuid.ecx & consts::CPUID_1C_X2APIC != 0 {
                apic_msr |= 1 << 10;
                true
            } else {
                todo!("No x2APIC available!");
                // TODO: Parse MADT for LAPIC base address.
            }
        };

        unsafe { asm::wrmsr(0x1B, apic_msr) };

        // Reset the TPR.
        result.write_register(0x80, 0);
        // Enable APIC bit in the SIVR.
        result.write_register(0xF0, result.read_register(0xF0) | 0x100);

        if !result.has_x2apic {
            result.write_register(0xE0, 0xF000_0000);
            // Logical destination = LAPIC ID.
            result.write_register(0xD0, result.read_register(0x20));
        }

        // TODO: Parse MADT and setup NMIs.

        // Tell the APIC timer to divide by 16.
        result.write_register(0x3E0, 3);
        // Set the timer counter to the highest possible value.
        result.write_register(0x380, u32::MAX);
        // Sleep for 10 milliseconds.
        clock::wait_ns(10 * 1_000_000);
        // Read how many ticks have passed in 10 ms.
        result.ticks_per_10ms = u32::MAX - result.read_register(0x390);

        unsafe { interrupt_disable() };

        // Finally, run the periodic timer interrupt on irq0.
        result.write_register(0x320, 0x20 | 0x20000);
        result.write_register(0x3E0, 3);
        result.write_register(0x380, result.ticks_per_10ms);

        unsafe { interrupt_enable() };

        print!("apic: Initialized LAPIC for CPU {}\n", cpu_info.id);
        return result;
    }

    const fn reg_to_x2apic(reg: u32) -> u32 {
        return if reg == 0x310 { 0x30 } else { (reg >> 4) } + 0x800;
    }

    fn read_register(&self, reg: u32) -> u32 {
        if self.has_x2apic {
            return unsafe { asm::rdmsr(Self::reg_to_x2apic(reg)) } as u32;
        } else {
            return unsafe {
                (PageTableEntry::get_hhdm_addr().0 as *const u32)
                    .byte_add(self.lapic_addr.0)
                    .read_volatile() as u32
            };
        }
    }

    fn write_register(&self, reg: u32, value: u32) {
        if self.has_x2apic {
            unsafe { asm::wrmsr(Self::reg_to_x2apic(reg), value as u64) };
        } else {
            unsafe {
                (PageTableEntry::get_hhdm_addr().0 as *mut u32)
                    .byte_add(self.lapic_addr.0)
                    .write_volatile(value)
            };
        }
    }
}

impl IrqController for LocalApic {
    fn id(&self) -> usize {
        return self.read_register(0x20) as usize;
    }

    fn eoi(&self) -> Result<(), IrqError> {
        self.write_register(0xB0, 0);
        return Ok(());
    }

    fn send_ipi(&self, target: IpiTarget) -> Result<(), IrqError> {
        let _ = target;
        todo!()
    }
}

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
