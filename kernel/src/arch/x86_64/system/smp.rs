use crate::{
    arch::x86_64::{
        ARCH_DATA,
        consts::{CR0_ET, CR0_PE, CR0_PG, CR4_PAE, MSR_EFER, MSR_EFER_LME, MSR_EFER_NXE},
        system::{
            apic::{DeliveryMode, DeliveryStatus, DestinationMode, LAPIC, Level, TriggerMode},
            gdt::{GDT, Gdt},
        },
    },
    generic::{
        clock,
        irq::IpiTarget,
        memory::{
            PhysAddr, VirtAddr,
            pmm::{AllocFlags, FreeList, PageAllocator},
            virt::{KERNEL_PAGE_TABLE, KERNEL_STACK_SIZE, VmFlags, VmLevel},
        },
        percpu::CpuData,
        util::mutex::Mutex,
    },
};
use alloc::vec::Vec;
use bytemuck::{Pod, Zeroable};
use core::arch::global_asm;
use core::mem::offset_of;
use uacpi_sys::{UACPI_STATUS_OK, acpi_entry_hdr, acpi_madt_lapic, uacpi_table};

unsafe extern "C" {
    pub unsafe static SMP_TRAMPOLINE_START: u8;
    pub unsafe static SMP_TRAMPOLINE_DATA: u8;
    pub unsafe static SMP_TRAMPOLINE_END: u8;
}

// The AP trampoline.
global_asm!("
.global SMP_TRAMPOLINE_START
.global SMP_TRAMPOLINE_ENTRY
.global SMP_TRAMPOLINE_END

.section .rodata
.code16

SMP_TRAMPOLINE_START:
    cli
    cld
    jmp 1f

SMP_TRAMPOLINE_DATA:
    .skip {info_size}

.set data_offset, (SMP_TRAMPOLINE_DATA - SMP_TRAMPOLINE_START)
.set gdtr_offset, (data_offset + {gdtr_offset})
.set farjmp_offset, (data_offset + {farjmp_offset})
.set temp_stack_offset, (data_offset + {temp_stack_offset})
.set kernel_cr3_offset, (data_offset + {kernel_cr3_offset})
.set entry_offset, (data_offset + {entry_offset})

1:
    mov bx, cs
    shl ebx, 4

.set idtr_offset, (invalid_idtr - SMP_TRAMPOLINE_START)
    lidtd cs:idtr_offset
    lgdtd cs:gdtr_offset

.set mode32_offset, (mode32 - SMP_TRAMPOLINE_START)
    lea eax, [ebx + mode32_offset]
    mov dword ptr cs:farjmp_offset, eax

    mov eax, {cr0_pe} | {cr0_et}
    mov cr0, eax
    jmp fword ptr cs:farjmp_offset

.code32
mode32:
    mov ax, {kernel32_ds}
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    xor  eax, eax
    lldt ax

    mov esp, dword ptr [ebx + temp_stack_offset]

    xor eax, eax
    mov cr4, eax

    or  eax, {cr4_pae}
    mov cr4, eax

    mov ecx, {msr_efer}
    mov eax, {efer_lme}
    xor edx, edx
    wrmsr

    mov eax, dword ptr [ebx + kernel_cr3_offset]
    mov cr3, eax

    mov eax, cr0
    or  eax, {cr0_pg}
    mov cr0, eax

.set mode64_offset, (mode64 - SMP_TRAMPOLINE_START)
    lea eax, [ebx + mode64_offset]
    push {kernel64_cs}
    push eax
    retf

.code64
mode64:
    mov ax, {kernel64_ds}
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov ecx, {msr_efer}
    rdmsr
    or eax, {efer_nxe}
    wrmsr

    xor ebp, ebp

    lea rdi, [rbx + data_offset]
    mov rax, [rbx + entry_offset]
    jmp rax

invalid_idtr:
    .word 0
    .quad 0

SMP_TRAMPOLINE_END:",
    info_size = const INFO_SIZE,

    gdtr_offset = const GDTR_OFFSET,
    farjmp_offset = const FARJMP_OFFSET,
    temp_stack_offset = const TEMP_STACK_OFFSET,
    kernel_cr3_offset = const KERNEL_CR3_OFFSET,
    entry_offset = const ENTRY_OFFSET,

    kernel32_ds = const KERNEL32_DS,
    kernel64_cs = const KERNEL64_CS,
    kernel64_ds = const KERNEL64_DS,

    cr0_pe = const CR0_PE,
    cr0_et = const CR0_ET,
    cr0_pg = const CR0_PG,
    cr4_pae = const CR4_PAE,

    msr_efer = const MSR_EFER,
    efer_lme = const MSR_EFER_LME,
    efer_nxe = const MSR_EFER_NXE,
);

const INFO_SIZE: usize = size_of::<InfoData>();
const GDTR_OFFSET: usize = offset_of!(InfoData, gdtr_limit);
const FARJMP_OFFSET: usize = offset_of!(InfoData, farjmp_offset);
const TEMP_STACK_OFFSET: usize = offset_of!(InfoData, temp_stack);
const KERNEL_CR3_OFFSET: usize = offset_of!(InfoData, kernel_cr3);
const ENTRY_OFFSET: usize = offset_of!(InfoData, entry);

const KERNEL32_DS: usize = offset_of!(Gdt, kernel32_data);
const KERNEL64_CS: usize = offset_of!(Gdt, kernel64_code);
const KERNEL64_DS: usize = offset_of!(Gdt, kernel64_data);

extern "C" fn ap_entry(info: *const InfoData) -> ! {
    let info_data = unsafe { info.read() };
    let lapic_id = unsafe { (&raw const info_data.lapic_id).read_unaligned() };

    unsafe {
        let mut stack: usize;
        core::arch::asm!("mov {stack}, rsp", stack = out(reg) stack);

        let stack_virt = PhysAddr::new(stack).as_hhdm::<u8>() as usize;
        core::arch::asm!("mov rsp, {stack}", stack = in(reg) stack_virt);

        let booted = info.byte_offset(offset_of!(InfoData, booted) as isize) as *mut u8;
        booted.write(1);
    }

    // XXX: Not safe to access `info` here anymore.

    log!("Hello from AP {lapic_id}");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[repr(C, packed)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct InfoData {
    gdt: Gdt,
    gdtr_limit: u16,
    gdtr_base: u64,
    farjmp_offset: u32,
    farjmp_segment: u32,
    temp_stack: u32,
    kernel_cr3: u64,
    entry: u64,
    lapic_id: u32,
    booted: u8,
}

fn start_ap(id: u32) {
    log!("Starting AP {id}");

    crate::arch::core::prepare_cpu(CpuData::get());

    let start = &raw const SMP_TRAMPOLINE_START; // Start of the trampoline.
    let data = &raw const SMP_TRAMPOLINE_DATA; // Start of the data passed to the trampoline.
    let end = &raw const SMP_TRAMPOLINE_END; // End of the trampoline.

    let tp_code =
        unsafe { core::slice::from_raw_parts(start, end.byte_offset_from_unsigned(start)) };

    assert!(tp_code.len() <= 0x1000);

    let Ok(mem) = FreeList::alloc_bytes(tp_code.len(), AllocFlags::Kernel20) else {
        error!("Failed to allocate 20-bit memory for the AP trampoline!");
        return;
    };

    let Ok(stack_mem) = FreeList::alloc_bytes(KERNEL_STACK_SIZE, AllocFlags::Kernel32) else {
        error!("Failed to allocate a stack for the AP trampoline!");
        return;
    };

    let temp_stack = stack_mem.value() + KERNEL_STACK_SIZE;

    // Prepare the AP trampoline.
    let buffer: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(mem.as_hhdm(), tp_code.len()) };

    buffer.copy_from_slice(tp_code);

    let data_offset = unsafe { data.offset_from_unsigned(start) };

    // Save our metadata to the trampoline.
    let info = InfoData {
        gdt: GDT,
        gdtr_limit: (size_of::<Gdt>() - 1) as u16,
        gdtr_base: (mem.value() + data_offset + offset_of!(InfoData, gdt)) as u64,
        farjmp_offset: 0,
        farjmp_segment: offset_of!(Gdt, kernel32_code) as u32,
        temp_stack: temp_stack as u32,
        kernel_cr3: KERNEL_PAGE_TABLE.lock().get_phys_addr().value() as u64,
        entry: ap_entry as *const fn() as u64,
        lapic_id: id,
        booted: 0,
    };

    buffer[data_offset..data_offset + size_of::<InfoData>()]
        .copy_from_slice(bytemuck::bytes_of(&info));

    let lapic = LAPIC.get();

    KERNEL_PAGE_TABLE
        .lock()
        .map_range::<FreeList>(
            VirtAddr::new(stack_mem.value()),
            stack_mem,
            VmFlags::Read | VmFlags::Exec,
            VmLevel::L1,
            KERNEL_STACK_SIZE,
        )
        .unwrap();

    KERNEL_PAGE_TABLE
        .lock()
        .map_single::<FreeList>(
            VirtAddr::new(mem.value()),
            mem,
            VmFlags::Read | VmFlags::Exec,
            VmLevel::L1,
        )
        .unwrap();

    lapic.send_ipi(
        IpiTarget::Specific(id),
        0,
        DeliveryMode::INIT,
        DestinationMode::Physical,
        DeliveryStatus::Idle,
        Level::Assert,
        TriggerMode::Edge,
    );
    clock::block_ns(10_000_000).unwrap();

    lapic.send_ipi(
        IpiTarget::Specific(id),
        (mem.value() >> 12) as u8,
        DeliveryMode::StartUp,
        DestinationMode::Physical,
        DeliveryStatus::Idle,
        Level::Assert,
        TriggerMode::Edge,
    );
    clock::block_ns(10_000_000).unwrap();

    let booted = unsafe {
        mem.as_hhdm::<u8>()
            .byte_add(data_offset)
            .byte_add(offset_of!(InfoData, booted))
    };

    while unsafe { booted.read_volatile() } == 0 {
        clock::block_ns(1_000_000).unwrap();
    }

    KERNEL_PAGE_TABLE
        .lock()
        .unmap_range(VirtAddr::new(stack_mem.value()), KERNEL_STACK_SIZE)
        .unwrap();

    KERNEL_PAGE_TABLE
        .lock()
        .unmap_single(VirtAddr::new(mem.value()))
        .unwrap();

    unsafe { FreeList::dealloc(mem, 1) };
}

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE, crate::system::acpi::TABLES_STAGE, crate::generic::clock::CLOCK_STAGE)]
    #[entails(crate::arch::APS_DISCOVERED_STAGE)]
    DISCOVER_STAGE: "arch.x86_64.discover-aps" => discover_aps;

    #[depends(crate::arch::APS_DISCOVERED_STAGE, crate::generic::clock::CLOCK_STAGE)]
    #[entails(crate::arch::AP_INIT_STAGE)]
    INIT_STAGE: "arch.x86_64.init-aps" => init_aps;
}

static FOUND_APS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn discover_aps() {
    // Parse the MADT to discover LAPICs.
    unsafe {
        let mut table = uacpi_table::default();
        let status = uacpi_sys::uacpi_table_find_by_signature(c"APIC".as_ptr(), &raw mut table);
        if status != UACPI_STATUS_OK {
            return;
        }

        let madt_ptr = table.__bindgen_anon_1.ptr as *const uacpi_sys::acpi_madt;
        let madt = madt_ptr.read_unaligned();

        let mut offset = 0;
        while offset < madt.hdr.length - size_of::<uacpi_sys::acpi_sdt_hdr>() as u32 {
            let ptr = madt_ptr.offset(1).byte_offset(offset as isize) as *const acpi_entry_hdr;
            let entry = ptr.read_unaligned();

            match entry.type_ as u32 {
                uacpi_sys::ACPI_MADT_ENTRY_TYPE_LAPIC => {
                    let lapic = (ptr as *const acpi_madt_lapic).read_unaligned();

                    // If this LAPIC is enabled and not the BSP, start it.
                    if lapic.flags & 1 != 0 && lapic.id as u32 != ARCH_DATA.get().lapic_id {
                        FOUND_APS.lock().push(lapic.id as u32)
                    }
                }
                _ => {}
            }
            offset += entry.length as u32;
        }

        uacpi_sys::uacpi_table_unref(&mut table);
    };
}

fn init_aps() {
    for ap in FOUND_APS.lock().iter() {
        start_ap(*ap);
    }
}
