use super::gdt::Gdt;
use crate::{
    arch::{
        sched::Context,
        x86_64::{
            ARCH_DATA,
            consts::{self},
            system::apic::LAPIC,
        },
    },
    generic::{
        self,
        irq::IrqHandlerKind,
        memory::{VirtAddr, virt::fault::PageFaultInfo},
        percpu::CpuData,
    },
};
use core::{
    arch::{asm, naked_asm},
    mem::offset_of,
};
use seq_macro::seq;

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
    // Create a new table.
    let idt = &raw mut IDT_TABLE;

    // Set all gates to their respective handlers.
    unsafe {
        seq!(N in 0..256 {
            (*idt).routines[N] = IdtEntry::new((interrupt_stub~N as usize).into(), 0, GateType::Interrupt);
        });
    }
}

/// Sets the IDT on this CPU.
pub fn set_idt() {
    let idtr = IdtRegister {
        limit: (size_of::<Idt>() - 1) as u16,
        base: &raw const IDT_TABLE,
    };
    unsafe {
        asm!("lidt [{0}]", in(reg) &idtr);
    }
}

/// Global storage for the interrupt descriptor table.
static mut IDT_TABLE: Idt = Idt::new();

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
#[allow(unused)]
enum GateType {
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
    const fn new(base: VirtAddr, interrupt_stack: u8, gate: GateType) -> Self {
        assert!(interrupt_stack <= 2, "`ist` must be 0, 1 or 2!");

        Self {
            base0: base.value() as u16,
            // Only allow handlers to be part of the kernel.
            selector: offset_of!(Gdt, kernel64_code) as u16,
            ist: interrupt_stack,
            attributes: (1 << 7) // = Present
                | (gate as u8 & 0xF),
            base1: (base.value() >> 16) as u16,
            base2: (base.value() >> 32) as u32,
            reserved: 0,
        }
    }
}

/// Invoked by an interrupt stub.
unsafe extern "C" fn idt_handler(context: *const Context) {
    let context = unsafe { context.as_ref().unwrap() };
    let isr = context.isr;

    match isr as u8 {
        // Exceptions.
        consts::IDT_PF => {
            page_fault_handler(context);
        }
        // Unhandled exceptions.
        0x00..0x20 => {
            error!("{:?}", context);
            panic!("Got an exception {} on CPU {}", isr, CpuData::get().id);
        }
        // IPIs
        consts::IDT_RESCHED => {
            unsafe { crate::arch::sched::preempt_disable() };
            let cpu = CpuData::get();
            if unsafe { crate::arch::sched::preempt_enable() } {
                LAPIC.get().eoi();
                cpu.scheduler.reschedule();
            }
        }
        // Any other ISR is an IRQ with a dynamic handler.
        _ => {
            match &ARCH_DATA.get().irq_handlers.lock()[isr as usize] {
                IrqHandlerKind::Static(x) => x.handle_immediate(),
                IrqHandlerKind::Dynamic(x) => x.handle_immediate(),
                IrqHandlerKind::None => {
                    panic!("Got an unhandled interrupt {:#x}!", isr);
                }
            };
        }
    };
}

// /// Try to send a signal to the user-space program or panic if the interrupt is caused by the kernel.
// fn try_signal_or_die(context: &Context, signal: u32, code: u32) {}

fn page_fault_handler(context: &Context) {
    let mut cr2: usize;
    unsafe { asm!("mov {cr2}, cr2", cr2 = out(reg) cr2) };

    let err = context.error;
    let info = PageFaultInfo {
        page_was_present: err & (1 << 0) != 0,
        caused_by_write: err & (1 << 1) != 0,
        caused_by_fetch: err & (1 << 4) != 0,
        caused_by_user: context.cs & consts::CPL_USER as u64 == consts::CPL_USER as u64,
        ip: (context.rip as usize).into(),
        addr: cr2.into(),
    };

    generic::memory::virt::fault::handler(&info);
}

// There are some interrupts which generate an error code on the stack, while others do not.
// We normalize this by just pushing 0 for those that don't generate an error code.
seq! { N in 0..256 {
    #[unsafe(naked)]
    unsafe extern "C" fn interrupt_stub~N() {
        naked_asm!(
            // These codes push an error on the stack. Do nothing.
            ".if ({i} == 8 || ({i} >= 10 && {i} <= 14) || {i} == 17 || {i} == 21 || {i} == 29 || {i} == 30)",
            // All other ones don't, so we need to push something ourselves.
            ".else",
            "push 0",
            ".endif",

            "push {i}",
            "jmp {interrupt_stub_internal}",

            i = const N,
            interrupt_stub_internal = sym interrupt_stub_internal
        );
    }
}}

const CS_OFFSET: usize = size_of::<Context>() - size_of::<u64>() - offset_of!(Context, cs);

/// To avoid having 256 big functions with essentially the same logic,
/// this function is meant to do the actual heavy lifting.
#[unsafe(naked)]
unsafe extern "C" fn interrupt_stub_internal() {
    naked_asm!(
        // Load the kernel GS base if we're coming from user space.
        "cmp word ptr [rsp+{cs}], {kernel_cs}",
        "je 2f",
        "swapgs",
        "2:",
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rbp",
        "push rdi",
        "push rsi",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "cld",
        // Zero out the base pointer since we can't trust it.
        //"xor rbp, rbp",
        // Load the frame as first argument.
        "mov rdi, rsp",
        "call {interrupt_handler}",
        "jmp {interrupt_return}",
        cs = const CS_OFFSET,
        kernel_cs = const offset_of!(Gdt, kernel64_code),
        interrupt_handler = sym idt_handler,
        interrupt_return = sym interrupt_return
    );
}

/// Returns from an interrupt frame.
#[unsafe(naked)]
pub unsafe extern "C" fn interrupt_return() {
    naked_asm!(
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rsi",
        "pop rdi",
        "pop rbp",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        // Change GS back if we came from user mode.
        "cmp word ptr [rsp+{cs}], {kernel_cs}",
        "je 2f",
        "swapgs",
        "2:",
        // Skip .error and .isr fields.
        "add rsp, 0x10",
        "iretq",
        cs = const CS_OFFSET,
        kernel_cs = const offset_of!(Gdt, kernel64_code),
    );
}
