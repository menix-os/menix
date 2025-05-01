#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::{
    arch,
    generic::{
        clock,
        memory::{
            self,
            virt::{KERNEL_PAGE_TABLE, VmFlags, VmLevel},
        },
        util,
    },
};
use alloc::{alloc::GlobalAlloc, boxed::Box};
use core::{
    alloc::Layout,
    ffi::{CStr, c_void},
    ptr::null_mut,
};
use spin::Mutex;

pub use uacpi::*;

#[unsafe(no_mangle)]
unsafe extern "C" fn uacpi_kernel_get_rsdp(out_rsdp_address: *mut uacpi_phys_addr) -> uacpi_status {
    match super::RSDP_ADDRESS.get() {
        Some(x) => unsafe {
            *out_rsdp_address = (*x).into();
            return UACPI_STATUS_OK;
        },
        None => return UACPI_STATUS_INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_map(addr: uacpi_phys_addr, len: uacpi_size) -> *mut c_void {
    let aligned_addr = util::align_down(addr as usize, arch::virt::get_page_size(VmLevel::L1));
    let difference = (addr as usize - aligned_addr);
    let aligned_len = util::align_up(len + difference, arch::virt::get_page_size(VmLevel::L1));
    return unsafe {
        KERNEL_PAGE_TABLE
            .write()
            .map_memory(
                aligned_addr.into(),
                VmFlags::Read | VmFlags::Write,
                VmLevel::L1,
                aligned_len,
            )
            .unwrap()
            .byte_add(difference)
    } as *mut c_void;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_unmap(addr: *mut c_void, len: uacpi_size) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_log(arg1: uacpi_log_level, arg2: *const uacpi_char) {
    let msg = unsafe { CStr::from_ptr(arg2) }.to_str().unwrap();
    match arg1 {
        UACPI_LOG_WARN => print_inner!("warn: acpi: ", "", "{}", msg),
        UACPI_LOG_DEBUG => print_inner!("debug: acpi: ", "", "{}", msg),
        UACPI_LOG_TRACE => print_inner!("trace: acpi: ", "", "{}", msg),
        UACPI_LOG_ERROR => print_inner!("error: acpi: ", "", "{}", msg),
        _ => print_inner!("acpi: ", "", "{}", msg),
    }
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_device_open(
    address: uacpi_pci_address,
    out_handle: *mut uacpi_handle,
) -> uacpi_status {
    // TODO
    return UACPI_STATUS_UNIMPLEMENTED;
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
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_unmap(handle: uacpi_handle) {}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_alloc(size: uacpi_size) -> *mut c_void {
    return unsafe {
        memory::slab::ALLOCATOR.alloc(Layout::from_size_align(size, align_of::<usize>()).unwrap())
            as *mut c_void
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free(mem: *mut c_void, size: uacpi_size) {
    // Frees of NULL are a no-op in uACPI.
    if mem == null_mut() {
        return;
    }

    unsafe {
        memory::slab::ALLOCATOR.dealloc(
            mem as *mut u8,
            Layout::from_size_align(size, align_of::<usize>()).unwrap(),
        )
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_get_nanoseconds_since_boot() -> uacpi_u64 {
    return clock::get_elapsed() as uacpi_u64;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_stall(usec: uacpi_u8) {
    clock::wait_ns(usec as usize);
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_sleep(msec: uacpi_u64) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_mutex() -> uacpi_handle {
    // TODO
    let mut b = Box::new(0);
    return Box::into_raw(b) as uacpi_handle;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_mutex(arg1: uacpi_handle) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_event() -> uacpi_handle {
    // TODO
    let mut b = Box::new(0);
    return Box::into_raw(b) as uacpi_handle;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_event(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_get_thread_id() -> uacpi_thread_id {
    // TODO
    return 0 as uacpi_thread_id;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_acquire_mutex(arg1: uacpi_handle, arg2: uacpi_u16) -> uacpi_status {
    // TODO
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_release_mutex(arg1: uacpi_handle) {
    // TODO
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
    // TODO
    return UACPI_STATUS_UNIMPLEMENTED;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_uninstall_interrupt_handler(
    arg1: uacpi_interrupt_handler,
    irq_handle: uacpi_handle,
) -> uacpi_status {
    // TODO
    return UACPI_STATUS_UNIMPLEMENTED;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_spinlock() -> uacpi_handle {
    let mut b = Box::new(Mutex::<usize>::new(0));
    return Box::into_raw(b) as uacpi_handle;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_spinlock(arg1: uacpi_handle) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_lock_spinlock(arg1: uacpi_handle) -> uacpi_cpu_flags {
    // TODO
    return 0;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_unlock_spinlock(arg1: uacpi_handle, arg2: uacpi_cpu_flags) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_schedule_work(
    arg1: uacpi_work_type,
    arg2: uacpi_work_handler,
    ctx: uacpi_handle,
) -> uacpi_status {
    // TODO
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_wait_for_work_completion() -> uacpi_status {
    // TODO
    return UACPI_STATUS_OK;
}
