// Advanced Configuration and Power Interface
// Wrapper for uACPI

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::{
    arch::{PhysAddr, firmware},
    generic::{
        self,
        clock::{self, ClockSource},
        memory::{
            self,
            virt::{KERNEL_PAGE_TABLE, VmFlags},
        },
    },
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    alloc::{Allocator, GlobalAlloc, Layout},
    ffi::{CStr, c_void},
    ptr::{NonNull, null_mut},
};
use spin::Once;
use uacpi::{
    self, uacpi_bool, uacpi_char, uacpi_cpu_flags, uacpi_firmware_request, uacpi_handle,
    uacpi_interrupt_handler, uacpi_io_addr, uacpi_log_level, uacpi_log_level_UACPI_LOG_DEBUG,
    uacpi_log_level_UACPI_LOG_ERROR, uacpi_log_level_UACPI_LOG_WARN, uacpi_pci_address,
    uacpi_phys_addr, uacpi_size, uacpi_status, uacpi_status_UACPI_STATUS_INTERNAL_ERROR,
    uacpi_status_UACPI_STATUS_OK, uacpi_thread_id, uacpi_u8, uacpi_u16, uacpi_u32, uacpi_u64,
    uacpi_work_handler, uacpi_work_type,
};

static RSDP_ADDRESS: Once<PhysAddr> = Once::new();

pub fn init(rsdp: PhysAddr) {
    RSDP_ADDRESS.call_once(|| return rsdp);

    // Get an early table window so we can initialize e.g. HPET and MADT.
    let mut early_mem: Vec<u8> = Vec::with_capacity(4096);
    unsafe { uacpi::uacpi_setup_early_table_access(early_mem.as_mut_ptr() as *mut c_void, 4096) };

    #[cfg(target_arch = "x86_64")]
    clock::switch(Box::new(firmware::Hpet::default()));

    print!("acpi: Initializing...\n");
    unsafe { uacpi::uacpi_initialize(0) };

    print!("acpi: Booting CPUs using MADT\n");
    // Setup the boot CPU.
    generic::percpu::setup_cpu();
    // TODO: Evaluate MADT and initialize all remaining CPUs.

    unsafe { uacpi::uacpi_namespace_load() };
    unsafe { uacpi::uacpi_namespace_initialize() };
}

#[unsafe(no_mangle)]
unsafe extern "C" fn uacpi_kernel_get_rsdp(out_rsdp_address: *mut uacpi_phys_addr) -> uacpi_status {
    match RSDP_ADDRESS.get() {
        Some(x) => unsafe {
            *out_rsdp_address = *x as uacpi_phys_addr;
            return uacpi_status_UACPI_STATUS_OK;
        },
        None => return uacpi_status_UACPI_STATUS_INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_map(addr: uacpi_phys_addr, len: uacpi_size) -> *mut c_void {
    return KERNEL_PAGE_TABLE.write().map_memory(
        addr as PhysAddr,
        VmFlags::Read | VmFlags::Write,
        0,
        len,
    ) as *mut c_void;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_unmap(addr: *mut c_void, len: uacpi_size) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_log(arg1: uacpi_log_level, arg2: *const uacpi_char) {
    let msg = unsafe { CStr::from_ptr(arg2) }.to_str().unwrap();
    match arg1 {
        uacpi_log_level_UACPI_LOG_WARN => warn!("acpi: {}", msg),
        uacpi_log_level_UACPI_LOG_ERROR => error!("acpi: {}", msg),
        _ => print!("acpi: {}", msg),
    }
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_device_open(
    address: uacpi_pci_address,
    out_handle: *mut uacpi_handle,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_device_close(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_read8(
    device: uacpi_handle,
    offset: uacpi_size,
    value: *mut uacpi_u8,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_read16(
    device: uacpi_handle,
    offset: uacpi_size,
    value: *mut uacpi_u16,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_read32(
    device: uacpi_handle,
    offset: uacpi_size,
    value: *mut uacpi_u32,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_write8(
    device: uacpi_handle,
    offset: uacpi_size,
    value: uacpi_u8,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_write16(
    device: uacpi_handle,
    offset: uacpi_size,
    value: uacpi_u16,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_write32(
    device: uacpi_handle,
    offset: uacpi_size,
    value: uacpi_u32,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_map(
    base: uacpi_io_addr,
    len: uacpi_size,
    out_handle: *mut uacpi_handle,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_unmap(handle: uacpi_handle) {
    todo!()
}
#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read8(
    arg1: uacpi_handle,
    offset: uacpi_size,
    out_value: *mut uacpi_u8,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read16(
    arg1: uacpi_handle,
    offset: uacpi_size,
    out_value: *mut uacpi_u16,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read32(
    arg1: uacpi_handle,
    offset: uacpi_size,
    out_value: *mut uacpi_u32,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write8(
    arg1: uacpi_handle,
    offset: uacpi_size,
    in_value: uacpi_u8,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write16(
    arg1: uacpi_handle,
    offset: uacpi_size,
    in_value: uacpi_u16,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write32(
    arg1: uacpi_handle,
    offset: uacpi_size,
    in_value: uacpi_u32,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_alloc(size: uacpi_size) -> *mut c_void {
    return unsafe {
        memory::ALLOCATOR.alloc(Layout::from_size_align(size, 16).unwrap()) as *mut c_void
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free(mem: *mut c_void, size: uacpi_size) {
    // Frees of NULL are a no-op in uACPI.
    if mem == null_mut() {
        return;
    }

    unsafe {
        memory::ALLOCATOR.deallocate(
            NonNull::new(mem as *mut u8).unwrap(),
            Layout::from_size_align(size, 16).unwrap(),
        )
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_get_nanoseconds_since_boot() -> uacpi_u64 {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_stall(usec: uacpi_u8) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_sleep(msec: uacpi_u64) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_mutex() -> uacpi_handle {
    // TODO
    return null_mut();
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_mutex(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_event() -> uacpi_handle {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_event(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_get_thread_id() -> uacpi_thread_id {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_acquire_mutex(arg1: uacpi_handle, arg2: uacpi_u16) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_release_mutex(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_wait_for_event(arg1: uacpi_handle, arg2: uacpi_u16) -> uacpi_bool {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_signal_event(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_reset_event(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_handle_firmware_request(
    arg1: *mut uacpi_firmware_request,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_install_interrupt_handler(
    irq: uacpi_u32,
    arg1: uacpi_interrupt_handler,
    ctx: uacpi_handle,
    out_irq_handle: *mut uacpi_handle,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_uninstall_interrupt_handler(
    arg1: uacpi_interrupt_handler,
    irq_handle: uacpi_handle,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_spinlock() -> uacpi_handle {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_spinlock(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_lock_spinlock(arg1: uacpi_handle) -> uacpi_cpu_flags {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_unlock_spinlock(arg1: uacpi_handle, arg2: uacpi_cpu_flags) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_schedule_work(
    arg1: uacpi_work_type,
    arg2: uacpi_work_handler,
    ctx: uacpi_handle,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_wait_for_work_completion() -> uacpi_status {
    todo!()
}
