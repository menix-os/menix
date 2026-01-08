use super::super::asm;
use crate::{
    arch::x86_64::system::apic::IoApic,
    memory::PhysAddr,
    system::pci::{Access, Address},
};
use alloc::boxed::Box;
use uacpi_sys::{
    UACPI_STATUS_OK, uacpi_handle, uacpi_io_addr, uacpi_size, uacpi_status, uacpi_table,
    uacpi_table_find_by_signature, uacpi_table_unref, uacpi_u8, uacpi_u16, uacpi_u32,
};

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read8(
    arg1: uacpi_handle,
    offset: uacpi_size,
    out_value: *mut uacpi_u8,
) -> uacpi_status {
    unsafe {
        (*out_value) = asm::read8((arg1 as usize + offset) as u16);
    }
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read16(
    arg1: uacpi_handle,
    offset: uacpi_size,
    out_value: *mut uacpi_u16,
) -> uacpi_status {
    unsafe {
        (*out_value) = asm::read16((arg1 as usize + offset) as u16);
    }
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read32(
    arg1: uacpi_handle,
    offset: uacpi_size,
    out_value: *mut uacpi_u32,
) -> uacpi_status {
    unsafe {
        (*out_value) = asm::read32((arg1 as usize + offset) as u16);
    }
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write8(
    arg1: uacpi_handle,
    offset: uacpi_size,
    in_value: uacpi_u8,
) -> uacpi_status {
    unsafe {
        asm::write8((arg1 as usize + offset) as u16, in_value);
    }
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write16(
    arg1: uacpi_handle,
    offset: uacpi_size,
    in_value: uacpi_u16,
) -> uacpi_status {
    unsafe {
        asm::write16((arg1 as usize + offset) as u16, in_value);
    }
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write32(
    arg1: uacpi_handle,
    offset: uacpi_size,
    in_value: uacpi_u32,
) -> uacpi_status {
    unsafe {
        asm::write32((arg1 as usize + offset) as u16, in_value);
    }
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_map(
    base: uacpi_io_addr,
    _len: uacpi_size,
    out_handle: *mut uacpi_handle,
) -> uacpi_status {
    unsafe {
        out_handle.write(base as uacpi_handle);
    }
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_unmap(handle: uacpi_handle) {
    _ = handle;
}

struct PortIoAccess;

impl PortIoAccess {
    fn select(&self, addr: Address, offset: u32) {
        unsafe {
            asm::write32(
                0xCF8,
                (addr.bus as u32) << 16
                    | (addr.slot as u32) << 11
                    | (addr.function as u32) << 8
                    | (offset & 0xFC)
                    | 1 << 31,
            );
        }
    }
}

impl Access for PortIoAccess {
    fn segment(&self) -> u16 {
        0
    }

    fn start_bus(&self) -> u8 {
        0
    }

    fn end_bus(&self) -> u8 {
        255
    }

    fn read32(&self, addr: Address, offset: u32) -> u32 {
        self.select(addr, offset);
        unsafe { asm::read32(0xCFC) }
    }

    fn write32(&self, addr: Address, offset: u32, value: u32) {
        self.select(addr, offset);
        unsafe { asm::write32(0xCFC, value) }
    }
}

#[initgraph::task(
    name = "arch.x86_64.acpi",
    entails = [crate::system::acpi::INIT_STAGE],
    depends = [crate::memory::MEMORY_STAGE],
)]
fn ACPI_STAGE() {
    unsafe { crate::system::pci::ACCESS.init(vec![Box::new(PortIoAccess)]) };
}

#[initgraph::task(
    name = "arch.x86_64.find-ioapics",
    depends = [crate::system::acpi::INIT_STAGE],
)]
fn IOAPIC_STAGE() {
    unsafe {
        let mut table = uacpi_table::default();
        let status = uacpi_table_find_by_signature(c"APIC".as_ptr(), &raw mut table);
        if status != UACPI_STATUS_OK {
            return;
        }

        let madt_ptr = table.__bindgen_anon_1.ptr as *const uacpi_sys::acpi_madt;
        let madt = madt_ptr.read_unaligned();

        let mut offset = size_of::<uacpi_sys::acpi_madt>();

        while offset < madt.hdr.length as usize {
            let entry_ptr = madt_ptr.byte_add(offset) as *const uacpi_sys::acpi_entry_hdr;
            let entry = entry_ptr.read_unaligned();

            match entry.type_ as _ {
                uacpi_sys::ACPI_MADT_ENTRY_TYPE_IOAPIC => {
                    let entry = (entry_ptr as *const uacpi_sys::acpi_madt_ioapic).read_unaligned();
                    IoApic::setup(
                        entry.id,
                        entry.gsi_base,
                        PhysAddr::from(entry.address as usize),
                    );
                }
                _ => (),
            }

            offset += entry.length as usize;
        }

        uacpi_table_unref(&mut table);
    }
}
