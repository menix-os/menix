#![allow(unused)]

use crate::{
    arch::{
        self,
        x86_64::{
            asm,
            consts::{self, IDT_RESCHED},
        },
    },
    generic::{
        clock,
        irq::{IrqHandler, IrqStatus},
        memory::{
            PhysAddr,
            mmio::{Mmio, Register},
        },
        percpu::CpuData,
        util::spin_mutex::SpinMutex,
    },
};
use core::{
    hint::unlikely,
    sync::atomic::{AtomicU32, Ordering},
    u32,
};

#[derive(Debug)]
pub struct LocalApic {
    /// How many ticks pass in 10 milliseconds.
    ticks_per_10ms: AtomicU32,
    /// If [`Some`], points to the xAPIC MMIO space.
    /// Otherwise, it's an x2APIC.
    xapic_regs: SpinMutex<Option<Mmio>>,
}

per_cpu! {
    pub static LAPIC: LocalApic = LocalApic { ticks_per_10ms: AtomicU32::new(0), xapic_regs: SpinMutex::new(None) };
}

mod regs {
    use crate::generic::memory::mmio::Register;

    pub const ID: Register<u32> = Register::new(0x20);
    pub const TPR: Register<u32> = Register::new(0x80);
    pub const EOI: Register<u32> = Register::new(0xB0);
    pub const LDR: Register<u32> = Register::new(0xD0);
    pub const DFR: Register<u32> = Register::new(0xE0);
    pub const SIVR: Register<u32> = Register::new(0xF0);
    pub const ESR: Register<u32> = Register::new(0x280);
    pub const ICR: Register<u32> = Register::new(0x300);
    pub const ICR_HI: Register<u32> = Register::new(0x310);
    pub const LVT_TR: Register<u32> = Register::new(0x320);
    pub const ICR_TIMER: Register<u32> = Register::new(0x380);
    pub const CCR: Register<u32> = Register::new(0x390);
    pub const DCR: Register<u32> = Register::new(0x3E0);
}

#[repr(u8)]
pub enum DeliveryMode {
    Fixed = 0b000,
    LowestPrio = 0b001,
    SMI = 0b010,
    NMI = 0b100,
    INIT = 0b101,
    StartUp = 0b110,
}

#[repr(u8)]
pub enum DestinationMode {
    Physical = 0,
    Logical = 1,
}

#[repr(u8)]
pub enum DeliveryStatus {
    Idle = 0,
    Pending = 1,
}

#[repr(u8)]
pub enum Level {
    Deassert = 0,
    Assert = 1,
}

#[repr(u8)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

pub enum IpiTarget {
    /// Send an interrupt to the calling CPU.
    ThisCpu,
    /// Send an interrupt to all CPUs.
    All,
    /// Send an interrupt to all CPUs except the calling CPU.
    AllButThisCpu,
    /// Send an interrupt to a specific CPU. The value is the ID of the target [`IrqController`].
    Specific(u32),
}

impl LocalApic {
    pub fn init() {
        let lapic = LAPIC.get();

        // Enable the APIC flag.
        let mut apic_msr = unsafe { asm::rdmsr(0x1B) };
        apic_msr |= 1 << 11;

        // Enable the x2APIC if we have it.
        *lapic.xapic_regs.lock() = {
            let cpuid = asm::cpuid(1, 0);
            if cpuid.ecx & consts::CPUID_1C_X2APIC != 0 {
                apic_msr |= 1 << 10;
                None
            } else {
                Some(unsafe { Mmio::new_mmio(PhysAddr::from(apic_msr & 0xFFFFF000), 0x1000) })
            }
        };

        unsafe { asm::wrmsr(0x1B, apic_msr) };

        // Reset the TPR.
        lapic.write_reg(regs::TPR, 0);
        // Enable APIC bit in the SIVR.
        lapic.write_reg(regs::SIVR, lapic.read_reg(regs::SIVR) | 0x100);

        if lapic.xapic_regs.lock().is_some() {
            lapic.write_reg(regs::DFR, 0xF000_0000);
            // Logical destination = LAPIC ID.
            lapic.write_reg(regs::LDR, lapic.read_reg(regs::ID));
        }

        // TODO: Parse MADT and setup NMIs.

        // Tell the APIC timer to divide by 16.
        lapic.write_reg(regs::DCR, 3);
        // Set the timer counter to the highest possible value.
        lapic.write_reg(regs::ICR_TIMER, u32::MAX as u64);

        // Sleep for 10 milliseconds.
        clock::block_ns(10_000_000)
            .expect("Unable to setup LAPIC, the kernel should have a working timer!");

        // Read how many ticks have passed in 10 ms.
        lapic.ticks_per_10ms.store(
            u32::MAX - lapic.read_reg(regs::CCR) as u32,
            Ordering::Relaxed,
        );

        // Finally, run the periodic timer interrupt.
        lapic.write_reg(regs::LVT_TR, IDT_RESCHED as u64 | 0x20000);
        lapic.write_reg(regs::DCR, 3);
        lapic.write_reg(
            regs::ICR_TIMER,
            lapic.ticks_per_10ms.load(Ordering::Relaxed) as u64,
        );
    }

    fn read_reg(&self, reg: Register<u32>) -> u64 {
        match &*self.xapic_regs.lock() {
            Some(x) => {
                if reg == regs::ICR {
                    x.read(regs::ICR) as u64 | (x.read(regs::ICR_HI) as u64) << 32
                } else {
                    x.read(reg).into()
                }
            }
            None => unsafe { asm::rdmsr(0x800 + (reg.offset() as u32 >> 4)) },
        }
    }

    fn write_reg(&self, reg: Register<u32>, value: u64) {
        match &*self.xapic_regs.lock() {
            Some(x) => {
                if reg == regs::ICR {
                    x.write(regs::ICR_HI, (value >> 32) as u32);
                    x.write(regs::ICR, value as u32);
                } else {
                    x.write(reg, value as u32)
                }
            }
            None => unsafe { asm::wrmsr(0x800 + (reg.offset() as u32 >> 4), value) },
        }
    }

    pub fn id(&self) -> u32 {
        return self.read_reg(regs::ID) as u32;
    }

    /// Signals an end of interrupt to the LAPIC.
    pub fn eoi(&self) {
        self.write_reg(regs::EOI, 0);
    }

    /// Sends an inter-processor interrupt.
    pub fn send_ipi(
        &self,
        target: IpiTarget,
        vector: u8,
        delivery_mode: DeliveryMode,
        destination_mode: DestinationMode,
        delivery_status: DeliveryStatus,
        level: Level,
        trigger_mode: TriggerMode,
    ) {
        let mut icr = vector as u64;
        icr |= (delivery_mode as u64) << 8;
        icr |= (destination_mode as u64) << 10;
        icr |= (delivery_status as u64) << 11;
        icr |= (level as u64) << 14;
        icr |= (trigger_mode as u64) << 15;
        icr |= (match target {
            IpiTarget::ThisCpu => 0b01,
            IpiTarget::All => 0b10,
            IpiTarget::AllButThisCpu => 0b11,
            IpiTarget::Specific(x) => {
                if self.xapic_regs.lock().is_some() {
                    icr |= (x as u8 as u64) << 56;
                } else {
                    icr |= (x as u64) << 32;
                }
                0b00
            }
        } as u64)
            << 18;

        self.write_reg(regs::ICR, icr);
    }
}

impl IrqHandler for LocalApic {
    // TODO
    fn handle_immediate(&self) -> IrqStatus {
        unsafe { arch::sched::preempt_disable() };
        self.eoi();

        if unlikely(unsafe { crate::arch::sched::preempt_enable() }) {
            CpuData::get().scheduler.reschedule();
        }

        return IrqStatus::Handled;
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
