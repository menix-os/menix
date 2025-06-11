use crate::{
    arch::x86_64::{
        ARCH_DATA,
        system::{
            apic::{DeliveryMode, DeliveryStatus, DestinationMode, LAPIC, Level, TriggerMode},
            gdt::{GDT, Gdt, GdtAccess, GdtDesc, GdtFlags, GdtLongDesc, GdtRegister},
        },
    },
    generic::{
        clock,
        irq::IpiTarget,
        memory::pmm::{AllocFlags, FreeList, PageAllocator},
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

1:
    mov bx, cs
    shl ebx, 4

.set idtr_offset, (invalid_idtr - SMP_TRAMPOLINE_START)
    lidtd cs:idtr_offset
.set gdtr_offset, (data_offset + {gdtr_offset})
    lgdtd cs:gdtr_offset

.set mode32_offset, (mode32 - SMP_TRAMPOLINE_START)
    lea eax, [ebx + mode32_offset]

.set farjmp_offset, (data_offset + {farjmp_offset})
    mov dword ptr cs:farjmp_offset, eax

    mov eax, 0x00000011
    mov cr0, eax
    jmp fword ptr cs:farjmp_offset

.code32
mode32:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    xor eax, eax

    xor eax, eax
    mov cr4, eax

3:
    hlt
    jmp 3b

invalid_idtr:
    .word 0
    .quad 0

SMP_TRAMPOLINE_END:",
    info_size = const INFO_SIZE,
    gdtr_offset = const GDTR_OFFSET,
    farjmp_offset = const FARJMP_OFFSET,
);

const INFO_SIZE: usize = size_of::<InfoData>();
const GDTR_OFFSET: usize = offset_of!(InfoData, gdtr_limit);
const FARJMP_OFFSET: usize = offset_of!(InfoData, farjmp_offset);

#[repr(C, packed)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct InfoData {
    gdtr_limit: u16,
    gdtr_base: u32,
    gdt: [GdtDesc; 3],
    farjmp_offset: u32,
    farjmp_segment: u32,
    lapic_id: u32,
}

fn start_ap(id: u32) {
    log!("Starting AP {id}");
    crate::arch::core::prepare_cpu(CpuData::get());

    let start = &raw const SMP_TRAMPOLINE_START; // Start of the trampoline.
    let data = &raw const SMP_TRAMPOLINE_DATA; // Start of the data passed to the trampoline.
    let end = &raw const SMP_TRAMPOLINE_END; // End of the trampoline.

    let tp_code =
        unsafe { core::slice::from_raw_parts(start, end.byte_offset_from_unsigned(start)) };

    let Ok(mem) = FreeList::alloc_bytes(tp_code.len(), AllocFlags::Kernel20) else {
        error!("Failed to allocate 20-bit memory for the AP trampoline!");
        return;
    };

    // Prepare the AP trampoline.
    let buffer: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(mem.as_hhdm(), tp_code.len()) };

    buffer.copy_from_slice(tp_code);

    let data_offset = unsafe { data.offset_from_unsigned(start) };

    // Save our metadata to the trampoline.
    let info = InfoData {
        lapic_id: id,
        gdtr_limit: ((size_of::<GdtDesc>() * 3) - 1) as u16,
        gdtr_base: mem.value() as u32 + data_offset as u32 + offset_of!(InfoData, gdt) as u32,
        gdt: [
            GdtDesc::empty(),
            GdtDesc::new(
                0xFFFF,
                0,
                GdtAccess::ReadWrite
                    | GdtAccess::Executable
                    | GdtAccess::Segment
                    | GdtAccess::Present,
                GdtFlags::ProtMode | GdtFlags::Granularity,
            ),
            GdtDesc::new(
                0xFFFF,
                0,
                GdtAccess::Accessed
                    | GdtAccess::ReadWrite
                    | GdtAccess::Segment
                    | GdtAccess::Present,
                GdtFlags::ProtMode | GdtFlags::Granularity,
            ),
        ],
        farjmp_offset: 0,
        farjmp_segment: 8,
    };

    buffer[data_offset..][0..size_of::<InfoData>()].copy_from_slice(bytemuck::bytes_of(&info));

    dbg!(buffer);

    let lapic = LAPIC.get();

    lapic.send_ipi(
        IpiTarget::Specific(id),
        0,
        DeliveryMode::INIT,
        DestinationMode::Physical,
        DeliveryStatus::Idle,
        Level::Assert,
        TriggerMode::Edge,
    );
    clock::block_ns(1000_0000).unwrap();

    lapic.send_ipi(
        IpiTarget::Specific(id),
        (mem.value() >> 12) as u8,
        DeliveryMode::StartUp,
        DestinationMode::Physical,
        DeliveryStatus::Idle,
        Level::Assert,
        TriggerMode::Edge,
    );
    clock::block_ns(1000_0000).unwrap();
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
                    dbg!(lapic);

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
